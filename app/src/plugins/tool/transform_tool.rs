use std::f32::consts::PI;
use std::fmt::Debug;

use crate::plugins::interactive_mesh::drag::Draggable;
use crate::plugins::interactive_mesh::selection::Selected;
use crate::state::camera::CameraModeImpl;
use bevy::math::{Quat, Vec3};
use bevy::prelude::*;
use bevy::state::state::FreelyMutableState;
use bevy_drag::controllers::transform_controller::{TransformBounds, TransformController};
use bevy_drag::controllers::transform_controller::TransformControllerSettings;
use leafwing_input_manager::plugin::InputManagerPlugin;
use leafwing_input_manager::prelude::{ActionState, InputMap};
use leafwing_input_manager::Actionlike;
use strum::{EnumIter, IntoEnumIterator};
use ts_rs::TS;
use bevy_camera::CameraMode;
use super::ToolState;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect, Default, EnumIter, Actionlike)]
pub enum Direction {
    #[default]
    PlusX,
    MinusX,
    PlusY,
    MinusY,
    PlusZ,
    MinusZ,
}

impl From<Direction> for Dir3 {
    fn from(value: Direction) -> Self {
        match value {
            Direction::PlusX => Dir3::X,
            Direction::MinusX => Dir3::NEG_X,
            Direction::PlusY => Dir3::Y,
            Direction::MinusY => Dir3::NEG_Y,
            Direction::PlusZ => Dir3::Z,
            Direction::MinusZ => Dir3::NEG_Z,
        }
    }
}

impl Direction {
    fn keybindings() -> InputMap<Self> {
        InputMap::new([
            (Self::PlusX, KeyCode::ArrowLeft),
            (Self::MinusX, KeyCode::ArrowRight),
            (Self::PlusY, KeyCode::ArrowUp),
            (Self::MinusY, KeyCode::ArrowDown),
            (Self::PlusZ, KeyCode::PageUp),
            (Self::MinusZ, KeyCode::PageDown),
        ])
    }
    fn remap(self, handedness: Handedness, principal_axis: PrincipalAxis) -> Self {
        match (handedness, principal_axis) {
            (Handedness::Right, PrincipalAxis::X1) => self,
            (Handedness::Right, PrincipalAxis::X2) => match self {
                Direction::PlusX => Direction::PlusY,
                Direction::MinusX => Direction::MinusY,
                Direction::PlusY => Direction::PlusZ,
                Direction::MinusY => Direction::MinusZ,
                Direction::PlusZ => Direction::PlusX,
                Direction::MinusZ => Direction::MinusX,
            },
            (Handedness::Right, PrincipalAxis::X3) => match self {
                Direction::PlusX => Direction::PlusZ,
                Direction::MinusX => Direction::MinusZ,
                Direction::PlusY => Direction::PlusX,
                Direction::MinusY => Direction::MinusX,
                Direction::PlusZ => Direction::PlusY,
                Direction::MinusZ => Direction::MinusY,
            },
            (Handedness::Left, PrincipalAxis::X1) => match self {
                Direction::PlusX => Direction::MinusX,
                Direction::MinusX => Direction::PlusX,
                Direction::PlusY => Direction::MinusY,
                Direction::MinusY => Direction::PlusY,
                Direction::PlusZ => Direction::MinusZ,
                Direction::MinusZ => Direction::PlusZ,
            },
            (Handedness::Left, PrincipalAxis::X2) => match self {
                Direction::PlusX => Direction::MinusY,
                Direction::MinusX => Direction::PlusY,
                Direction::PlusY => Direction::MinusZ,
                Direction::MinusY => Direction::PlusZ,
                Direction::PlusZ => Direction::MinusX,
                Direction::MinusZ => Direction::PlusX,
            },
            (Handedness::Left, PrincipalAxis::X3) => match self {
                Direction::PlusX => Direction::MinusZ,
                Direction::MinusX => Direction::PlusZ,
                Direction::PlusY => Direction::MinusX,
                Direction::MinusY => Direction::PlusX,
                Direction::PlusZ => Direction::MinusY,
                Direction::MinusZ => Direction::PlusY,
            },
            _ => self,
        }
    }
}

#[derive(
    PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect, Default, EnumIter, Actionlike, Resource,
)]
pub enum PrincipalAxis {
    #[default]
    X1,
    X2,
    X3,
}

impl From<PrincipalAxis> for Dir3 {
    fn from(value: PrincipalAxis) -> Self {
        match value {
            PrincipalAxis::X1 => Dir3::X,
            PrincipalAxis::X2 => Dir3::Y,
            PrincipalAxis::X3 => Dir3::Z,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect, Default, Resource)]
pub enum Handedness {
    #[default]
    Right,
    Left,
}

impl PrincipalAxis {
    fn keybindings() -> InputMap<Self> {
        InputMap::new([
            (Self::X1, KeyCode::KeyX),
            (Self::X2, KeyCode::KeyY),
            (Self::X3, KeyCode::KeyZ),
        ])
    }
}

#[derive(
    PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect, Default, EnumIter, Actionlike, Resource,
)]
pub enum TransformSpace {
    #[default]
    Global,
    Local,
}
impl TransformSpace {
    fn keybindings() -> InputMap<Self> {
        InputMap::new([(Self::Global, KeyCode::KeyG), (Self::Local, KeyCode::KeyL)])
    }
}

// Context: Selected Entity

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect, EnumIter, Event)]
pub enum TransformEvent {
    Translate(PrincipalAxis, TransformSpace, Direction),
    Rotate(PrincipalAxis, TransformSpace, Direction),
    Scale(PrincipalAxis, TransformSpace, Direction),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, SubStates, Default, Component, Actionlike, Reflect)]
#[source(ToolState = ToolState::Transform)]
pub enum TransformToolState {
    #[default]
    Translate,
    Scale,
    Rotate,
}

impl TransformToolState {
    fn keybindings() -> InputMap<Self> {
        InputMap::new([
            (Self::Translate, KeyCode::KeyT),
            (Self::Rotate, KeyCode::KeyR),
            (Self::Scale, KeyCode::KeyS),
        ])
    }
}

pub struct TransformToolPlugin;

impl TransformToolPlugin {
    pub fn setup(
        mut commands: Commands,
        query: Query<Entity, (
            With<Transform>,
            Without<Draggable>,
            Without<TransformController>,
        )>,
    ) {
        info!("Setting up TransformToolPlugin");
        
        // Enable transform controls
        commands.insert_resource(TransformControllerSettings {
            enabled: true,
            ..default()
        });

        // Add required components for transformation
        for entity in query.iter() {
            info!("Adding transform components to entity: {}", entity);
            commands.entity(entity)
                .insert(Draggable::default())
                .insert(TransformController::default())
                .insert(TransformBounds::default());
        }
    }

    fn handle_transform_state_transition(
        mut camera_controller: ResMut<CameraModeImpl>,
        mut state_events: EventReader<StateTransitionEvent<TransformToolState>>,
    ) {
        for event in state_events.read() {
            // Ensure camera is unlocked when transitioning states
            camera_controller.unlock();
            info!("Transform tool state transition: {:?} -> {:?}", event.exited, event.entered);
        }
    }


    fn update_resource_with_keyboard<T: Actionlike + Resource + Debug>(
        keys: Res<ActionState<T>>,
        mut state: ResMut<T>,
    ) {
        let pressed_keys = keys.get_just_pressed();

        for key in pressed_keys {
            info!("Updated state with key: {:?}", key);
            *state = key;
        }
    }

    fn update_state_with_keyboard<T: Actionlike + States + FreelyMutableState>(
        keys: Res<ActionState<T>>,
        mut next_tool_state: ResMut<NextState<T>>,
    ) {
        let pressed_keys = keys.get_just_pressed();

        for key in pressed_keys {
            info!("Set next tool state with key: {:?}", key);
            next_tool_state.set(key);
        }
    }

    fn emit_transform_event(
        axis_direction_actions: Res<ActionState<Direction>>,
        axis: Res<PrincipalAxis>,
        transform_space: Res<TransformSpace>,
        transform_tool_state: Res<State<TransformToolState>>,
        mut event_writer: EventWriter<TransformEvent>,
    ) {
        for axis_direction in axis_direction_actions.get_just_pressed() {
            let event = match transform_tool_state.get() {
                TransformToolState::Translate => {
                    TransformEvent::Translate(*axis, *transform_space, axis_direction)
                }
                TransformToolState::Rotate => {
                    TransformEvent::Rotate(*axis, *transform_space, axis_direction)
                }
                TransformToolState::Scale => {
                    TransformEvent::Scale(*axis, *transform_space, axis_direction)
                }
            };

            info!("Generated event: {:?}", event);
            event_writer.send(event);
        }
    }

    fn consume_transform_event(
        mut event_reader: EventReader<TransformEvent>,
        mut selected_query: Query<&mut Transform, With<Selected>>,
    ) {
        info!("Consuming transform events");

        let translation_amount: f32 = 1.0;
        let rotation_amount: f32 = PI / 4.0;
        let scale_amount: f32 = 1.1;

        for event in event_reader.read() {
            for mut transform in selected_query.iter_mut() {
                info!("Processing event: {:?}", event);

                match *event {
                    TransformEvent::Translate(principal_axis, space, translation_direction) => {
                        let dir = Dir3::from(translation_direction.remap(Handedness::Right, principal_axis));
                        let translation_vector = translation_amount * Vec3::from(dir);

                        match space {
                            TransformSpace::Global => {
                                transform.translation += translation_vector;
                            }
                            TransformSpace::Local => {
                                let local_translation: Vec3 = transform.rotation * translation_vector;
                                transform.translation += local_translation;
                            }
                        }
                    }
                    TransformEvent::Rotate(principal_axis, space, rotation_direction) => {
                        let dir = Dir3::from(rotation_direction.remap(Handedness::Right, principal_axis));
                        let rotation_quat = Quat::from_axis_angle(Vec3::from(dir), rotation_amount);

                        match space {
                            TransformSpace::Global => {
                                transform.rotation = rotation_quat * transform.rotation;
                            }
                            TransformSpace::Local => {
                                transform.rotation *= rotation_quat;
                            }
                        }
                    }
                    TransformEvent::Scale(principal_axis, space, scale_direction) => {
                        let dir = Dir3::from(scale_direction.remap(Handedness::Right, principal_axis));
                        let scale_vector = Vec3::from(dir).map(|component| {
                            match component {
                                1.0 => scale_amount,
                                -1.0 => 1.0 / scale_amount,
                                _ => 1.0,
                            }
                        });

                        match space {
                            TransformSpace::Global => {
                                transform.scale *= scale_vector;
                            }
                            TransformSpace::Local => {
                                let local_scale = transform.rotation * scale_vector;
                                transform.scale *= local_scale;
                            }
                        }
                    },
                }
            }
        }
    }

    pub fn cleanup(
        mut commands: Commands,
        mut camera_controller: ResMut<CameraModeImpl>,
        query: Query<(Entity, &TransformController), With<Draggable>>,
    ) {
        info!("Cleaning up TransformToolPlugin");
        
        // Ensure camera is unlocked when exiting transform tool
        camera_controller.unlock();

        // Disable transform controls
        commands.insert_resource(TransformControllerSettings {
            enabled: false,
            ..default()
        });

        // Reset transform controller state and remove draggable
        for (entity, _) in query.iter() {
            info!("Cleaning up transform components for entity: {}", entity);
            commands.entity(entity)
                .remove::<Draggable>()
                .remove::<TransformController>();
        }
    }
}

impl Plugin for TransformToolPlugin {
    fn build(&self, app: &mut App) {
        // if !app.is_plugin_added::<DragTransformPlugin<CameraModeImpl>>() {
        //     app.add_plugins(DragTransformPlugin::<CameraModeImpl>::default());
        // }

        app.init_state::<TransformToolState>()
            // Register the TransformEvent to the app's event system
            .add_event::<TransformEvent>()
            // Add the InputManagerPlugin for handling TransformToolState inputs
            .add_plugins(InputManagerPlugin::<TransformToolState>::default())
            // Initialize the ActionState resource for TransformToolState
            .init_resource::<ActionState<TransformToolState>>()
            // Insert keybindings for TransformToolState actions
            .insert_resource(TransformToolState::keybindings())
            // Initialize the Axis resource
            .init_resource::<PrincipalAxis>()
            // Add the InputManagerPlugin for handling Axis inputs
            .add_plugins(InputManagerPlugin::<PrincipalAxis>::default())
            // Initialize the ActionState resource for Axis
            .init_resource::<ActionState<PrincipalAxis>>()
            // Insert keybindings for Axis actions
            .insert_resource(PrincipalAxis::keybindings())
            // Initialize the TransformSpace resource
            .init_resource::<TransformSpace>()
            // Add the InputManagerPlugin for handling TransformSpace inputs
            .add_plugins(InputManagerPlugin::<TransformSpace>::default())
            // Initialize the ActionState resource for TransformSpace
            .init_resource::<ActionState<TransformSpace>>()
            // Insert keybindings for TransformSpace actions
            .insert_resource(TransformSpace::keybindings())
            // Add the InputManagerPlugin for handling axis_direction inputs
            .add_plugins(InputManagerPlugin::<Direction>::default())
            // Initialize the ActionState resource for axis_direction
            .init_resource::<ActionState<Direction>>()
            // Insert keybindings for axis_direction actions
            .insert_resource(Direction::keybindings())
            .init_resource::<Handedness>()
            // Add the setup system to be run when entering the Transform tool state
            .add_systems(OnEnter(ToolState::Transform), Self::setup)
            // Add the cleanup system to be run when exiting the Transform tool state
            .add_systems(OnExit(ToolState::Transform), Self::cleanup)
            // Add systems to be run during the Update phase, conditioned on being in the Transform tool state
            .add_systems(
                Update,
                (
                    // Emit transform events based on input actions
                    Self::emit_transform_event,
                    // Update the tool state based on keyboard input
                    Self::update_state_with_keyboard::<TransformToolState>,
                    // Update the Axis resource based on keyboard input
                    Self::update_resource_with_keyboard::<PrincipalAxis>,
                    // Update the TransformSpace resource based on keyboard input
                    Self::update_resource_with_keyboard::<TransformSpace>,
                    // Consume and process transform events if any are present
                    Self::consume_transform_event.run_if(on_event::<TransformEvent>),
                )
                    // Ensure these systems only run when in the Transform tool state
                    .run_if(in_state(ToolState::Transform)),
            )
            .add_systems(
                Update,
                Self::handle_transform_state_transition
                    .run_if(on_event::<StateTransitionEvent<TransformToolState>>)
            );
    }
}
