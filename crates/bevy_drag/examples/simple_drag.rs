//! https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
//!
//!
//!

use bevy::prelude::*;
use bevy_camera::{
    pan_orbit_camera::{OrbitCameraController, OrbitCameraControllerPlugin},
    CameraMode,
};
use bevy_drag::drag::{DragTransformPlugin, Draggable};



fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Add a plane to represent the ground
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::new(Vec3::new(0., 1., 0.), Vec2::new(10., 10.)))),
        MeshMaterial3d(materials.add(StandardMaterial::default())),
        Transform::default(),
    ));

    // Add a grid system with distinct colors
    let grid_size = 1.0;
    let grid_count = 10;
    for x in 0..grid_count {
        for z in 0..grid_count {
            let color = Color::srgb(
                x as f32 / grid_count as f32,
                z as f32 / grid_count as f32,
                0.5,
            );
            commands.spawn((
                Mesh3d(meshes.add(Cuboid::default())),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: color,
                    ..default()
                })),
                Transform::from_xyz(x as f32 * grid_size, 0.5, z as f32 * grid_size),
                Draggable::default(),
            ));
        }
    }

    // Improved lighting
    commands.spawn((
        PointLight {
            intensity: 3000.0, // Increased intensity for better lighting
            shadows_enabled: true,
            range: 20.0, // Adjust range for better coverage
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0., 0., -2.).looking_at(Vec3::ZERO, Vec3::Y),
        OrbitCameraController::default(),
    ));
}



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

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resizable: true,
                    fit_canvas_to_parent: true,
                    ..default()
                }),
                ..default()
            }),
            MeshPickingPlugin,
            DragTransformPlugin::<CameraModeImpl>::default(),
            OrbitCameraControllerPlugin::<CameraModeImpl>::default(),
        ))
        .add_systems(Startup, (setup_scene, setup_camera))
        .run();
}
