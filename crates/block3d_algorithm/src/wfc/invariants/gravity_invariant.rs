use block3d_core::block::Block3DLike;
use petgraph::graph::NodeIndex;

use crate::wfc::invariants::WFCInvariant;
use crate::wfc::solver::{WFCSolver, state::NodeState};

pub struct GravityInvariant;

impl<T: Block3DLike> WFCInvariant<T> for GravityInvariant {
    fn check(&self, node: NodeIndex, state: &NodeState<T>, solver: &WFCSolver<T>) -> bool {
        //bottom layer always supported
        //check if below node has at least one collapsed state
       todo!();
    }

    fn propagate(&self, node: NodeIndex, solver: &WFCSolver<T>) -> Vec<NodeIndex> {
        // Gravity might affect nodes above and below
        todo!();
     
    }
}