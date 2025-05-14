pub mod skylark_block;
pub mod lego_block;

use serde::{Deserialize, Serialize};
use strum::AsRefStr;
use strum::EnumProperty;
use crate::face::Face;
use strum::EnumIter;

#[derive(Clone, Copy, Serialize, Deserialize, Debug, Default, Hash, Eq, PartialEq, EnumIter, strum::Display, EnumProperty, AsRefStr)]
pub enum BlockKind {
    #[strum(props(
        icon = "icons/window_48px.png",
        color = "#87CEEB"  // Sky blue
    ))]
    Window,
    
    #[strum(props(
        icon = "icons/door_48px.png",
        color = "#8B4513"  // Saddle brown
    ))]
    Door,
    
    #[strum(props(
        icon = "icons/wall_48px.png",
        color = "#D3D3D3"  // Light gray
    ))]
    Wall,
    
    #[strum(props(
        icon = "icons/floor_48px.png",
        color = "#DEB887"  // Burlywood
    ))]
    Floor,
    
    #[strum(props(
        icon = "icons/ceiling_48px.png", 
        color = "#F5F5F5"  // White smoke
    ))]
    Ceiling,
    
    #[strum(props(
        icon = "icons/column_48px.png",
        color = "#A9A9A9"  // Dark gray
    ))]
    Column,
    
    #[strum(props(
        icon = "icons/stairs_48px.png",
        color = "#CD853F"  // Peru
    ))]
    Stairs,

    #[strum(props(
        icon = "icons/void_48px.png",
        color = "#8B0000"  // Dark red
    ))]
    #[default]
    Void,

}



// We store the block in a database:
// - size, faces, block kind
pub trait Block3DLike: Clone + Default {
    fn block_kind(&self) -> BlockKind;
    fn faces(&self) -> impl Iterator<Item = Face>;
    fn size(&self) -> (u32, u32, u32);
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash, Eq, PartialEq)]
pub enum Block3D {
    // Skylark(skylark_block::SkylarkBlock),
    Lego(lego_block::LegoBlock),
}
impl Default for Block3D {
    fn default() -> Self {
        Block3D::Lego(lego_block::LegoBlock::default())
    }
}

impl Block3DLike for Block3D {
   
   
    fn size(&self) -> (u32, u32, u32) {
        match self {
            Block3D::Lego(block) => block.size(),
        }
    }
    fn block_kind(&self) -> BlockKind {
        match self {
            Block3D::Lego(block) => block.block_kind(),
        }
    }
    
    fn faces(&self) -> impl Iterator<Item = Face> {
        match self {
            Block3D::Lego(block) => block.faces(),
        }
    }
   
}
