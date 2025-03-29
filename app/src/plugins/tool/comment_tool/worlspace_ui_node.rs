use bevy::prelude::*;
use bevy::window::WindowResized;
use std::marker::PhantomData;
use bevy::log;



#[derive(Component)]
#[require(Node, ZIndex, BackgroundColor)]

pub struct WorldspaceUiNode<M: Component + Default> {
    pub world_position: Vec3,
    /// Size of the UI node in pixels
    pub ui_size: Vec2,
    _child: PhantomData<M>,
}

impl<M: Component + Default> WorldspaceUiNode<M> {
    const DEFAULT_UI_SIZE: Vec2 = Vec2::new(50.0, 50.0);

    /// Creates a new WorldspaceUiNode at the specified world position
    /// with default size
    pub fn new(world_position: Vec3) -> Self {
        Self { 
            world_position, 
            ui_size: Self::DEFAULT_UI_SIZE,
            _child: PhantomData 
        }
    }

    /// Creates a new WorldspaceUiNode with custom size
    pub fn with_size(world_position: Vec3, ui_size: Vec2) -> Self {
        Self { 
            world_position,
            ui_size,
            _child: PhantomData 
        }
    }

    fn setup(
        mut commands: Commands,
        mut query: Query<(Entity, &WorldspaceUiNode<M>, &Children, &mut Node, &mut BackgroundColor, &mut ZIndex), Added<WorldspaceUiNode<M>>>,
        camera_query: Query<
            (&Camera, &GlobalTransform),
            (With<Camera3d>, Changed<GlobalTransform>),
        >,
    ) {
        let Ok((camera, camera_transform)) = camera_query.get_single() else {
            return;
        };

        for (entity, worldspace_ui_node, children, mut node, mut bg_color, mut z_index) in &mut query {
            let screen_position = match camera
                .world_to_viewport(camera_transform, worldspace_ui_node.world_position)
            {
                Ok(screen_position) => screen_position,
                Err(_) => {
                    log::error!("Failed to convert world position to screen position.");
                    Vec2::ZERO
                },
            };

            // Modify the existing components in place
            *node = Node {
                position_type: PositionType::Absolute,
                top: Val::Px(screen_position.y),
                left: Val::Px(screen_position.x),
                padding: UiRect::all(Val::Px(0.0)),
                border: UiRect::all(Val::Px(0.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                ..default()
            };
            *bg_color = BackgroundColor(Color::NONE);
            z_index.0 = 0;

            // If there are no existing children, create the content node
            if children.is_empty() {
                commands.entity(entity).with_children(|parent| {
                    parent.spawn((
                        Node {
                            width: Val::Px(worldspace_ui_node.ui_size.x),
                            height: Val::Px(worldspace_ui_node.ui_size.y),
                            ..default()
                        },
                        M::default(),
                    ));
                });
            }
        }
    }

    fn update_node_positions(
        worldspace_button_query: &mut Query<(&WorldspaceUiNode<M>, &mut Node, &mut ZIndex)>,
        camera: &Camera,
        camera_transform: &GlobalTransform,
    ) -> bool {
        let mut any_updates = false;
        for (worldspace_ui_node, mut node, mut z_index) in worldspace_button_query.iter_mut() {
            let world_position = worldspace_ui_node.world_position;
            
            match camera.world_to_viewport(camera_transform, world_position) {
                Ok(screen_position) => {
                    node.left = Val::Px(screen_position.x);
                    node.top = Val::Px(screen_position.y);

                    let distance = camera_transform.translation().distance(world_position);
                    z_index.0 = -distance as i32;
                    any_updates = true;
                }
                Err(e) => {
                    log::warn!("Failed to update node position: {:?}", e);
                    // Could optionally hide the node when it can't be positioned
                    // node.display = Display::None;
                }
            }
        }
        any_updates
    }

    pub fn sync_worldspace_position_on_camera_change(
        mut worldspace_ui_node_query: Query<(&WorldspaceUiNode<M>, &mut Node, &mut ZIndex)>,
        camera_query: Query<(&Camera, &GlobalTransform), With<Camera3d>>,
    ) {
        let Ok((camera, camera_transform)) = camera_query.get_single() else {
            return;
        };

        Self::update_node_positions(&mut worldspace_ui_node_query, camera, camera_transform);
    }

    pub fn sync_worldspace_position_on_window_resize(
        mut worldspace_ui_node_query: Query<(&WorldspaceUiNode<M>, &mut Node, &mut ZIndex)>,
        camera_query: Query<
            (&Camera, &GlobalTransform),
            (With<Camera3d>, Changed<GlobalTransform>),
        >,
    ) {
        let Ok((camera, camera_transform)) = camera_query.get_single() else {
            return;
        };

        Self::update_node_positions(&mut worldspace_ui_node_query, camera, camera_transform);
    }

}

pub struct WorldspaceUiNodePlugin<M: Component + Default> {
    _child: PhantomData<M>,
}

impl<M: Component + Default> Default for WorldspaceUiNodePlugin<M> {
    fn default() -> Self {
        Self::new()
    }
}

impl<M: Component + Default>  WorldspaceUiNodePlugin<M> {
    pub fn new() -> Self {
        Self { _child: PhantomData }
    }
}

impl<M: Component + Default> Plugin for WorldspaceUiNodePlugin<M> {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            WorldspaceUiNode::<M>::setup,
            WorldspaceUiNode::<M>::sync_worldspace_position_on_camera_change,
            WorldspaceUiNode::<M>::sync_worldspace_position_on_window_resize
                .run_if(on_event::<WindowResized>),
        ));
    }
}

