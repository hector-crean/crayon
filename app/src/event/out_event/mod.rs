use bevy::prelude::*;
use ts_rs::TS;

use crate::plugins::tool::{ToolState};


#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Event, TS)]
#[serde(tag = "type", content = "data")]
pub enum CrayonOutEvent {
    ToolChanged(ToolState),
}

impl CrayonOutEvent {
    pub fn handle(mut event_rdr: EventReader<CrayonOutEvent>) {
        for ev in event_rdr.read(){
            info!("event: {:?}", ev);
            match ev {
                CrayonOutEvent::ToolChanged(tool_state) => {
                    // let _  = GLOBAL_EVENT_CHANNEL.send(ev.clone());
                }
            }
        }
    }
}



