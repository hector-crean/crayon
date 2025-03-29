use block3d_core::{block::Block3DLike, Orientation};
use strum::IntoEnumIterator;


#[derive(Clone, Debug)]
pub struct NodeState<T> {
    pub orientation: Orientation,
    pub block: T,
}

impl<T: Block3DLike> NodeState<T> {
    pub fn new(block: T, orientation: Orientation) -> Self {
        Self {
            block,
            orientation,
        }
    }
    fn orientation_iter() -> impl Iterator<Item = Orientation> {
        Orientation::iter()
    }
}


#[derive(Debug, Clone)]
pub struct EdgeState {
    
}

