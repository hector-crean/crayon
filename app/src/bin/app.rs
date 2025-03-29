

use bevy::{app::{App, AppExit}};

use crayon_app::app_plugin::AppPlugin;

pub fn main() -> AppExit {
   App::new().add_plugins(AppPlugin).run()
}



