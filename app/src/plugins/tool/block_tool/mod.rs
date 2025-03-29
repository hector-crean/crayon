
use crate::camera::CameraRaycast;
use crate::app_plugin::GroundPlane;
use bevy::window::PrimaryWindow;
use bevy::{color::palettes, prelude::*};
use block3d_core::block::{Block3DLike, BlockKind};
use crate::plugins::radial_menu::{
    CloseRadialMenu, OpenRadialMenu, RadialItemData, RadialMenuItemDataLike, RadialMenuPosition
};
use strum::IntoEnumIterator;

use super::ToolState;

// #[derive(Component)]
// pub struct DraftPreviewBlock;

#[derive(Default, SubStates, Debug, Clone, Eq, PartialEq, Hash)]
#[source(ToolState = ToolState::Block)]
pub enum BlockToolState {
    #[default]
    /// Actively previewing where the block will be placed
    Previewing,
    /// Choose the block kind
    ChooseBlockKind,
    /// Block has been placed but can still be adjusted (size, rotation, etc)
    Adjusting,
    /// Block placement is confirmed and complete
    Confirmed,
    /// Cancelled placement (returns to Ready state)
    Cancelled,
}

#[derive(Component)]
pub enum BlockUiState {
    Draft,
    Final,
}

#[derive(Event)]
pub enum BlockToolEvent {
    Adding { position: Vec3, rotation: Quat },
}

impl From<BlockKind> for RadialItemData {
    fn from(kind: BlockKind) -> Self {
        match kind {
            BlockKind::Window => RadialItemData {
                icon: "icons/window_48px.png".to_string(),
                color: palettes::tailwind::BLUE_400.into(),
                label: "Window".to_string(),
            },
            BlockKind::Door => RadialItemData {
                icon: "icons/door_48px.png".to_string(),
                color: palettes::tailwind::GREEN_400.into(),
                label: "Door".to_string(),
            },
            BlockKind::Wall => RadialItemData {
                icon: "icons/wall_48px.png".to_string(),
                color: palettes::tailwind::RED_400.into(),
                label: "Wall".to_string(),
            },
            BlockKind::Floor => RadialItemData {
                icon: "icons/floor_48px.png".to_string(),
                color: palettes::tailwind::YELLOW_400.into(),
                label: "Floor".to_string(),
            },
            BlockKind::Ceiling => RadialItemData {
                icon: "icons/ceiling_48px.png".to_string(),
                color: palettes::tailwind::PURPLE_400.into(),
                label: "Ceiling".to_string(),
            },
            BlockKind::Column => RadialItemData {
                icon: "icons/column_48px.png".to_string(),
                color: palettes::tailwind::ORANGE_400.into(),
                label: "Column".to_string(),
            },
            BlockKind::Stairs => RadialItemData {
                icon: "icons/stairs_48px.png".to_string(),
                color: palettes::tailwind::INDIGO_400.into(),
                label: "Stairs".to_string(),
            },
        }
    }
}

impl RadialMenuItemDataLike for BlockKind {}

#[derive(Component, Default)]
pub struct BlockToolPlugin<T: Block3DLike + Send + Sync + 'static>(T);

impl<T: Block3DLike + Send + Sync + 'static + Default> Plugin for BlockToolPlugin<T> {
    fn build(&self, app: &mut App) {
        app
            .init_state::<BlockToolState>()
            .add_event::<BlockToolEvent>()
            .add_systems(OnEnter(ToolState::Block), Self::setup_draft_block)
            .add_systems(OnExit(ToolState::Block), Self::despawn_draft_block)
            .add_systems(
                OnEnter(BlockToolState::ChooseBlockKind),
                Self::open_radial_menu,
            )
            .add_systems(
                OnExit(BlockToolState::ChooseBlockKind),
                Self::close_radial_menu,
            )
            .add_systems(
                Update,
                (
                    Self::update_preview_position.run_if(in_state(BlockToolState::Previewing)),
                    Self::handle_preview_click.run_if(in_state(BlockToolState::Previewing)),
                    Self::handle_adjusting.run_if(in_state(BlockToolState::Adjusting)),
                    Self::handle_confirmation.run_if(in_state(BlockToolState::Confirmed)),
                    Self::handle_cancellation.run_if(in_state(BlockToolState::Cancelled)),
                    Self::handle_escape_key,
                    Self::handle_state_transition,
                )
                    .run_if(in_state(ToolState::Block)),
            );
    }
}

impl<T: Block3DLike + Send + Sync + 'static + Default> BlockToolPlugin<T> {
    fn handle_state_transition(
        mut state_reader: EventReader<StateTransitionEvent<BlockToolState>>,
    ) {
        for event in state_reader.read() {
            info!(
                "BlockToolState state changed from {:?} to {:?}",
                event.exited, event.entered
            );
        }
    }

    fn handle_block_ui(
        mut commands: Commands,
        block_query: Query<(Entity, &BlockUiState), Changed<BlockUiState>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        let draft_material = materials.add(StandardMaterial {
            base_color: Color::rgba(1.0, 1.0, 1.0, 0.5), // Semi-transparent white
            alpha_mode: AlphaMode::Blend,
            ..default()
        });
        let final_material = materials.add(StandardMaterial {
            base_color: Color::rgba(1.0, 1.0, 1.0, 0.5), // Semi-transparent white
            alpha_mode: AlphaMode::Blend,
            ..default()
        });
        for (entity, block_state) in block_query.iter() {
            match *block_state {
                BlockUiState::Draft => {
                    commands
                        .entity(entity)
                        .insert((MeshMaterial3d(draft_material.clone()),));
                }
                BlockUiState::Final => {
                    commands
                        .entity(entity)
                        .insert((MeshMaterial3d(final_material.clone()),));
                }
            }
        }
    }
    fn setup_draft_block(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        commands.spawn((
            BlockUiState::Draft,
            Mesh3d(meshes.add(Cuboid::default())),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::rgba(1.0, 1.0, 1.0, 0.5), // Semi-transparent white
                alpha_mode: AlphaMode::Blend,
                ..default()
            })),
            Transform::default(),
        ));
    }

    fn handle_preview_click(
        mouse_input: Res<ButtonInput<MouseButton>>,
        mut next_state: ResMut<NextState<BlockToolState>>,
    ) {
        if mouse_input.just_pressed(MouseButton::Left) {
            next_state.set(BlockToolState::ChooseBlockKind);
        }
    }

    fn handle_adjusting(
        keyboard_input: Res<ButtonInput<KeyCode>>,
        mouse_input: Res<ButtonInput<MouseButton>>,
        next_state: ResMut<NextState<BlockToolState>>,
        mut query: Query<(&mut Transform, &BlockUiState)>,
    ) {
        // Handle rotation with R key
        if keyboard_input.just_pressed(KeyCode::KeyR) {
            for (mut transform, _) in query.iter_mut() {
                transform.rotate_local_y(90f32.to_radians());
            }
        }

        // Confirm placement with Enter or double-click
        // if keyboard_input.just_pressed(KeyCode::Enter)
        //     || (mouse_input.just_pressed(MouseButton::Left)
        //         && mouse_input.press_time(MouseButton::Left).unwrap() < 0.3) {
        //     next_state.set(BlockToolState::Confirmed);
        // }
    }

    fn handle_confirmation(
        mut commands: Commands,
        mut next_state_tool: ResMut<NextState<ToolState>>,
        query: Query<(Entity, &Transform, &BlockUiState)>,
        mut block_events: EventWriter<BlockToolEvent>,
    ) {
        for (entity, transform, state) in query.iter() {
            if matches!(state, BlockUiState::Draft) {
                // Emit event for block creation
                block_events.send(BlockToolEvent::Adding {
                    position: transform.translation,
                    rotation: transform.rotation,
                });

                // Update the block to final state
                commands.entity(entity).insert(BlockUiState::Final);
            }
        }

        // Return to transform tool
        next_state_tool.set(ToolState::Transform);
    }

    fn handle_cancellation(
        mut commands: Commands,
        mut next_state_tool: ResMut<NextState<ToolState>>,
        query: Query<(Entity, &BlockUiState)>,
    ) {
        for (entity, state) in query.iter() {
            if matches!(state, BlockUiState::Draft) {
                commands.entity(entity).despawn();
            }
        }

        // Return to transform tool
        next_state_tool.set(ToolState::Transform);
    }

    fn handle_escape_key(
        keyboard_input: Res<ButtonInput<KeyCode>>,
        mut next_state: ResMut<NextState<BlockToolState>>,
    ) {
        if keyboard_input.just_pressed(KeyCode::Escape) {
            next_state.set(BlockToolState::Cancelled);
        }
    }

    fn update_preview_position(
        camera_query: Single<(&Camera, &GlobalTransform)>,
        ground: Single<&GlobalTransform, With<GroundPlane>>,
        windows: Query<&Window>,
        mut preview_query: Query<(Entity, &mut Transform, &BlockUiState)>,
    ) {
        let Ok(windows) = windows.get_single() else {
            return;
        };
        let (camera, camera_transform) = *camera_query;
        let Some(cursor_position) = windows.cursor_position() else {
            return;
        };

        let Some(point) = camera.get_cursor_world_position(
            camera_transform,
            cursor_position,
            ground.translation(),
            ground.up().as_vec3(),
        ) else {
            return;
        };

        for (entity, mut transform, block_state) in preview_query.iter_mut() {
            match block_state {
                BlockUiState::Draft => {
                    transform.translation = point;
                }
                BlockUiState::Final => {}
            }
        }
    }

    fn despawn_draft_block(
        mut commands: Commands,
        preview_query: Query<(Entity, &BlockUiState), With<BlockUiState>>,
    ) {
        for (entity, block_state) in preview_query.iter() {
            match block_state {
                BlockUiState::Draft => {
                    commands.entity(entity).despawn();
                }
                BlockUiState::Final => {}
            }
        }
    }

    fn open_radial_menu(
        mut event_wtr: EventWriter<OpenRadialMenu>,
        window: Query<&Window, With<PrimaryWindow>>,
        camera_query: Query<(&Camera, &GlobalTransform)>,
        ground: Query<&GlobalTransform, With<GroundPlane>>,
    ) {
        let window = window.single();
        let (camera, camera_transform) = camera_query.single();
        let ground_transform = ground.single();

        // Get cursor position, return early if cursor is outside window
        let Some(cursor_position) = window.cursor_position() else {
            return;
        };

        // Get world position where cursor intersects ground plane
        let Some(world_position) = camera.get_cursor_world_position(
            camera_transform,
            cursor_position,
            ground_transform.translation(),
            ground_transform.up().as_vec3(),
        ) else {
            return;
        };

        event_wtr.send(OpenRadialMenu {
            items: BlockKind::iter().map(RadialItemData::from).collect(),
            position: RadialMenuPosition::WorldSpace(world_position),
        });
    }
    fn close_radial_menu(mut event_wtr: EventWriter<CloseRadialMenu>) {
        event_wtr.send(CloseRadialMenu);
    }
}
