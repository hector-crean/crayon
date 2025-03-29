pub mod markup;

use bevy::prelude::*;
use markup::{polyline::{PolylineMarkupPlugin, PolylineMarkupState}, rectangle::{RectangleMarkupPlugin, RectangleMarkupState}};
use strum::{EnumIter, IntoEnumIterator};

use crate::plugins::radial_menu::{OpenRadialMenu, RadialItemData, RadialMenuPosition};

use super::ToolState;

#[derive(Default, SubStates, Debug, Clone, Eq, PartialEq, Hash, EnumIter)]
#[source(ToolState = ToolState::Markup)]
pub enum MarkupToolState {
    #[default]
    Polyline,   
    Rectangle,   
    // Point,       
    // Polygon,     
    // Circle,      
    // MultiPoint,  
}

impl From<MarkupToolState> for RadialItemData {
    fn from(state: MarkupToolState) -> Self {
        match state {
            MarkupToolState::Polyline => RadialItemData {
                label: "Polyline".to_string(),
                icon: "icons/markup_tool.png".to_string(),
                color: Color::WHITE,    
            },
            MarkupToolState::Rectangle => RadialItemData {
                label: "Rectangle".to_string(),
                icon: "icons/markup_tool.png".to_string(),
                color: Color::WHITE,
            },
        }
    }
}

pub struct MarkupToolPlugin;

impl Plugin for MarkupToolPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<MarkupToolState>()
            .add_sub_state::<PolylineMarkupState>()
            .add_sub_state::<RectangleMarkupState>()
            .add_plugins((PolylineMarkupPlugin, RectangleMarkupPlugin))
            .add_systems(
                Update,
                Self::handle_state_transition
                    .run_if(on_event::<StateTransitionEvent<MarkupToolState>>),
            ).add_systems(OnEnter(ToolState::Markup), Self::open_radial_menu);
    }
}

impl MarkupToolPlugin {
    fn handle_state_transition(
        mut state_reader: EventReader<StateTransitionEvent<MarkupToolState>>,
    ) {
        for event in state_reader.read() {
            info!("MarkupTool state changed from {:?} to {:?}", event.exited, event.entered);
        }
    }

    fn open_radial_menu(mut radial_menu_rdr: EventWriter<OpenRadialMenu>) {
        radial_menu_rdr.send(OpenRadialMenu {
            items: MarkupToolState::iter().map(|state| state.into()).collect(),
            position: RadialMenuPosition::ScreenCenter,
        });
    }
}
