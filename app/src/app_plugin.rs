use crate::event::{CrayonInEvent, CrayonOutEvent};
use crate::event_channel::GLOBAL_EVENT_CHANNEL;
use crate::materials::button_material::ButtonMaterialPlugin;
use crate::plugins::duplex_plugin::create_duplex_plugin;
use crate::plugins::interactive_mesh::InteractiveMeshPlugin;
use crate::plugins::pointer_events::DoubleClickPlugin;
use crate::plugins::radial_menu::RadialMenuPlugin;
use crate::plugins::tool::toolbar::ToolbarPlugin;
use crate::plugins::tool::ToolPlugin;
use crate::state::camera::CameraModeImpl;
use bevy::app::App;
use bevy::prelude::*;
use bevy::window::{PresentMode, WindowTheme};
use bevy::winit::WinitSettings;
use bevy_camera::pan_orbit_camera::{OrbitCameraController, OrbitCameraControllerPlugin};
use bevy_camera::MainCamera;
use bevy_crayon_materials::wireframe::VoidMaterialPlugin;
use bevy_debug_grid::{DebugGridPlugin, Grid};
use bevy_gaussian_splatting::GaussianCamera;
use bevy_inspector_egui::bevy_egui::EguiContexts;
use bevy_mod_outline::OutlinePlugin;
use block3d_core::block::Block3DLike;
use bevy_mod_reqwest::ReqwestPlugin;
use bevy_polyline::PolylinePlugin;
use bevy_wfc::{WFCPlugin, WFCSolveComplete, WFCSolveRequest};
use block3d_core::block::lego_block::LegoBlock;
use block3d_core::block::{Block3D, BlockKind};
use block3d_core::Orientation;
use block3d_core::face::Face;
use block3d_core::connection::{OrientedInterface, ConnectorInterface};
use std::collections::HashSet;
use std::time::Duration;
use std::collections::HashMap;
use bevy::input::common_conditions::input_just_pressed;
use bevy::picking::events::Pointer;
use bevy::picking::pointer::PointerButton;



pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        // Set up console error logging
        console_error_panic_hook::set_once();

        let (duplex_plugin, js_rx, js_tx) = create_duplex_plugin::<CrayonInEvent, CrayonOutEvent>();

        GLOBAL_EVENT_CHANNEL.set_sender(js_tx);
        GLOBAL_EVENT_CHANNEL.set_receiver(js_rx);

        app.add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Self::configure_window()),
                    ..default()
                })
                .set(AssetPlugin {
                    watch_for_changes_override: Some(true),
                    ..Default::default()
                })
                .build(),
        );
        app.insert_resource(WinitSettings {
            focused_mode: bevy::winit::UpdateMode::Reactive { wait: Duration::from_millis(10), react_to_device_events: true, react_to_user_events: true, react_to_window_events: true },
            unfocused_mode: bevy::winit::UpdateMode::reactive_low_power(Duration::from_millis(10)),
        });


        app
           .add_plugins((
                duplex_plugin,
                OrbitCameraControllerPlugin::<CameraModeImpl>::default(),
                ToolPlugin,
                ReqwestPlugin::default(),
                MeshPickingPlugin,
            ))
           .add_plugins((
                ButtonMaterialPlugin,
                PolylinePlugin,
                DoubleClickPlugin,
                DebugGridPlugin::default(),
                // CursorPlugin,
            ))
           .add_plugins((
                InteractiveMeshPlugin,
                WFCPlugin::default(),
                GroundPlanePlugin,
                RadialMenuPlugin,
                VoidMaterialPlugin,
            ));

        #[cfg(not(target_arch = "wasm32"))]
        app.add_plugins(ToolbarPlugin);

        app.add_systems(Startup, (Self::setup_camera))
            .add_systems(
                Update,
                (
                    CrayonInEvent::handle.run_if(on_event::<CrayonInEvent>),
                    CrayonOutEvent::handle.run_if(on_event::<CrayonOutEvent>),
                ),
            );

    }
}

impl AppPlugin {
    fn configure_window() -> Window {
        Window {
            title: "Crayon".into(),
            name: Some("bevy.app".into()), 
            canvas: Some("#crayon-canvas".into()),
            resolution: (500., 300.).into(),
            present_mode: PresentMode::AutoVsync,
            fit_canvas_to_parent: true,
            prevent_default_event_handling: true,
            window_theme: Some(WindowTheme::Dark),
            enabled_buttons: bevy::window::EnabledButtons {
                maximize: false,
                ..Default::default()
            },
            visible: true,
            ..default()
        }
    }

    fn setup_camera(mut commands: Commands) {
        let controller = OrbitCameraController::default();
        let transform = controller.generate_camera_transform();

        // Add camera
        commands.spawn((
            Camera3d::default(),
            GaussianCamera::default(),
            MainCamera,
            controller,
            transform,
        ));

        // Add ambient light with improved settings
        commands.insert_resource(AmbientLight {
            color: Color::rgb(0.9, 0.9, 1.0), // Slightly blue-tinted white for better atmosphere
            brightness: 0.4,                  // Increased brightness for better visibility
        });

        // Add primary directional light (sun-like)
        commands.spawn((
            DirectionalLight {
                color: Color::rgb(1.0, 0.95, 0.9), // Warm sunlight color
                illuminance: 12000.0,              // More realistic illuminance value
                shadows_enabled: true,
                shadow_depth_bias: 0.02,           // Reduce shadow acne
                shadow_normal_bias: 0.6,           // Improve shadow quality
                ..default()
            },
            Transform::from_xyz(4.0, 8.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
        ));

        // Add secondary fill light (opposite direction, lower intensity)
        commands.spawn((
            DirectionalLight {
                color: Color::rgb(0.8, 0.85, 1.0), // Cooler color for fill light
                illuminance: 3000.0,               // Lower intensity than main light
                shadows_enabled: false,            // No shadows from fill light
                ..default()
            },
            Transform::from_xyz(-3.0, 5.0, -3.0).looking_at(Vec3::ZERO, Vec3::Y),
        ));
    }
}


pub struct GroundPlanePlugin;

impl Plugin for GroundPlanePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, GroundPlane::spawn)
            .add_systems(Update, GroundPlane::draw_cursor);
    }
}

#[derive(Component)]
pub struct GroundPlane;

impl GroundPlane {
    fn spawn(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        commands.spawn((
            Self,
            Transform::from_translation(Vec3::ZERO),
            Mesh3d(meshes.add(Plane3d::default())),
            MeshMaterial3d(materials.add(StandardMaterial::default())),
            // PickingBehavior::IGNORE,
            Grid {
                spacing: 1.0,
                count: 10,
                color: Color::WHITE,
                alpha_mode: AlphaMode::Blend,
            },
        ));
    }

    fn draw_cursor(
        camera_query: Single<(&Camera, &GlobalTransform)>,
        ground: Single<&GlobalTransform, With<Self>>,
        windows: Query<&Window>,
        mut gizmos: Gizmos,
    ) {
        let Ok(windows) = windows.get_single() else {
            return;
        };

        let (camera, camera_transform) = *camera_query;

        let Some(cursor_position) = windows.cursor_position() else {
            return;
        };

        // Calculate a ray pointing from the camera into the world based on the cursor's position.
        let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
            return;
        };

        // Calculate if and where the ray is hitting the ground plane.
        let Some(distance) =
            ray.intersect_plane(ground.translation(), InfinitePlane3d::new(ground.up()))
        else {
            return;
        };
        let point = ray.get_point(distance);

        // Draw a circle just above the ground plane at that position.
        gizmos.circle(
            Isometry3d::new(
                point + ground.up() * 0.01,
                Quat::from_rotation_arc(Vec3::Z, ground.up().as_vec3()),
            ),
            0.2,
            Color::WHITE,
        );
    }
}
