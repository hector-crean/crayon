// Add these near the top of the file with other imports
use bevy::pbr::{MaterialPipeline, MaterialPipelineKey, MaterialPlugin};
use bevy::render::mesh::MeshVertexBufferLayout;
use bevy::render::render_resource::{AsBindGroup, ShaderRef, RenderPipelineDescriptor, PolygonMode};
use bevy::{prelude::*, asset::Asset, asset::load_internal_asset};

const SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(1234567890);

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct VoidMaterial {
    #[uniform(0)]
    pub color: LinearRgba,
    #[uniform(0)]
    pub grid_thickness: f32,
    #[uniform(0)]
    pub grid_spacing: f32,
}

impl Material for VoidMaterial {
    fn fragment_shader() -> ShaderRef {
        SHADER_HANDLE.into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

pub struct VoidMaterialPlugin;

impl Plugin for VoidMaterialPlugin {
    fn build(&self, app: &mut App) {
        // Load shader as an internal asset
        load_internal_asset!(
            app,
            SHADER_HANDLE,
            "void_material.wgsl",
            Shader::from_wgsl
        );

        app.add_plugins(MaterialPlugin::<VoidMaterial>::default());
    }
}