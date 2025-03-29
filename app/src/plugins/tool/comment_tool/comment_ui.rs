use bevy::{color::palettes, prelude::*};

#[derive(Component, Default)]
pub enum CommentUi {
    #[default]
    Moving,
    Editing,
    Viewing,
}

impl CommentUi {
   
    pub fn view(state: &Self, asset_server: &AssetServer) -> (Node, BackgroundColor, BorderRadius, ImageNode) {

        let node = Node {
            width: Val::Px(24.0),
            height: Val::Px(24.0),
            padding: UiRect::all(Val::Px(4.0)),
            display: Display::Flex,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(4.0),
            ..default()
        };

        (
            node.clone(),
            BackgroundColor(Color::Srgba(palettes::tailwind::RED_500)),
            BorderRadius::new(Val::Px(4.0), Val::Px(4.0), Val::Px(4.0), Val::Px(4.0)),
            ImageNode { 
                image: asset_server.load("icons/message-circle_48px.png"),
                ..default()
            }
        )
    }
    fn handle_added(
        mut commands: Commands,
        query: Query<(Entity, &CommentUi), Added<CommentUi>>,
        asset_server: Res<AssetServer>,
    ) {
        for (entity, comment_ui) in query.iter() {
            info!("CommentUi added");
            let comment_ui_bundle = CommentUi::view(comment_ui, &asset_server);
            commands.entity(entity).insert(comment_ui_bundle);
        }
    }
    fn update(
        commands: Commands,
        query: Query<(Entity, &CommentUi), (Added<CommentUi>, Changed<CommentUi>)>,
    ) {
        for (entity, comment_ui) in query.iter() {
         
        }
    }

}


pub struct CommentUiPlugin;

impl Plugin for CommentUiPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, CommentUi::handle_added);
    }
}