use bevy::prelude::*;
use crayon_app::preprocess::PreprocessLensPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PreprocessLensPlugin))
        .run();
}
