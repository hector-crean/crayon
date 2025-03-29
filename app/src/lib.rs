pub mod materials;
pub mod plugins;
pub mod state;
pub mod app_plugin;
pub mod event;
pub mod event_channel;
pub mod transaction;
pub mod remote;
pub mod meshes;
pub mod camera;

use bevy::{app::{App, Update}, log::{error, info}};
use event::CrayonInEvent;
use event_channel::GLOBAL_EVENT_CHANNEL;
use app_plugin::AppPlugin;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use std::sync::atomic::{AtomicBool, Ordering};

// static IS_APP_RUNNING: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));

// macro_rules! generate_typescript_static {
//     ($($type:ty),* $(,)?) => {
//         paste::paste! {
//             $(
//                 pub static [<$type:snake:upper _TS>]: Lazy<String> = Lazy::new(|| {
//                     let conf = ts::Typescript::new();
//                     ts::export::<$type>(&conf)
//                         .expect(&format!("Failed to generate TypeScript for {}", stringify!($type)))
//                 });
//             )*
//         }
//     };
// }

// // Usage example:
// generate_typescript_static! {
//     CrayonEvent,
// }


#[wasm_bindgen(typescript_custom_section)]
const CRAYON_TYPES_TS: &'static str = include_str!("./crayon_types.d.ts");


/// @param {CrayonEvent} message
#[wasm_bindgen(skip_typescript, js_name = sendCrayonEvent)]
pub fn send_crayon_event(message: JsValue) -> Result<(), JsValue> {
    info!("Sending message to bevy: {:?}", message);

    let event = serde_wasm_bindgen::from_value::<CrayonInEvent>(message)
    .map_err(|e| JsValue::from_str(&format!("Failed to deserialize event: {}", e)))?;

    GLOBAL_EVENT_CHANNEL.send(event).map_err(|e| {
        error!("Failed to send event: {}", e);
        JsValue::from_str("Failed to send event")
    })?;

    Ok(())
}

#[wasm_bindgen(skip_typescript, js_name = drainEventQueue)]
pub fn drain_event_queue() -> Result<JsValue, JsValue> {
    let mut events = Vec::new();
    
    loop {
        match GLOBAL_EVENT_CHANNEL.try_receive() {
            Ok(Some(event)) => {
                events.push(event);
            },
            Ok(None) => {
                // No more events in the channel, break the loop
                break;
            },
            Err(e) => {
                return Err(JsValue::from_str(&format!("Error receiving event: {}", e)));
            }
        }
    }
    
    // Serialize the events vector to a JsValue
    serde_wasm_bindgen::to_value(&events)
        .map_err(|e| JsValue::from_str(&format!("Failed to serialize events: {}", e)))
}

// Add a static flag to track if the app is already running
static APP_RUNNING: AtomicBool = AtomicBool::new(false);

#[wasm_bindgen(js_name = runApp)]
pub fn run_app() -> Result<(), JsValue> {
    // Check if app is already running
    if APP_RUNNING.load(Ordering::SeqCst) {
        return Err(JsValue::from_str("App is already running"));
    }

    // Set app as running
    APP_RUNNING.store(true, Ordering::SeqCst);

    // Create and run the app with proper cleanup
    let mut app = App::new();
    app.add_plugins(AppPlugin);
    

    
    app.run();
    
    // Reset running state when complete
    APP_RUNNING.store(false, Ordering::SeqCst);
    
    Ok(())
}

// Add a cleanup function
#[wasm_bindgen(js_name = cleanup)]
pub fn cleanup() {
    APP_RUNNING.store(false, Ordering::SeqCst);
}







