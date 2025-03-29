use crate::CameraMode;
use bevy::prelude::*;

#[derive(Debug, PartialEq, Eq)]
pub enum CameraModes {
    //     Orbiting: The camera rotates around a target object or point of interest. The camera's movement is constrained to a certain distance from the target and a fixed angle of inclination.
    Orbiting {
        target: Option<Entity>,
        // distance: i32,
        // elevation: i32,
        // azimuth: i32,
    },
    // Following: The camera follows a target object or character as it moves through the scene. The camera's movement is constrained to a certain distance and angle from the target.
    Following {
        target: Entity,
        // offset: Vec3,
    },
    // First-person: The camera is positioned at the player's eye level and follows the player's movements. The camera's movement is generally limited to the player's movements and the player's field of view.
    FirstPerson {
        // yaw: f32,
        // pitch: f32,
    },
    // Third-person: The camera is positioned behind the player and follows the player's movements. The camera's movement is generally limited to a certain distance and angle from the player.
    ThirdPerson {},
    // Top-down: The camera is positioned directly above the scene and provides a bird's-eye view of the action. The camera's movement is generally limited to panning and zooming.
    TopDown,
    // Cinematic: The camera is used to create a cinematic effect, such as a cutscene or dramatic reveal. The camera's movement is generally scripted and may include special effects such as depth of field or motion blur.
    Cinematic,
}

impl Default for CameraModes {
    fn default() -> Self {
        Self::Orbiting { target: None }
    }
}

#[derive(Debug, PartialEq, Eq, Resource, Default)]
pub struct CameraModeImpl {
    locked: bool,
    mode: CameraModes,
}

impl CameraMode for CameraModeImpl {
    fn is_locked(&self) -> bool {
        self.locked
    }

    fn lock(&mut self) {
        self.locked = true;
    }

    fn unlock(&mut self) {
        self.locked = false
    }
}
