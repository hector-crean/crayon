pub mod compatibility;
pub mod error;
pub mod state;
pub mod spatial_grid;

use block3d_core::block::Block3DLike;
use block3d_core::Orientation;
use petgraph::graph::NodeIndex;
use spatial_grid::SpatialGrid;
use std::collections::{HashMap, HashSet, BTreeMap};

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
    // Spatial grid for collision detection
    spatial_grid: SpatialGrid,
    // Connection graph to track block relationships
    connections: HashMap<NodeIndex, HashMap<String, (NodeIndex, String)>>,
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
            spatial_grid: SpatialGrid::new(1.0), // 1.0 unit cell size
            connections: HashMap::new(),
        }
    }

    pub fn solve(&mut self) -> Result<(), WFCError> {
        // Initialize node states with potential blocks
        self.initialize_states()?;
        
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
            // Remove from spatial grid and connections
            self.spatial_grid.remove_node(prev_node);
            self.connections.remove(&prev_node);
            
            // Restore previous state
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

        // Get valid states based on invariants and connection constraints
        let mut valid_states: Vec<NodeState<T>> = self.block_set
            .iter()
            .flat_map(|block| {
                Orientation::iter().map(move |orientation| {
                    // Use with_position to preserve the node's position and initialize connections
                    NodeState::with_position(block.clone(), orientation, current_position)
                })
            })
            .filter(|state| {
                // Apply invariants
                self.invariants.iter().all(|inv| inv.check(node, state, self)) &&
                // Check for collisions
                !self.would_collide(state) &&
                // Check if it can connect to at least one existing block (unless it's the first block)
                (self.collapsed.is_empty() || self.can_connect_to_existing(state))
            })
            .collect();

        // If this is the first node, any valid state is acceptable
        if self.collapsed.is_empty() {
            valid_states = self.block_set
                .iter()
                .flat_map(|block| {
                    Orientation::iter().map(move |orientation| {
                        NodeState::with_position(block.clone(), orientation, current_position)
                    })
                })
                .filter(|state| {
                    self.invariants.iter().all(|inv| inv.check(node, state, self))
                })
                .collect();
        }

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

        // Add to spatial grid
        let position = (
            selected_state.position.0 as f32,
            selected_state.position.1 as f32,
            selected_state.position.2 as f32,
        );
        let size = (
            selected_state.block.size().0 as f32,
            selected_state.block.size().1 as f32,
            selected_state.block.size().2 as f32,
        );
        self.spatial_grid.add_node(node, position, size);

        // Update connections
        self.establish_connections(node, &selected_state)?;

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
    
    // Check if a state would collide with existing blocks
    fn would_collide(&self, state: &NodeState<T>) -> bool {
        let position = state.world_position();
        let size = (
            state.block.size().0 as f32,
            state.block.size().1 as f32,
            state.block.size().2 as f32,
        );
        
        // Get potential collision candidates
        let candidates = self.spatial_grid.potential_collisions(position, size);
        
        // Check each candidate for collision
        for candidate_idx in candidates {
            if let Some(candidate_state) = self.graph.node_weight(candidate_idx) {
                if state.collides_with(candidate_state) {
                    return true;
                }
            }
        }
        
        false
    }
    
    // Check if a state can connect to at least one existing block
    fn can_connect_to_existing(&self, state: &NodeState<T>) -> bool {
        // Skip if no blocks exist yet
        if self.collapsed.is_empty() {
            return true;
        }
        
        // Check connection possibilities with neighboring blocks
        for collapsed_node in &self.collapsed {
            if let Some(collapsed_state) = self.graph.node_weight(*collapsed_node) {
                if state.can_connect_to(collapsed_state).is_some() {
                    return true;
                }
            }
        }
        
        false
    }
    
    // Establish connections between a node and existing blocks
    fn establish_connections(&mut self, node: NodeIndex, state: &NodeState<T>) -> Result<(), WFCError> {
        // Skip if no blocks exist yet
        if self.collapsed.is_empty() {
            return Ok(());
        }
        
        // Find compatible connections
        let mut possible_connections = Vec::new();
        
        for other_node in &self.collapsed {
            if *other_node == node {
                continue;
            }
            
            if let Some(other_state) = self.graph.node_weight(*other_node) {
                if let Some((self_conn_id, other_conn_id)) = state.can_connect_to(other_state) {
                    possible_connections.push((*other_node, self_conn_id, other_conn_id));
                }
            }
        }
        
        // If we found connections, establish them
        if !possible_connections.is_empty() {
            // For simplicity, just use the first compatible connection
            let (other_node, self_conn_id, other_conn_id) = &possible_connections[0];
            
            // Update connection maps
            self.connections.entry(node).or_insert_with(HashMap::new)
                .insert(self_conn_id.clone(), (*other_node, other_conn_id.clone()));
            
            self.connections.entry(*other_node).or_insert_with(HashMap::new)
                .insert(other_conn_id.clone(), (node, self_conn_id.clone()));
            
            // Update NodeState connection information
            if let Some(node_state) = self.graph.node_weight_mut(node) {
                if let Some(conn) = node_state.connections.get_mut(self_conn_id) {
                    conn.connected_to = Some((*other_node, other_conn_id.clone()));
                    node_state.is_connected = true;
                }
            }
            
            if let Some(other_state) = self.graph.node_weight_mut(*other_node) {
                if let Some(conn) = other_state.connections.get_mut(other_conn_id) {
                    conn.connected_to = Some((node, self_conn_id.clone()));
                    other_state.is_connected = true;
                }
            }
        }
        
        Ok(())
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
  
    
    /// Find a node at the given grid position
    pub fn find_node_at_position(&self, position: (usize, usize, usize)) -> Option<NodeIndex> {
        for node_idx in self.graph.node_indices() {
            if let Some(state) = self.graph.node_weight(node_idx) {
                if state.position == position {
                    return Some(node_idx);
                }
            }
        }
        None
    }

    pub fn collapse_node_at_position(&mut self, position: (usize, usize, usize)) -> Result<(), WFCError> {

        let node_index = self.find_node_at_position(position)
            .ok_or(WFCError::NodeNotFoundAtPosition(position))?;

          // Collapse the specified node
          match self.collapse_node(node_index) {
            Ok(_) => {},
            Err(e) => return Err(e), // Return error if initial collapse fails
        }
        
        Ok(())
    }
    pub fn set_node_at_position(&mut self, position: (usize, usize, usize), block: T) -> Result<(), WFCError> {
        let node_index = self.find_node_at_position(position)
            .ok_or(WFCError::NodeNotFoundAtPosition(position))?;

        let node_state = self.graph.node_weight_mut(node_index)
            .ok_or(WFCError::NodeNotFound(node_index))?;

        node_state.block = block;

        Ok(())
    }
}
