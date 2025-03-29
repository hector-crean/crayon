use crate::face::Face;
use super::{Block3DLike, BlockKind};
use strum::IntoEnumIterator;
use serde::{Serialize, Deserialize};


// We will probably store in a database a `block` instance, which will have its size, kind, and faces. The orientation and position is the only
// thing that will be dynamic?

#[derive(Serialize, Deserialize, Debug, Clone, Default, Hash, Eq, PartialEq)]
pub struct LegoBlock {
    pub size: (u32, u32, u32),
    pub block_kind: BlockKind,
    pub faces: Vec<Face>,
}

impl Block3DLike for LegoBlock {

   
    fn size(&self) -> (u32, u32, u32) {
        self.size
    }
    fn block_kind(&self) -> BlockKind {
        self.block_kind
    }
  
    fn faces(&self) -> impl Iterator<Item = Face> {
        self.faces.iter().cloned()
    }
}

impl LegoBlock {
    pub fn new(size: (u32, u32, u32), block_kind: BlockKind, faces: Vec<Face>) -> Self {
        Self { size, block_kind, faces }
    }
}
