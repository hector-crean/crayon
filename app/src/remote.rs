use bevy::prelude::*;
use bevy::reflect::{Reflectable};
use bevy::remote::{BrpError, RemotePlugin};
use serde::Deserialize;
use serde_json::Value;

pub struct CrayonRemotePlugin<T: Event + Reflectable + for<'de> Deserialize<'de>>(pub T);

impl<T: Event + Reflectable + for<'de> Deserialize<'de>> Plugin for CrayonRemotePlugin<T> {
    fn build(&self, app: &mut App) {
        app
        .add_event::<T>()
        .register_type::<T>()
        .add_plugins(
            // `default` adds all of the built-in methods, while `with_method` extends them
            RemotePlugin::default()
                // .with_method(BRP_SEND_EVENT_METHOD, process_remote_send_event_request::<T>)
                // ... more methods can be added by chaining `with_method`
        );
    }
}

// Add to builtin_methods.rs constants
pub const BRP_SEND_EVENT_METHOD: &str = "crayon_event";

#[derive(Deserialize)]
struct BrpSendEventParams<T> {
    event_data: T,
}

pub fn process_remote_send_event_request<T: Event + for<'de> Deserialize<'de>>(
    In(params): In<Option<Value>>, 
) -> Result<T, BrpError> {
    match params {
        Some(params) => {
            let params: BrpSendEventParams<T> = serde_json::from_value(params).map_err(|e| BrpError::component_error(format!("error parsing event parameters: {}", e)))?;
            
            Ok(params.event_data)
        }
        None => Err(BrpError::component_error("error parsing event parameters"))
    }
    
}


