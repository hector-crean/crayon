use bevy::prelude::*;
use dendron_app::preprocess::PreprocessLensPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PreprocessLensPlugin))
        .run();
}
