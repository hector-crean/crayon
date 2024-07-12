pub mod materials;
pub mod plugins;
pub mod preprocess;
pub mod state;

use bevy::app::App;
use bevy::prelude::*;
use bevy_camera::{
    pan_orbit_camera::{OrbitCameraController, OrbitCameraControllerPlugin},
    CameraMode,
};
use bevy_mod_picking::{
    debug::DebugPickingPlugin, prelude::low_latency_window_plugin, DefaultPickingPlugins,
};
use crayon_types::events::DrayonEvent;
use crossbeam_channel::{unbounded, Receiver, Sender};
use materials::lens_material::LensMaterialPlugin;
use plugins::message_bridge::MsgBridgePlugin;
use state::camera::CameraModeImpl;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[derive(Component)]
pub struct MainCamera;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DefaultPlugins.set(AssetPlugin {
                watch_for_changes_override: Some(true),
                ..Default::default()
            }),
            LensMaterialPlugin,
            OrbitCameraControllerPlugin::<CameraModeImpl>::default(),
            DefaultPickingPlugins
                .build()
                .disable::<DebugPickingPlugin>(),
        ))
        .add_systems(
            Startup,
            (LensMaterialPlugin::example_setup, Self::setup_camera),
        );
    }
}

impl GamePlugin {
    fn setup_camera(mut commands: Commands) {
        commands.spawn((
            Camera3dBundle {
                transform: Transform::from_xyz(0., 5.0, 0.).looking_at(Vec3::ZERO, Vec3::Y),
                // projection: Projection::Orthographic(OrthographicProjection::default()),
                ..default()
            },
            MainCamera,
            OrbitCameraController::default(),
        ));
    }
}

#[wasm_bindgen]
pub struct DrayonApp {
    app: App,
    tx: Sender<DrayonEvent>,
    rx: Receiver<DrayonEvent>,
}

#[wasm_bindgen]
impl DrayonApp {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let (tx, rx) = unbounded::<DrayonEvent>();

        let mut app = App::new();

        app.add_plugins((GamePlugin, MsgBridgePlugin::new(tx.clone(), rx.clone())));

        Self { app, tx, rx }
    }
    #[wasm_bindgen]
    pub fn run(&mut self) -> Result<(), JsValue> {
        self.app.run();
        Ok(())
    }

    #[wasm_bindgen]
    pub fn queue_cmd(&mut self, js_input_events: JsValue) -> Result<JsValue, JsValue> {
        let maybe_input_events: Result<Vec<DrayonEvent>, _> =
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
