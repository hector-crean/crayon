use bevy::{color::palettes, prelude::*, ui::widget::NodeImageMode};
use strum::IntoEnumIterator;
use super::{ToolState};
use strum::EnumProperty; 


#[derive(Component)]
pub struct Toolbar;

impl Toolbar {
    // Define button colors as constants
    const NORMAL_BUTTON: Color = Color::NONE;
    const HOVERED_BUTTON: Color = Color::rgb(0.475, 0.475, 0.475);
    const PRESSED_BUTTON: Color = Color::rgb(0.592, 0.694, 0.835);
    const SELECTED_BUTTON: Color = Color::rgb(0.4, 0.6, 0.8);
    const CONTAINER_COLOR: Color = Color::rgb(0.235, 0.235, 0.235);

    pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands
            .spawn((
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    height: Val::Auto,
                    bottom: Val::Px(8.0),
                    left: Val::Px(0.0),
                    right: Val::Px(0.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                Toolbar,
            ))
            .with_children(|parent| {
                parent
                    .spawn((
                        Node {
                            width: Val::Auto,
                            height: Val::Auto,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            column_gap: Val::Px(8.0),
                            padding: UiRect::all(Val::Px(8.)),
                            ..default()
                        },
                        BackgroundColor(Self::CONTAINER_COLOR),
                        BorderRadius::new(
                            // top left
                            Val::Px(4.),
                            // top right
                            Val::Px(4.),
                            // bottom right
                            Val::Px(4.),
                            // bottom left
                            Val::Px(4.),
                        ),
                    ))
                    .with_children(|toolbar| {
                        for tool_state in ToolState::iter() {
                            let icon_path = "icons/move.png";
                            
                            toolbar
                                .spawn((
                                    Button,
                                    Node {
                                        width: Val::Px(32.0),
                                        height: Val::Px(32.0),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        display: Display::Flex,
                                        flex_direction: FlexDirection::Column,
                                        column_gap: Val::Px(4.0),
                                        ..default()
                                    },
                                    tool_state,
                                    BackgroundColor(Self::NORMAL_BUTTON),
                                    BorderColor(Color::BLACK),
                                    BorderRadius::new(
                                        // top left
                                        Val::Px(4.),
                                        // top right
                                        Val::Px(4.),
                                        // bottom right
                                        Val::Px(4.),
                                        // bottom left
                                        Val::Px(4.),
                                    ),
                                ))
                                .with_children(|button| {
                                    let tool_label_str: &'static str = tool_state.into();
                                    let tool_icon = tool_state.get_str("icon").unwrap();

                                    button.spawn((
                                        ImageNode {
                                            image: asset_server.load(tool_icon),
                                            color: Color::WHITE,
                                            flip_x: false,
                                            flip_y: false,
                                            image_mode: NodeImageMode::Auto,
                                            ..default()
                                        },
                                        Node {
                                            width: Val::Px(24.0),
                                            height: Val::Px(24.0),
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            display: Display::Flex,
                                            flex_direction: FlexDirection::Column,
                                            column_gap: Val::Px(4.0),
                                            ..default()
                                        }
                                    ));
                                });
                        }
                    });
            });
    }

    pub fn handle_interaction_changes(
        mut next_tool_state: ResMut<NextState<ToolState>>,
        tool_state: Res<State<ToolState>>,
        mut query: Query<
            (&Interaction, &mut BackgroundColor, &mut BorderColor, &ToolState),
            (Changed<Interaction>, With<Button>),
        >,
    ) {
        for (interaction, mut color, mut border_color, tool) in &mut query {
            let active_tool_label: &'static str = (*tool_state.get()).into();
            let tool_label: &'static str = (*tool).into();
            let tool_active = active_tool_label == tool_label;

            match (*interaction, tool_active) {
                (Interaction::Pressed, _) => {
                    next_tool_state.set(*tool);
                    *color = Self::PRESSED_BUTTON.into();
                    border_color.0 = palettes::tailwind::RED_300.into();
                }
                (Interaction::Hovered, true) | (Interaction::None, true) => {
                    *color = Self::SELECTED_BUTTON.into();
                    border_color.0 = Color::WHITE;
                }
                (Interaction::Hovered, false) => {
                    *color = Self::HOVERED_BUTTON.into();
                    border_color.0 = Color::WHITE;
                }
                (Interaction::None, false) => {
                    *color = Self::NORMAL_BUTTON.into();
                    border_color.0 = Color::BLACK;
                }
            }
        }
    }

    pub fn update_tool_state(
        tool_state: Res<State<ToolState>>,
        mut query: Query<(&mut BackgroundColor, &mut BorderColor, &ToolState), With<Button>>,
    ) {
        for (mut color, mut border_color, tool) in &mut query {
            let active_tool_label: &'static str = (*tool_state.get()).into();
            let tool_label: &'static str = (*tool).into();
            let tool_active = active_tool_label == tool_label;

            match tool_active {
                true => {
                    *color = Self::SELECTED_BUTTON.into();
                    border_color.0 = palettes::tailwind::GREEN_300.into();
                }
                false => {
                    *color = Self::NORMAL_BUTTON.into();
                    border_color.0 = Color::BLACK;
                }
            }
        }
    }
}

pub struct ToolbarPlugin;

impl Plugin for ToolbarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Toolbar::setup)
            .add_systems(
                Update,
                (
                    Toolbar::handle_interaction_changes,
                    Toolbar::update_tool_state.run_if(on_event::<StateTransitionEvent<ToolState>>),
                ),
            );
    }
}