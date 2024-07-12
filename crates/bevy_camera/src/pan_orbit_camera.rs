use crate::api::{CameraController, CameraMode};
use bevy::{
    input::mouse::{
        MouseButtonInput, MouseMotion,
        MouseScrollUnit::{Line, Pixel},
        MouseWheel,
    },
    prelude::*,
    render::camera::Camera,
};
use bevy_mod_picking::{
    debug::DebugPickingPlugin,
    pointer::InputMove,
    prelude::{Click, Down, Drag, DragEnd, DragStart, Pointer, PointerButton, Up},
    DefaultPickingPlugins,
};
use std::ops::RangeInclusive;

#[derive(Default)]
pub struct OrbitCameraControllerPlugin<T: CameraMode>(pub T);

impl<T: CameraMode + Send + Sync + 'static> Plugin for OrbitCameraControllerPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_systems(First, Self::init_camera_state)
            .add_event::<OrbitCameraControllerEvents>()
            .add_systems(
                PostUpdate,
                (Self::emit_motion_events).run_if(
                    not(on_event::<Pointer<DragStart>>())
                        .and_then(not(on_event::<Pointer<Drag>>()))
                        .and_then(not(on_event::<Pointer<DragEnd>>()))
                        .and_then(not(on_event::<Pointer<Down>>()))
                        .and_then(not(on_event::<Pointer<Up>>()))
                        .and_then(not(on_event::<Pointer<Click>>()))
                        .and_then(run_criteria::<T>),
                ),
            )
            .add_systems(Update, (Self::emit_zoom_events))
            .add_systems(
                Last,
                (
                    Self::consume_pan_and_orbit_events,
                    Self::consume_zoom_events,
                    Self::update_camera_transform_system,
                )
                    .chain()
                    .run_if(on_event::<OrbitCameraControllerEvents>()),
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
    pub x: f32,
    pub y: f32,
    pub pitch_range: RangeInclusive<f32>,
    pub distance: f32,
    pub center: Vec3,
    pub rotate_sensitivity: f32,
    pub pan_sensitivity: f32,
    pub zoom_sensitivity: f32,
    pub rotate_button: MouseButton,
    pub pan_button: MouseButton,
    pub enabled: bool,
}

impl Default for OrbitCameraController {
    fn default() -> Self {
        OrbitCameraController {
            x: 0.0,
            y: std::f32::consts::FRAC_PI_2,
            pitch_range: 0.01..=3.13,
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
    pub fn new(dist: f32, center: Vec3) -> OrbitCameraController {
        OrbitCameraController {
            distance: dist,
            center,
            ..Self::default()
        }
    }
}

impl CameraController for OrbitCameraController {
    fn update_camera_transform_system(
        mut query: Query<
            (&OrbitCameraController, &mut Transform),
            (Changed<OrbitCameraController>, With<Camera>),
        >,
    ) {
        for (camera, mut transform) in query.iter_mut() {
            if camera.enabled {
                let rot = Quat::from_axis_angle(Vec3::Y, camera.x)
                    * Quat::from_axis_angle(-Vec3::X, camera.y);
                transform.translation = (rot * Vec3::Y) * camera.distance + camera.center;
                transform.look_at(camera.center, Vec3::Y);
            }
        }
    }
}

impl<T: CameraMode> OrbitCameraControllerPlugin<T> {
    pub fn init_camera_state(mut commands: Commands) {
        commands.init_resource::<T>()
    }

    pub fn update_camera_transform_system(
        query: Query<
            (&OrbitCameraController, &mut Transform),
            (Changed<OrbitCameraController>, With<Camera>),
        >,
    ) {
        OrbitCameraController::update_camera_transform_system(query);
    }

    pub fn emit_motion_events(
        mut events: EventWriter<OrbitCameraControllerEvents>,
        mut pointer_motion_events: EventReader<InputMove>,
        pointer_button_input: Res<ButtonInput<MouseButton>>,
        mut query: Query<&OrbitCameraController>,
    ) {
        let mut delta = Vec2::ZERO;
        for event in pointer_motion_events.read() {
            delta += event.delta;
        }
        for camera in query.iter_mut() {
            if camera.enabled {
                if pointer_button_input.pressed(camera.rotate_button) {
                    events.send(OrbitCameraControllerEvents::Orbit(delta));
                }

                if pointer_button_input.pressed(camera.pan_button) {
                    events.send(OrbitCameraControllerEvents::Pan(delta));
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
                        camera.x -= delta.x * camera.rotate_sensitivity * time.delta_seconds();
                        camera.y -= delta.y * camera.rotate_sensitivity * time.delta_seconds();
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
                            * time.delta_seconds();
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
