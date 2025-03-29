use bevy::prelude::*;
use crate::plugins::tool::markup_tool::MarkupToolState;




/// The top-level states for the polyline drawing tool, subordinate to `MarkupToolState::Polyline`.
#[derive(Default, SubStates, Debug, Clone, Eq, PartialEq, Hash)]
#[source(MarkupToolState = MarkupToolState::Rectangle)]
pub enum RectangleMarkupState {
    /// Waiting for the first click to place the initial point of a new polyline.
    #[default]
    AwaitingFirstClick,

    /// After placing the first point, we keep adding more points with each click until double-click finishes.
    Drafting,

    /// The user has double-clicked to finish the polyline. We might do post-processing or a small delay here.
    Finished,
}


/// The main plugin that handles creating and updating polylines based on user clicks.
pub struct RectangleMarkupPlugin;

impl Plugin for RectangleMarkupPlugin {
    fn build(&self, app: &mut App) {
        app
            // We want to use these sub-states in the `MarkupToolState::Polyline` context.
            .init_state::<RectangleMarkupState>()
            // Simple plugin setup
            .add_systems(Startup, Self::setup);
            // Systems for the main update stage
           
    }
}

impl RectangleMarkupPlugin {
    /// Called once at plugin setup time.
    fn setup() {
        info!("RectangleMarkupPlugin: setup complete");
    }

    /// Logs all state transitions for debugging.
    fn debug_state_transition(
        mut state_reader: EventReader<StateTransitionEvent<RectangleMarkupState>>,
    ) {
        for event in state_reader.read() {
            info!(
                "RectangleMarkupState changed from {:?} to {:?}",
                event.exited, event.entered
            );
        }
    }


}