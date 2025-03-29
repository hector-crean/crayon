
use bevy::{color::palettes, ecs::query::QueryData, prelude::*, ui::FocusPolicy, window::WindowResized};
use strum::{IntoEnumIterator};

use crate::plugins::tool::comment_tool::worlspace_ui_node::{WorldspaceUiNode, WorldspaceUiNodePlugin};


pub trait UiComponent {
    type Query<'a>;
    type QueryMut<'a>;
}



// Components instead of bundles
#[derive(Component, Default)]
pub struct RadialMenu {
    center: Vec2,
    radius: f32,
}

impl RadialMenu {
    // Size of the menu relative to radius (2.5 means the container is 2.5x larger than the radius * 2)
    pub const SIZE_MULTIPLIER: f32 = 1.;
    // Base radius of the menu
    pub const BASE_RADIUS: f32 = 150.0;
    // How far out the items are positioned (as a percentage of container size)
    pub const ITEM_RADIUS_PERCENT: f32 = 1.;
    // Padding around menu items
    pub const ITEM_PADDING: f32 = 8.0;


    fn center_percentage_coordinates(&self) -> (f32, f32) {
        (self.center.x * 100.0, self.center.y * 100.0)
    }
}



pub trait RadialMenuItemDataLike: Send + Sync + std::fmt::Debug + Clone + 'static + Into<RadialItemData> {}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct RadialItemData {
    pub icon: String,
    pub color: Color,
    pub label: String,
}


#[derive(Component, Debug)]
#[require(Interaction, Node, BackgroundColor, BorderColor, Button)]
pub struct RadialMenuItem {
    item: RadialItemData,
    angle: f32,
    selected: bool,
}


#[derive(QueryData)]
#[query_data(derive(Debug))]
pub struct RadialMenuItemQuery<'a> {
    pub item: &'a RadialMenuItem,
    pub interaction: &'a Interaction,
    pub node: &'a Node,
    pub bg_color: &'a BackgroundColor,
    pub border_color: &'a BorderColor,
    pub button: &'a Button,
}



#[derive(QueryData)]
#[query_data(mutable, derive(Debug))]
pub struct RadialMenuItemQueryMut<'a> {
    pub item: &'a mut RadialMenuItem,
    pub interaction: &'a mut Interaction,
    pub node: &'a mut Node,
    pub bg_color: &'a mut BackgroundColor,
    pub border_color: &'a mut BorderColor,
    pub button: &'a mut Button,
}




impl RadialMenuItem {
    pub const RADIUS: f32 = 20.0;

    fn calculate_position(index: usize, total_items: usize, container_size: f32) -> (f32, Vec2) {
        let angle = (index as f32 / total_items as f32) * 2.0 * std::f32::consts::PI;
        let (sin, cos) = angle.sin_cos();
        
        let radius = container_size * RadialMenu::ITEM_RADIUS_PERCENT / 2.0;
        let position = Vec2::new(container_size / 2.0, container_size / 2.0) 
            + Vec2::new(cos * radius, sin * radius) 
            + Vec2::new(-Self::RADIUS / 2.0, -Self::RADIUS / 2.0);
        
        (angle, position)
    }

    fn spawn(parent: &mut ChildBuilder, item: RadialItemData, index: usize, total_items: usize, container_size: f32) {
        let (angle, position) = Self::calculate_position(index, total_items, container_size);
        parent.spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(position.x),
                top: Val::Px(position.y),
                width: Val::Px(Self::RADIUS),
                height: Val::Px(Self::RADIUS),
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },

        )).with_children(|parent| {
            parent.spawn((
                RadialMenuItem {
                    item: item.clone(),
                    angle,
                    selected: false,
                },
                Node {
                    width: Val::Auto,
                    height: Val::Auto,
                    padding: UiRect::all(Val::Px(4.0)),
                    // min_width: Val::Px(Self::RADIUS),
                    // min_height: Val::Px(Self::RADIUS),
                    ..default()
                },
                BackgroundColor(item.color),
                Button,
                BorderColor(palettes::tailwind::CYAN_400.into()),
                BorderRadius::all(Val::Px(8.0)),
            ))
            .with_children(|parent| {
                parent.spawn((
                    Text::new(item.label.clone()),
                    TextFont {
                        font_size: 12.0,
                        ..default()
                    },
                ));
            });
        });
    }
}

#[derive(Component)]
pub struct MenuBackground;

#[derive(Event)]
pub struct RadialMenuSelection(pub RadialItemData);

#[derive(Debug, Clone)]
pub enum RadialMenuPosition {
    WorldSpace(Vec3),
    ScreenCenter,
}

#[derive(Event)]
pub struct OpenRadialMenu {
    pub items: Vec<RadialItemData>,
    pub position: RadialMenuPosition,
}

#[derive(Event)]
pub struct CloseRadialMenu;

#[derive(Default)]
pub struct RadialMenuPlugin;

// Add states to track menu visibility
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum RadialMenuState {
    #[default]
    Hidden,
    Visible,
}

impl Plugin for RadialMenuPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(WorldspaceUiNodePlugin::<RadialMenu>::new())
            .init_state::<RadialMenuState>()
            .add_event::<RadialMenuSelection>()
            .add_event::<OpenRadialMenu>()
            .add_event::<CloseRadialMenu>()
            .add_systems(Update, handle_menu_events)
            .add_systems(
                Update,
                (
                    update_radial_menu_items,
                    handle_radial_menu_selection,
                ).run_if(in_state(RadialMenuState::Visible))
            )
            .add_systems(OnExit(RadialMenuState::Visible), despawn_radial_menu)
            .add_systems(Update, handle_window_resize.run_if(in_state(RadialMenuState::Visible)));
    }
}

// New component to help with despawning
#[derive(Component)]
struct RadialMenuScreen;

// New system to handle menu events
fn handle_menu_events(
    mut commands: Commands,
    mut open_events: EventReader<OpenRadialMenu>,
    mut close_events: EventReader<CloseRadialMenu>,
    mut menu_state: ResMut<NextState<RadialMenuState>>,
    query: Query<Entity, With<RadialMenuScreen>>,
) {
    for event in open_events.read() {
        // First despawn any existing menu
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }
        
        spawn_radial_menu(&mut commands, event.position.clone(), &event.items);
        menu_state.set(RadialMenuState::Visible);
    }

    for _ in close_events.read() {
        menu_state.set(RadialMenuState::Hidden);
    }
}

// Update spawn function to take items directly
fn spawn_radial_menu(
    commands: &mut Commands,
    position: RadialMenuPosition,
    items: &[RadialItemData],
) {
    let total_items = items.len();
    let container_size = RadialMenu::BASE_RADIUS * 2.0 * RadialMenu::SIZE_MULTIPLIER;
    let menu = RadialMenu {
        center: Vec2::ZERO,
        radius: RadialMenu::BASE_RADIUS,
    };

    let mut entity_commands = commands.spawn((
        menu,
        RadialMenuScreen,
        MenuBackground,
        FocusPolicy::Pass,
    ));

    // Add positioning component based on the mode
    match position {
        RadialMenuPosition::WorldSpace(world_pos) => {
            entity_commands.insert(WorldspaceUiNode::<RadialMenu>::with_size(
                world_pos,
                Vec2::new(container_size, container_size),
            ));
        }
        RadialMenuPosition::ScreenCenter => {
            entity_commands.insert(
            (
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(50.0),
                    top: Val::Percent(50.0),
                    width: Val::Px(container_size),
                    height: Val::Px(container_size),
                    ..default()
                },
            )
            );
        }
    };

    entity_commands.with_children(|parent| {
        parent.spawn((
            Node {
                width: Val::Px(container_size),
                height: Val::Px(container_size),
                position_type: PositionType::Absolute,
                left: Val::Px(-container_size / 2.0),
                top: Val::Px(-container_size / 2.0),
                
                ..default()
            },
            FocusPolicy::Pass,
            BorderRadius::all(Val::Percent(100.0)),
            BackgroundColor(Color::rgba(0.0, 0.0, 0.0, 0.0)), 
        )).with_children(|parent| {
            // Spawn individual menu items
            for (i, item) in items.iter().enumerate() {
                RadialMenuItem::spawn(parent, item.clone(), i, total_items, container_size);
            }
        });
    });
}



fn update_radial_menu_items(
    mut items: Query<RadialMenuItemQueryMut>,
    mut selected_event: EventWriter<RadialMenuSelection>,
    mut menu_state: ResMut<NextState<RadialMenuState>>,
) {
    for RadialMenuItemQueryMutItem { 
        item, 
        interaction, 
        node, 
        mut bg_color, 
        mut border_color,
        button
    } in &mut items {
        match (*interaction, item.selected) {
            (Interaction::Pressed, _) => {
                *bg_color = item.item.color.with_alpha(0.8).into();
                border_color.0 = Color::rgb(0.0, 0.5, 1.0); // Blue highlight for pressed state
                
                selected_event.send(RadialMenuSelection(item.item.clone()));
                menu_state.set(RadialMenuState::Hidden);
            },
            (Interaction::Hovered, true) => {
                *bg_color = item.item.color.with_alpha(1.0).into();
                border_color.0 = Color::WHITE;
            },
            (_, true) => {
                *bg_color = item.item.color.with_alpha(0.9).into();
                border_color.0 = Color::rgb(0.8, 0.8, 0.8);
            },
            _ => {
                *bg_color = item.item.color.with_alpha(0.7).into();
                border_color.0 = Color::rgb(0.2, 0.2, 0.2);
            },
        }
    }
}




// New despawn system
fn despawn_radial_menu(
    mut commands: Commands,
    query: Query<Entity, With<RadialMenuScreen>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

// Update selection handler to change state
pub fn handle_radial_menu_selection(
    commands: Commands,
    query: Query<(&RadialMenuItem, &Interaction), Changed<Interaction>>,
    mut selected_event: EventWriter<RadialMenuSelection>,
    mut menu_state: ResMut<NextState<RadialMenuState>>,
) {
    for (item, interaction) in query.iter() {
        if *interaction == Interaction::Pressed && item.selected {
            selected_event.send(RadialMenuSelection(item.item.clone()));
            menu_state.set(RadialMenuState::Hidden);
        }
    }
}

fn handle_window_resize(
    mut menu_query: Query<(&RadialMenu, &mut Node)>,
    mut resize_events: EventReader<WindowResized>,
) {
    for _event in resize_events.read() {

        for (menu, mut node) in menu_query.iter_mut() {
            let (left_percent, top_percent) = menu.center_percentage_coordinates();
            node.left = Val::Percent(left_percent);
            node.top = Val::Percent(top_percent);
        }
    }
}