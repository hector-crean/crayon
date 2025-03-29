pub mod comment;
pub mod comment_ui;
pub mod worlspace_ui_node;
use bevy::picking::events::{Click, Pointer};
use bevy::prelude::*;
use comment_ui::{CommentUi, CommentUiPlugin};
use worlspace_ui_node::{WorldspaceUiNode, WorldspaceUiNodePlugin};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::ToolState;

pub struct CommentToolPlugin;

impl Plugin for CommentToolPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((WorldspaceUiNodePlugin::<CommentUi>::new(), CommentUiPlugin))
            .insert_resource(SelectedComment::default())
            .init_state::<CommentToolState>()
            .add_event::<CommentToolEvent>()
            .add_systems(
                Update,
                (
                    Self::handle_placing.run_if(in_state(CommentToolState::Placing)),
                    Self::handle_editing.run_if(in_state(CommentToolState::Editing)),
                    Self::handle_viewing.run_if(in_state(CommentToolState::Viewing)),
                ),
            );
    }
}

#[derive(
    Debug, Clone, Copy, Eq, PartialEq, Hash, SubStates, Component, Default, Serialize, Deserialize,
)]
#[source(ToolState = ToolState::Comment)]
pub enum CommentToolState {
    #[default]
    Placing, // User is choosing where to place the comment (was SelectingAnchor)
    Editing,
    Viewing, // User is viewing comment thread(s)
}

#[derive(Event)]
pub enum CommentToolEvent {
    SelectComment { comment_id: Uuid },
    RemoveComment { comment_id: Uuid },
    ChangeCommentMode { state: CommentToolState },
}

#[derive(Default, Resource)]
pub struct SelectedComment {
    comment_id: Option<Entity>,
}




impl CommentToolPlugin {
    fn handle_placing(
        mut commands: Commands,
        mut pointer_events: EventReader<Pointer<Click>>,
        next_state: ResMut<NextState<CommentToolState>>,
    ) {
        for event in pointer_events.read() {
            if let Some(position) = event.hit.position {
                commands.spawn((
                    WorldspaceUiNode::<CommentUi>::new(position),
                ));
                // next_state.set(CommentToolState::Editing);
            }
        }
    }

    fn handle_editing(next_state: ResMut<NextState<CommentToolState>>) {
        // next_state.set(CommentToolState::Viewing);
    }

    fn handle_viewing(next_state: ResMut<NextState<CommentToolState>>) {
        // next_state.set(CommentToolState::Placing);
    }

  
}
