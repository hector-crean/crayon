pub mod compatibility;
pub mod error;
pub mod state;

use block3d_core::block::Block3DLike;
use block3d_core::Orientation;
use petgraph::graph::NodeIndex;
use std::collections::{HashMap, HashSet};

use error::WFCError;
use petgraph::visit::Dfs;

use state::NodeState;

use super::graph::WFCGraph;
use super::observer::WFCObserverLike;
use crate::wfc::heuristics::Heuristic;
use crate::wfc::invariants::WFCInvariant;
use crate::wfc::solver::compatibility::CompatibilityTable;
use fixedbitset::FixedBitSet;
use strum::IntoEnumIterator;


pub struct WFCSolver<T: Block3DLike> {
    pub graph: WFCGraph<T>,
    pub stack: HashMap<NodeIndex, NodeState<T>>,
    pub collapsed: HashSet<NodeIndex>,
    pub invariants: Vec<Box<dyn WFCInvariant<T>>>,
    pub compatibility: CompatibilityTable<T>,
    pub heuristic: Box<dyn Heuristic<T>>,
    observers: Vec<Box<dyn WFCObserverLike<T>>>,
    // A set of all blocks
    block_set: HashSet<T>,
}

impl<T: Block3DLike> WFCSolver<T> {
    pub fn new(
        graph: WFCGraph<T>,
        block_set: HashSet<T>,
        invariants: Vec<Box<dyn WFCInvariant<T>>>,
        heuristic: Box<dyn Heuristic<T>>,
        observers: Vec<Box<dyn WFCObserverLike<T>>>,
    ) -> Self {
        Self {
            graph,
            stack: HashMap::new(),
            collapsed: HashSet::new(),
            invariants,
            compatibility: CompatibilityTable::new(),
            heuristic,
            observers,
            block_set,
        }
    }

    pub fn solve(&mut self) -> Result<(), WFCError> {
        let node_id = self.graph.add_node(NodeState::new(T::default(), Orientation::default()));
        let mut dfs = Dfs::new(&self.graph.0, node_id);

        let mut stack = Vec::<(NodeIndex, NodeState<T>)>::new();

        while let Some(node) = dfs.next(&self.graph.0) {
            
            let node_state = self.graph.node_weight(node)
                .ok_or(WFCError::NodeNotFound(node))?
                .clone();
            
            stack.push((node, node_state));

            match self.collapse_node(node) {
                Ok(_) => {},
                Err(_) => {
                    match self.backtrack(&mut dfs, &mut stack) {
                        Ok(true) => {
                            // We now need to re-collapse in a different way
                            self.collapse_node(node)?;
                        },
                        _ => return Err(WFCError::NoSolution),
                    }
                }
            }
        }

        Ok(())
    }

    fn backtrack(&mut self, dfs: &mut Dfs<NodeIndex, FixedBitSet>, stack: &mut Vec<(NodeIndex, NodeState<T>)>) -> Result<bool, WFCError> {
        if let Some((prev_node, prev_state)) = stack.pop() {
            match self.graph.node_weight_mut(prev_node) {
                Some(node_state) => {
                    *node_state = prev_state;
                    dfs.stack.push(prev_node); // Revisit the previous node
                    Ok(true)
                }
                None => Err(WFCError::NoSolution),
            }
        } else {
            Err(WFCError::NoSolution)
        }
    }

    fn initialize_states(&mut self) -> Result<(), WFCError> {
        // If needed, run invariants or prune impossible states upfront
        Ok(())
    }

    fn collapse_node(&mut self, node: NodeIndex) -> Result<NodeState<T>, WFCError> {
        let node_state = self.graph.node_weight(node)
            .ok_or(WFCError::NodeNotFound(node))?;

        // Skip if already collapsed
        if self.collapsed.contains(&node) {
            return Ok(node_state.clone());
        }

        // Get the current position from the node
        let current_position = node_state.position;

        // Get valid states based on invariants
        let valid_states: Vec<NodeState<T>> = self.block_set
            .iter()
            .flat_map(|block| {
                Orientation::iter().map(move |orientation| {
                    // Use with_position to preserve the node's position
                    NodeState::with_position(block.clone(), orientation, current_position)
                })
            })
            .filter(|state| {
                self.invariants.iter().all(|inv| inv.check(node, state, self))
            })
            .collect();

        if valid_states.is_empty() {
            return Err(WFCError::NoValidStatesAfterInvariants(node));
        }

        // Use heuristic to select state if multiple options exist
        let selected_state = self.heuristic.select_state_for_node(node, &valid_states)
            .ok_or(WFCError::HeuristicFailure(node))?;

        // Update node state
        if let Some(node_weight) = self.graph.node_weight_mut(node) {
            *node_weight = selected_state.clone();
        }

        // Mark as collapsed
        self.collapsed.insert(node);

        // Notify observers
        self.notify_collapse(node, &selected_state);

        // Propagate constraints to neighbors
        let affected_nodes: Vec<NodeIndex> = self.invariants.iter()
            .flat_map(|inv| inv.propagate(node, self))
            .collect();

        if !affected_nodes.is_empty() {
            self.notify_propagate(&affected_nodes);
        }

        Ok(selected_state)
    }

    pub fn add_observer(&mut self, observer: Box<dyn WFCObserverLike<T>>) {
        self.observers.push(observer);
    }

    fn notify_collapse(&self, node: NodeIndex, state: &NodeState<T>) {
        for observer in &self.observers {
            observer.on_collapse(node, state);
        }
    }

    fn notify_propagate(&self, affected: &[NodeIndex]) {
        for observer in &self.observers {
            observer.on_propagate(affected);
        }
    }

    /// Collapses a specific node and propagates constraints to its neighbors.
    pub fn collapse_specific_node(&mut self, node_index: NodeIndex) -> Result<(), WFCError> {
        // Collapse the specified node
        match self.collapse_node(node_index) {
            Ok(_) => {},
            Err(e) => return Err(e), // Return error if initial collapse fails
        }
        
        // Propagate constraints to neighbors (this is already handled in collapse_node)
        // but we might want to do an additional broader check here if needed
        
        Ok(())
    }
}
