
use bevy::{picking::backend::HitData, prelude::PointerButton, reflect::Reflect};

use bevy::prelude::*;

#[derive(Resource)]
pub struct ClickTimer(f32);

impl Default for ClickTimer {
    fn default() -> Self {
        ClickTimer(0.0)
    }
}

#[derive(Clone, PartialEq, Debug, Reflect)]
pub struct DoubleClick {
    /// Pointer button pressed and lifted to trigger this event.
    pub button: PointerButton,
    /// Information about the picking intersection.
    pub hit: HitData,
    /// Duration between the first and second click
    pub duration: f32,
}


pub struct DoubleClickPlugin;

impl Plugin for DoubleClickPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClickTimer::default())
        .add_event::<Pointer<DoubleClick>>()
        .add_systems(Update, Self::double_click);
    }
   
}

impl DoubleClickPlugin {
    fn double_click(
        mut click_reader: EventReader<Pointer<Click>>, 
        mut click_timer: ResMut<ClickTimer>,
        time: Res<Time>,
        mut double_click_writer: EventWriter<Pointer<DoubleClick>>,
    ) {
        for click in click_reader.read() {
            let threshold = 0.5;
            let current_time = time.elapsed_secs();
            let click_delta = current_time - click_timer.0;
            
            if click_delta < threshold {
                // Handle double click
                let event = DoubleClick {
                    button: click.button,
                    hit: click.hit.clone(),
                    duration: click_delta,
                };

                double_click_writer.send(Pointer::<DoubleClick>::new(click.target, click.pointer_id, click.pointer_location.clone(), event));
            }
            
            // Update the click timer with the current time
            click_timer.0 = current_time;
        }
    }
}

