use block3d_core::{block::Block3DLike, Orientation};
use strum::IntoEnumIterator;


#[derive(Clone, Debug)]
pub struct NodeState<T> {
    pub orientation: Orientation,
    pub block: T,
    pub position: (usize, usize, usize),  // Store x, y, z coordinates
}

impl<T: Block3DLike> NodeState<T> {
    pub fn new(block: T, orientation: Orientation) -> Self {
        Self {
            block,
            orientation,
            position: (0, 0, 0),  // Default position
        }
    }
    
    pub fn with_position(block: T, orientation: Orientation, position: (usize, usize, usize)) -> Self {
        Self {
            block,
            orientation,
            position,
        }
    }
    
    fn orientation_iter() -> impl Iterator<Item = Orientation> {
        Orientation::iter()
    }
}


#[derive(Debug, Clone)]
pub struct EdgeState {
    
}

