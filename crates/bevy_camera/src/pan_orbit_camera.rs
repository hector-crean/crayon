use crate::api::{CameraController, CameraMode};
use bevy::{
    input::mouse::{
        MouseScrollUnit::{Line, Pixel},
        MouseWheel,
    },
    picking::pointer::PointerInput,
    prelude::*,
    render::camera::Camera,
};
// use bevy_mod_picking::{
//     pointer::InputMove,
//     prelude::{Click, Down, Drag, DragEnd, DragStart, Pointer, Up},
// };

use bevy::picking::{
    pointer::PointerAction,
    prelude::{Click, Down, Drag, DragEnd, DragStart, Pointer, Up},
};

use std::{f32::consts::FRAC_PI_2, ops::RangeInclusive};

#[derive(Default)]
pub struct OrbitCameraControllerPlugin<T: CameraMode>(pub T);

impl<T: CameraMode + Send + Sync + 'static> Plugin for OrbitCameraControllerPlugin<T> {
    fn build(&self, app: &mut App) {
        app.init_resource::<T>()
            .add_event::<OrbitCameraControllerEvents>()
            .add_systems(Startup, Self::update_camera_transform_system)
            .add_systems(
                PostUpdate,
                (Self::emit_motion_events).run_if(
                    not(on_event::<Pointer<DragStart>>)
                        .and(not(on_event::<Pointer<Drag>>))
                        .and(not(on_event::<Pointer<DragEnd>>))
                        .and(not(on_event::<Pointer<Down>>))
                        .and(not(on_event::<Pointer<Up>>))
                        .and(not(on_event::<Pointer<Click>>))
                        .and(run_criteria::<T>),
                ),
            )
            .add_systems(Update, Self::emit_zoom_events)
            .add_systems(
                Last,
                (
                    Self::consume_pan_and_orbit_events,
                    Self::consume_zoom_events,
                    Self::update_camera_transform_system,
                )
                    .chain()
                    .run_if(on_event::<OrbitCameraControllerEvents>),
            );
    }
}

const LINE_TO_PIXEL_RATIO: f32 = 0.1;

fn run_criteria<T: CameraMode>(mode: Res<T>) -> bool {
    !(*mode).is_locked()
}

#[derive(Event)]
pub enum OrbitCameraControllerEvents {
    Orbit(Vec2),
    Pan(Vec2),
    Zoom(f32),
}

#[derive(Component)]
pub struct OrbitCameraController {
    // the horizontal rotation angle of the camera around the target center
    pub x: f32,
    //vertical rotation angle of the camera
    pub y: f32,
    //rhe range of allowable values for the y field
    pub pitch_range: RangeInclusive<f32>,
    //the distance of the camera from the target center
    pub distance: f32,
    //the center of the camera's orbit
    pub center: Vec3,
    //the sensitivity of the camera's rotation
    pub rotate_sensitivity: f32,
    //the sensitivity of the camera's panning
    pub pan_sensitivity: f32,
    //the sensitivity of the camera's zooming
    pub zoom_sensitivity: f32,
    pub rotate_button: MouseButton,
    pub pan_button: MouseButton,
    pub enabled: bool,
}

impl Default for OrbitCameraController {
    fn default() -> Self {
        OrbitCameraController {
            x: FRAC_PI_2 / 2.0,
            y: FRAC_PI_2 / 2.0,
            pitch_range: 0.01..= FRAC_PI_2,
            distance: 20.0,
            center: Vec3::ZERO,
            rotate_sensitivity: 0.4,
            pan_sensitivity: 0.4,
            zoom_sensitivity: 0.4,
            rotate_button: MouseButton::Left,
            pan_button: MouseButton::Right,
            enabled: true,
        }
    }
}

impl OrbitCameraController {
    pub fn new(dist: f32, center: Vec3, initial_transform: Transform) -> OrbitCameraController {
        let (x, y) = {
            let forward = initial_transform.rotation * Vec3::Z;
            let x = forward.x.atan2(forward.z);
            let y = forward.y.asin();
            (x, y)
        };

        OrbitCameraController {
            distance: dist,
            center,
            x,
            y,
            ..Self::default()
        }
    }

    pub fn generate_camera_transform(&self) -> Transform {
        let rot = Quat::from_axis_angle(Vec3::Y, self.x)
            * Quat::from_axis_angle(-Vec3::X, self.y);
        let translation = (rot * Vec3::Y) * self.distance + self.center;
        Transform::from_translation(translation).looking_at(self.center, Vec3::Y)
    }
}

impl CameraController for OrbitCameraController {
    fn update_camera_transform_system(
        mut query: Query<
            (&Self, &mut Transform),
            (Or<(Changed<Self>, Added<Self>)>, With<Camera3d>),
        >,
    ) {
        for (controller, mut transform) in query.iter_mut() {
            if controller.enabled {
                let rot = Quat::from_axis_angle(Vec3::Y, controller.x)
                    * Quat::from_axis_angle(-Vec3::X, controller.y);
                transform.translation = (rot * Vec3::Y) * controller.distance + controller.center;
                transform.look_at(controller.center, Vec3::Y);
            }
        }
    }
}

impl<T: CameraMode> OrbitCameraControllerPlugin<T> {
    pub fn update_camera_transform_system(
        query: Query<
            (&OrbitCameraController, &mut Transform),
            (
                Or<(Changed<OrbitCameraController>, Added<OrbitCameraController>)>,
                With<Camera3d>,
            ),
        >,
    ) {
        OrbitCameraController::update_camera_transform_system(query);
    }

    pub fn emit_motion_events(
        mut events: EventWriter<OrbitCameraControllerEvents>,
        mut pointer_motion_events: EventReader<PointerInput>,
        pointer_button_input: Res<ButtonInput<MouseButton>>,
        mut query: Query<&OrbitCameraController>,
    ) {
        let mut total_delta = Vec2::ZERO;

        for event in pointer_motion_events.read() {
            if let PointerAction::Moved { delta } = event.action {
                total_delta += delta;
            }
        }
        for camera in query.iter_mut() {
            if camera.enabled {
                if pointer_button_input.pressed(camera.rotate_button) {
                    events.send(OrbitCameraControllerEvents::Orbit(total_delta));
                }

                if pointer_button_input.pressed(camera.pan_button) {
                    events.send(OrbitCameraControllerEvents::Pan(total_delta));
                }
            }
        }
    }

    // pub fn emit_camera_motion_events(
    //     // Input
    //     mut input_presses: EventReader<pointer::InputPress>,
    //     mut input_moves: EventReader<pointer::InputMove>,
    //     // Output
    //     mut camera_cmd_events: EventWriter<OrbitCameraControllerEvents>,
    // ) {
    //     let mut delta = Vec2::ZERO;

    //     for input_move in input_moves.iter() {
    //         delta += input_move.;

    //         // camera_cmd_events.send(OrbitCameraControllerEvents::Orbit(delta))
    //     }
    // }

    pub fn consume_pan_and_orbit_events(
        time: Res<Time>,
        mut events: EventReader<OrbitCameraControllerEvents>,
        mut query: Query<(&mut OrbitCameraController, &mut Transform, &mut Camera)>,
    ) {
        for (mut camera, transform, _) in query.iter_mut() {
            if !camera.enabled {
                continue;
            }

            for event in events.read() {
                match event {
                    OrbitCameraControllerEvents::Orbit(delta) => {
                        camera.x -= delta.x * camera.rotate_sensitivity * time.delta_secs();
                        camera.y -= delta.y * camera.rotate_sensitivity * time.delta_secs();
                        camera.y = camera
                            .y
                            .max(*camera.pitch_range.start())
                            .min(*camera.pitch_range.end());
                    }
                    OrbitCameraControllerEvents::Pan(delta) => {
                        let right_dir = transform.rotation * -Vec3::X;
                        let up_dir = transform.rotation * Vec3::Y;
                        let pan_vector = (delta.x * right_dir + delta.y * up_dir)
                            * camera.pan_sensitivity
                            * time.delta_secs();
                        camera.center += pan_vector;
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn emit_zoom_events(
        mut events: EventWriter<OrbitCameraControllerEvents>,
        mut mouse_wheel_events: EventReader<MouseWheel>,
        mut query: Query<&OrbitCameraController>,
    ) {
        // Skip zoom if hovering over UI or egui
        // let ui_hovering = ui_interaction.iter().any(|i| *i != Interaction::None);
        // if ui_hovering || egui_context.ctx_mut().wants_pointer_input() {
        //     return;
        // }

        let mut total = 0.0;
        for event in mouse_wheel_events.read() {
            total += event.y
                * match event.unit {
                    Line => 1.0,
                    Pixel => LINE_TO_PIXEL_RATIO,
                };
        }

        if total != 0.0 {
            for camera in query.iter_mut() {
                if camera.enabled {
                    events.send(OrbitCameraControllerEvents::Zoom(total));
                }
            }
        }
    }

    pub fn consume_zoom_events(
        mut query: Query<&mut OrbitCameraController, With<Camera>>,
        mut events: EventReader<OrbitCameraControllerEvents>,
    ) {
        for mut camera in query.iter_mut() {
            for event in events.read() {
                if camera.enabled {
                    if let OrbitCameraControllerEvents::Zoom(distance) = event {
                        camera.distance += camera.zoom_sensitivity * (*distance);
                        camera.distance = f32::clamp(camera.distance, 0.1, f32::MAX)
                    }
                }
            }
        }
    }
}
