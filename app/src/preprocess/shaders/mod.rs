use bevy::{asset::embedded_asset, prelude::*};
use itertools::Itertools;

pub(crate) const PREPROCESS_SHADER: &str =
    "embedded://crayon_app/preprocess/shaders/preprocess.wgsl";

pub(crate) fn load_preprocess_shaders(app: &mut App) {
    embedded_asset!(app, "preprocess.wgsl");

    // InternalShaders::load(
    //     app,
    //     &["embedded://crayon/preprocess/shaders/preprocess.wgsl"],
    // );
}

// #[derive(Default, Resource)]
// pub(crate) struct InternalShaders(Vec<Handle<Shader>>);

// impl InternalShaders {
//     pub(crate) fn load(app: &mut App, shaders: &[&'static str]) {
//         let mut shaders = shaders
//             .iter()
//             .map(|&shader| app.world_mut().resource_mut::<AssetServer>().load(shader))
//             .collect_vec();

//         let mut internal_shaders = app.world_mut().resource_mut::<InternalShaders>();
//         internal_shaders.0.append(&mut shaders);
//     }
// }
