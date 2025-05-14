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
use bevy_debug_grid::{DebugGridPlugin, Grid};
use bevy_gaussian_splatting::GaussianCamera;
use bevy_inspector_egui::bevy_egui::EguiContexts;
use block3d_core::block::Block3DLike;
use bevy_mod_reqwest::ReqwestPlugin;
use bevy_polyline::PolylinePlugin;
use bevy_wfc::{WFCPlugin, WFCSolveComplete, WFCSolveRequest, UserTriggeredCollapse};
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

// Component to store block information for debugging
#[derive(Component)]
struct BlockInfo {
    kind: BlockKind,
    position: (usize, usize, usize),
    orientation: Orientation,
    size: (u32, u32, u32),
}

impl BlockInfo {
    fn new(kind: BlockKind, position: (usize, usize, usize), orientation: Orientation, size: (u32, u32, u32)) -> Self {
        Self { kind, position, orientation, size }
    }
}

// Component to identify text markers
#[derive(Component)]
struct BlockTypeMarker;

const SHAPES_X_EXTENT: f32 = 14.0;
const EXTRUSION_X_EXTENT: f32 = 16.0;
const Z_EXTENT: f32 = 5.0;

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
                WFCPlugin,
                GroundPlanePlugin,
                RadialMenuPlugin,
            ));

        #[cfg(not(target_arch = "wasm32"))]
        app.add_plugins(ToolbarPlugin);

        app.add_systems(Startup, (Self::setup_camera, setup_wfc))
            .add_systems(
                Update,
                (
                    handle_wfc_results.run_if(on_event::<WFCSolveComplete>),
                    CrayonInEvent::handle.run_if(on_event::<CrayonInEvent>),
                    CrayonOutEvent::handle.run_if(on_event::<CrayonOutEvent>),
                    check_ui_interaction,
                    handle_grid_click.run_if(on_event::<Pointer::<MouseButton>>),
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

        // Add ambient light
        commands.insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 0.3,
        });

        // Add directional light
        commands.spawn((
            DirectionalLight {
                color: Color::WHITE,
                illuminance: 10000.0,
                shadows_enabled: true,
                ..default()
            },
            Transform::from_xyz(4.0, 8.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
        ));
    }
}

fn setup_wfc(mut commands: Commands, mut solve_requests: EventWriter<WFCSolveRequest>) {
    // Create initial block set with various block types and proper connection faces
    let mut block_set = HashSet::new();

    // Create face with stud interface for top
    let stud_face = Face::new(OrientedInterface {
        interface: ConnectorInterface::Stud,
        orientation: Orientation::O0,
    });

    // Create face with tube interface for bottom
    let tube_face = Face::new(OrientedInterface {
        interface: ConnectorInterface::Tube,
        orientation: Orientation::O0,
    });

    // Add 1x1x1 blocks with proper connection faces
    block_set.insert(Block3D::Lego(LegoBlock::new(
        (1, 1, 1),
        BlockKind::Wall,
        vec![stud_face.clone(), tube_face.clone()],
    )));
    block_set.insert(Block3D::Lego(LegoBlock::new(
        (1, 1, 1),
        BlockKind::Floor,
        vec![stud_face.clone(), tube_face.clone()],
    )));
    block_set.insert(Block3D::Lego(LegoBlock::new(
        (1, 1, 1),
        BlockKind::Door,
        vec![stud_face.clone(), tube_face.clone()],
    )));
    block_set.insert(Block3D::Lego(LegoBlock::new(
        (1, 1, 1),
        BlockKind::Window,
        vec![stud_face.clone(), tube_face.clone()],
    )));
    
    // Add a few 2x1x1 blocks for variety
    block_set.insert(Block3D::Lego(LegoBlock::new(
        (2, 1, 1),
        BlockKind::Wall,
        vec![stud_face.clone(), tube_face.clone()],
    )));
    block_set.insert(Block3D::Lego(LegoBlock::new(
        (2, 1, 1),
        BlockKind::Floor,
        vec![stud_face.clone(), tube_face.clone()],
    )));

    // Use a 3x3x3 grid for a more interesting structure
    solve_requests.send(WFCSolveRequest {
        dimensions: (5, 5, 5),  // Increased depth for more complex structures
        block_set,
    });
    
    // FUTURE ENHANCEMENTS:
    // 1. Update WFCSolveRequest to accept invariants like GravityInvariant
    // 2. Add compatibility rules between blocks for proper LEGO connections
    // 3. Implement more sophisticated heuristics for block selection
    // 4. Add support for different block shapes and connection types
    // 5. Include constraints for structural integrity
}

fn handle_wfc_results(mut complete_events: EventReader<WFCSolveComplete>, mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    for event in complete_events.read() {
        match &event.result {
            Ok(graph) => {
                // Create a summary of block types
                let mut block_type_counts = HashMap::new();
                
                for node_idx in graph.node_indices() {
                    if let Some(node_state) = graph.node_weight(node_idx) {
                        let block_kind = node_state.block.block_kind();
                        *block_type_counts.entry(block_kind).or_insert(0) += 1;
                    }
                }
                
                // Log summary and detailed info at different log levels
                info!("WFC solution found!");
                info!("---------------------");
                info!("Total nodes: {}", graph.node_count());
                
                for (block_kind, count) in &block_type_counts {
                    info!("  - {}: {} blocks", block_kind, count);
                }
                
                // Detailed graph info at debug level instead of info
                info!("\nDetailed graph structure:\n{}", graph.pretty_print());
                
                // Get the dimensions of the WFC solution
                // Since we don't have direct access to the dimensions that were used to create the grid,
                // we'll calculate an approximate grid size based on the number of nodes
                let node_count = graph.node_count();
                let approx_size = (node_count as f32).cbrt().ceil() as usize;
                let grid_size = approx_size;
                
                // For each node in the graph, create a visual representation
                for node_idx in graph.node_indices() {
                    if let Some(node_state) = graph.node_weight(node_idx) {
                        // Get the actual position stored in the node state
                        let (x, y, z) = node_state.position;
                        
                        // Get block information
                        let block = &node_state.block;
                        let orientation = node_state.orientation;
                        let block_kind = block.block_kind();
                        let size = block.size();
                        
                        // Create a position vector, spacing blocks by 1 unit
                        let position = Vec3::new(x as f32, y as f32, z as f32);
                        
                        // Create material based on block kind with higher transparency
                        let material = materials.add(match block_kind {
                            BlockKind::Wall => StandardMaterial {
                                base_color: Color::rgba(0.8, 0.8, 0.8, 0.6), // Light gray for walls with transparency
                                alpha_mode: AlphaMode::Blend,
                                ..default()
                            },
                            BlockKind::Floor => StandardMaterial {
                                base_color: Color::rgba(0.6, 0.5, 0.4, 0.6), // Brown for floors with transparency
                                alpha_mode: AlphaMode::Blend,
                                ..default()
                            },
                            BlockKind::Door => StandardMaterial {
                                base_color: Color::rgba(0.6, 0.3, 0.2, 0.6), // Brown for doors with transparency
                                alpha_mode: AlphaMode::Blend,
                                ..default()
                            },
                            BlockKind::Window => StandardMaterial {
                                base_color: Color::rgba(0.3, 0.7, 0.9, 0.6), // Blue for windows with transparency
                                alpha_mode: AlphaMode::Blend,
                                ..default()
                            },
                            _ => StandardMaterial {
                                base_color: Color::rgba(1.0, 1.0, 1.0, 0.6), // White for other types with transparency
                                alpha_mode: AlphaMode::Blend,
                                ..default()
                            },
                        });
                        
                        // Determine the size of the block
                        let (width, height, depth) = size;
                        let block_size = Vec3::new(width as f32, height as f32, depth as f32);
                        
                        // Create a cuboid mesh based on the block's size
                        let mesh = meshes.add(Cuboid::new(
                            block_size.x, 
                            block_size.y, 
                            block_size.z
                        ));
                        
                        // Convert orientation to rotation
                        let rotation = match orientation {
                            Orientation::O0 => Quat::from_rotation_y(0.0),
                            Orientation::O90 => Quat::from_rotation_y(std::f32::consts::FRAC_PI_2),
                            Orientation::O180 => Quat::from_rotation_y(std::f32::consts::PI),
                            Orientation::O270 => Quat::from_rotation_y(3.0 * std::f32::consts::FRAC_PI_2),
                        };
                        
                        // Spawn entity with mesh, material, and block info component
                        let block_entity = commands.spawn((
                            Mesh3d(mesh),
                            MeshMaterial3d(material),
                            Transform::from_translation(position)
                                .with_rotation(rotation),
                            GlobalTransform::default(),
                            // Add component to store block info for debugging
                            BlockInfo::new(block_kind, (x, y, z), orientation, size),
                        )).id();
                    }
                }
            }
            Err(e) => {
                error!("WFC generation failed: {}", e);
            }
        }
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

fn check_ui_interaction(
    camera_mode: ResMut<CameraModeImpl>,
    ui_interaction: Query<&Interaction, (With<Node>, Changed<Interaction>)>,
    egui_context: EguiContexts,
) {
    // Check for UI interaction
    // let ui_hovering = ui_interaction
    //     .iter()
    //     .any(|interaction| *interaction != Interaction::None);

    // Check for egui interaction
    // let egui_hovering = egui_context.ctx_mut().wants_pointer_input() || 
    //                    egui_context.ctx_mut().wants_keyboard_input();

    // camera_mode.set_ui_interaction(ui_hovering || egui_hovering);
}

// System to handle user clicks on grid cells
fn handle_grid_click(
    mut commands: Commands,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    ground_plane_query: Query<&GlobalTransform, With<GroundPlane>>, // Assuming you have a GroundPlane entity
    windows: Query<&Window>,
    mut wfc_collapse_writer: EventWriter<UserTriggeredCollapse>,
    // Add query for pickable blocks if you want to click existing blocks instead of empty grid cells
    // pick_query: Query<(&PickSelection, &BlockInfo)>, 
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let Ok(window) = windows.get_single() else { return };
        let Some(cursor_pos) = window.cursor_position() else { return };
        let Ok((camera, camera_transform)) = camera_query.get_single() else { return };
        let Ok(ground_transform) = ground_plane_query.get_single() else { return }; // Get ground plane transform


        // Raycast from camera to ground plane
        if let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_pos) {
            if let Some(distance) = ray.intersect_plane(ground_transform.translation(), InfinitePlane3d::new(ground_transform.up())) {
                let click_point = ray.get_point(distance);
                
                // Convert click_point to grid coordinates (assuming 1x1x1 grid cells)
                let grid_x = click_point.x.round() as usize;
                let grid_y = click_point.y.round() as usize;
                let grid_z = click_point.z.round() as usize;
                
                info!("Grid cell clicked at: ({}, {}, {})", grid_x, grid_y, grid_z);
                
                // Send the UserTriggeredCollapse event
                wfc_collapse_writer.send(UserTriggeredCollapse { position: (grid_x, grid_y, grid_z) });
            }
        }
    }
}

