use std::f32::INFINITY;

use bevy::prelude::*;
use bitflags::bitflags;




bitflags! {
    // Attributes can be applied to flags types
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct TransformMode: u32 {
        const Translate = 1;
        const Rotate = 1 << 1;
        const Scale = 1 << 2;
    }
}



#[derive(Resource)]
pub struct TransformControllerSettings {
    pub enabled: bool,
    pub grid_snapping: Option<Vec3>,
    pub translation_constraints: Option<BVec3>, 
    pub rotation_constraints: Option<Vec3>,    // Constrain rotations to certain axes
    pub scale_constraints: Option<Vec3>,       // Constrain scaling behavior
}
impl Default for TransformControllerSettings {
    fn default() -> Self {
        Self { enabled: true, grid_snapping: Some(Vec3 { x: 1.0, y: 1.0, z: 1.0 }), translation_constraints: Some(BVec3 { x: true, y: true, z: true}), rotation_constraints: None, scale_constraints: None }
    }
}



#[derive(Component)]
pub struct TransformBounds {
    pub min: Vec3,
    pub max: Vec3,
}

impl Default for TransformBounds {
    fn default() -> Self {
        Self { min: Vec3::new(-INFINITY, -INFINITY, -INFINITY), max: Vec3::new(INFINITY, INFINITY, INFINITY) }
    }
}

impl TransformBounds {
    pub fn contains(&self, point: Vec3) -> bool {
        point.x >= self.min.x && point.x <= self.max.x &&
        point.y >= self.min.y && point.y <= self.max.y &&
        point.z >= self.min.z && point.z <= self.max.z
    }
}



#[derive(Component)]
#[require(Transform, TransformBounds)]
pub struct TransformController {
    pub enabled: bool,
    pub drag_start_pointer_position: Option<Vec3>,
    pub drag_start_entity_position: Option<Vec3>,
    pub mode: TransformMode,
}
impl Default for TransformController {
    fn default() -> Self {
        Self {
            enabled: true,
            drag_start_pointer_position: None,
            drag_start_entity_position: None,
            mode: TransformMode::Translate,
        }
    }
}



