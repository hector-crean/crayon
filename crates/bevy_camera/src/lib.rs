pub mod api;
pub mod mode;
pub mod pan_orbit_camera;

pub use api::{CameraController, CameraMode, CameraRig};
use bevy::prelude::Component;


#[derive(Component)]
pub struct MainCamera;