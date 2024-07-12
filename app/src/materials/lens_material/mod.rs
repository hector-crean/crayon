//! A shader and a material that uses it.

use bevy::{
    asset::embedded_asset,
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
};

// pub(crate) const LENS_MATERIAL_SHADER: &str =
//     "embedded://crayon_app/materials/lens_material/lens.wgsl";

pub(crate) const LENS_MATERIAL_SHADER: &str = "shaders/lens.wgsl";

pub struct LensMaterialPlugin;

impl Plugin for LensMaterialPlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "lens.wgsl");
        app.add_plugins(MaterialPlugin::<LensMaterial>::default());
    }
}

impl LensMaterialPlugin {
    pub fn example_setup(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<LensMaterial>>,
        asset_server: Res<AssetServer>,
    ) {
        let amsler_grid = asset_server.load("img/DNA.png");
        let refraction_map = asset_server.load("img/sky_refraction_map.png");
        let refraction_lut = asset_server.load("img/sky_refraction_lut.png");

        // cube
        commands.spawn(MaterialMeshBundle {
            mesh: meshes.add(Plane3d::default()),
            transform: Transform::from_xyz(0.0, 0., 0.0),
            material: materials.add(LensMaterial {
                refactive_index: 1.1,
                diffuse_map: Some(amsler_grid),
                refraction_map: Some(refraction_map),
                refraction_lut: Some(refraction_lut),
                alpha_mode: AlphaMode::Blend,
            }),
            ..default()
        });
    }
}

// This struct defines the data that will be passed to your shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct LensMaterial {
    #[uniform(0)]
    refactive_index: f32,
    #[texture(1)]
    #[sampler(2)]
    diffuse_map: Option<Handle<Image>>,
    #[texture(3)]
    #[sampler(4)]
    refraction_lut: Option<Handle<Image>>,
    #[texture(5)]
    #[sampler(6)]
    refraction_map: Option<Handle<Image>>,

    alpha_mode: AlphaMode,
}

/// The Material trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material api docs for details!
impl Material for LensMaterial {
    fn fragment_shader() -> ShaderRef {
        LENS_MATERIAL_SHADER.into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}
