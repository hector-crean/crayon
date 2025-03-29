 use bevy::pbr::{Material, MeshMaterial3d};
 use bevy::ecs::prelude::*;
 
 use bevy::prelude::Shader;
 use bevy::render::{mesh::{Mesh, Mesh3d}, render_resource::{AsBindGroup, ShaderRef}};
 use bevy::color::LinearRgba;
 use bevy::color::palettes::basic::RED;
 use bevy::asset::{load_internal_asset, Asset, AssetServer, Assets, Handle};
 use bevy::math::primitives::Capsule3d;
use bevy::prelude::*;

 #[derive(AsBindGroup, Debug, Clone, Asset, Default, Reflect)]
 pub struct ButtonMaterial {
     // Uniform bindings must implement `ShaderType`, which will be used to convert the value to
     // its shader-compatible equivalent. Most core math types already implement `ShaderType`.
     #[uniform(0)]
     color: LinearRgba,
   
 }

 // All functions on `Material` have default impls. You only need to implement the
 // functions that are relevant for your material.
 impl Material for ButtonMaterial {
     fn fragment_shader() -> ShaderRef {
         "shaders/custom_material.wgsl".into()
     }
 }



pub struct ButtonMaterialPlugin;

pub const SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(12823766040132746065);


impl Plugin for ButtonMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<ButtonMaterial>()
        .register_type::<ButtonMaterial>();
        load_internal_asset!(
            app,
            SHADER_HANDLE,
            "button_material.wgsl",
            Shader::from_wgsl
        );
    }
}





 // Spawn an entity with a mesh using `ButtonMaterial`.
 fn setup(
     mut commands: Commands,
     mut meshes: ResMut<Assets<Mesh>>,
     mut materials: ResMut<Assets<ButtonMaterial>>,
     asset_server: Res<AssetServer>
 ) {
     commands.spawn((
         Mesh3d(meshes.add(Capsule3d::default())),
         MeshMaterial3d(materials.add(ButtonMaterial {
             color: RED.into(),
         })),
     ));
 }
//  ```

//  In WGSL shaders, the material's binding would look like this:

//  ```wgsl
//  @group(2) @binding(0) var<uniform> color: vec4<f32>;
//  @group(2) @binding(1) var color_texture: texture_2d<f32>;
//  @group(2) @binding(2) var color_sampler: sampler;
//  ```