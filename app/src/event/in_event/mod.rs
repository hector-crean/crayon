use bevy::prelude::*;
use ts_rs::TS;

use crate::plugins::tool::{ToolState};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Event, TS)]
#[serde(tag = "type", content = "data")]
pub enum CrayonInEvent {
    ChangeTool(ToolState),
    ExitApp,
}




impl CrayonInEvent {
    pub fn handle(mut event_rdr: EventReader<CrayonInEvent>, mut next_tool_state: ResMut<NextState<ToolState>>, mut app_exit: EventWriter<AppExit>) {
        for ev in event_rdr.read(){
            info!("event: {:?}", ev);
            match ev {
                CrayonInEvent::ChangeTool(tool_label) => {
                    next_tool_state.set( *tool_label);
                }
                CrayonInEvent::ExitApp => {
                    info!("AppExit event received:");
                    app_exit.send(AppExit::Success);
                }
               
            }
        }
    }
}






