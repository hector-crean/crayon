pub mod weighted_random_heuristic;
use block3d_core::block::Block3DLike;
use petgraph::graph::NodeIndex;

use crate::wfc::solver::state::NodeState;



pub trait Heuristic<T: Block3DLike> {
    /// Select a node to collapse based on entropy or other criteria
    fn select_node_to_collapse(&self, solver_states: &[Vec<NodeState<T>>], collapsed: &std::collections::HashSet<NodeIndex>) -> Option<NodeIndex>;

    /// Given multiple valid states for a node, select one (weighted random, etc.)
    fn select_state_for_node(&self, node: NodeIndex, valid_states: &[NodeState<T>]) -> Option<NodeState<T>>;
}


