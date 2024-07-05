use crate::util::InternalShaders;
use bevy::{asset::embedded_asset, prelude::*};

pub const DEFAULT_SHADER: &str = "embedded://bevy_voxel_plugin/render/shaders/render/default.wgsl";

pub(crate) fn load_terrain_shaders(app: &mut App) {
    embedded_asset!(app, "types.wgsl");

    InternalShaders::load(
        app,
        &["embedded://bevy_voxel_plugin/render/shaders/types.wgsl"],
    );
}
