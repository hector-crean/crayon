use std::marker::PhantomData;

use block3d_core::block::Block3DLike;
use petgraph::graph::NodeIndex;

use super::solver::state::NodeState;

// Define a trait for observers in the WFC algorithm
pub trait WFCObserverLike<T: Block3DLike> {
    // Method to be called when a node collapses
    fn on_collapse(&self, node: NodeIndex, state: &NodeState<T>);
    // Method to be called when propagation occurs
    fn on_propagate(&self, affected: &[NodeIndex]);
}

// Define a struct implementing the observer pattern for WFC
pub struct WFCObserver<T: Block3DLike> { 
    phantom: PhantomData<T>,
}

// Implement the WFCObserverLike trait for WFCObserver
impl<T: Block3DLike> WFCObserverLike<T> for WFCObserver<T> {
    // Placeholder for collapse event handling
    fn on_collapse(&self, node: NodeIndex, state: &NodeState<T>) {
        todo!()
    }
    // Placeholder for propagation event handling
    fn on_propagate(&self, affected: &[NodeIndex]) {
        todo!()
    }
}

// Implement the Default trait for WFCObserver
impl<T: Block3DLike> Default for WFCObserver<T> {
    // Provide a default constructor using the new method
    fn default() -> Self {
        Self::new()
    }
}

// Implement methods for WFCObserver
impl<T: Block3DLike> WFCObserver<T> {
    // Constructor for WFCObserver
    pub fn new() -> Self {
        Self { phantom: PhantomData }
    }
}
