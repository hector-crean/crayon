//! A shader and a material that uses it.

use bevy::{
    asset::embedded_asset,
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
};

pub(crate) const LENS_MATERIAL_SHADER: &str =
    "embedded://crayon_app/materials/lens_material/lens.wgsl";

pub struct LensMaterialPlugin;

impl Plugin for LensMaterialPlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "lens.wgsl");
        app.add_plugins(MaterialPlugin::<CustomMaterial>::default());
    }
}

impl LensMaterialPlugin {
    pub fn example_setup(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<CustomMaterial>>,
        asset_server: Res<AssetServer>,
    ) {
        let amsler_grid = asset_server.load("img/amsler_grid.png");
        // cube
        commands.spawn(MaterialMeshBundle {
            mesh: meshes.add(Plane3d::default()),
            transform: Transform::from_xyz(0.0, 0., 0.0),
            material: materials.add(CustomMaterial {
                color: LinearRgba::BLUE,
                color_texture: Some(amsler_grid),
                alpha_mode: AlphaMode::Blend,
            }),
            ..default()
        });
    }
}

// This struct defines the data that will be passed to your shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct CustomMaterial {
    #[uniform(0)]
    color: LinearRgba,
    #[texture(1)]
    #[sampler(2)]
    color_texture: Option<Handle<Image>>,
    alpha_mode: AlphaMode,
}

/// The Material trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material api docs for details!
impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        LENS_MATERIAL_SHADER.into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}
