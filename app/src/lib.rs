pub mod plugins;
pub mod preprocess;

use bevy::app::App;
use bevy::prelude::*;
use crossbeam_channel::{unbounded, Receiver, Sender};
use dendron_types::events::DendronEvent;
use plugins::message_bridge::MsgBridgePlugin;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen]
pub struct DendronApp {
    app: App,
    tx: Sender<DendronEvent>,
    rx: Receiver<DendronEvent>,
}

#[wasm_bindgen]
impl DendronApp {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let (tx, rx) = unbounded::<DendronEvent>();

        let mut app = App::new();

        app.add_plugins((DefaultPlugins, MsgBridgePlugin::new(tx.clone(), rx.clone())));

        Self { app, tx, rx }
    }
    #[wasm_bindgen]
    pub fn run(&mut self) -> Result<(), JsValue> {
        self.app.run();
        Ok(())
    }

    #[wasm_bindgen]
    pub fn queue_cmd(&mut self, js_input_events: JsValue) -> Result<JsValue, JsValue> {
        let maybe_input_events: Result<Vec<DendronEvent>, _> =
            serde_wasm_bindgen::from_value(js_input_events);

        if let Ok(input_events) = maybe_input_events {
            for input_event in input_events {
                self.tx
                    .send(input_event)
                    .expect("Failed to send event through Crossbeam channel");
            }

            self.app.update();
        }

        let mut output_events = Vec::new();

        while let Ok(event) = self.rx.try_recv() {
            output_events.push(event);
        }

        serde_wasm_bindgen::to_value(&output_events).map_err(Into::into)
    }
}
