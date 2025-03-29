pub mod gravity_invariant;
use block3d_core::block::Block3DLike;
use petgraph::graph::NodeIndex;
use crate::wfc::solver::WFCSolver;

use super::{solver::state::NodeState};


// Trait for implementing WFC constraints/invariants
pub trait WFCInvariant<T: Block3DLike> {
    fn check(&self, node: NodeIndex, state: &NodeState<T>, solver: &WFCSolver<T>) -> bool;
    fn propagate(&self, node: NodeIndex, solver: &WFCSolver<T>) -> Vec<NodeIndex>;
}
