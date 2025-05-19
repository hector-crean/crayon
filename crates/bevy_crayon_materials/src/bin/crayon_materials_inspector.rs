use bevy::prelude::*;
use bevy_crayon_materials::wireframe::{VoidMaterial, VoidMaterialPlugin};

fn main() {
    let mut app = App::new();

    app.add_plugins((DefaultPlugins, VoidMaterialPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut wireframe_materials: ResMut<Assets<VoidMaterial>>,
    // mut standard_materials: ResMut<Assets<StandardMaterial>>,
) {
  
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(4.0, 8.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Add ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 1.0,
    });

    // Add directional light
    commands.spawn((
        DirectionalLight {
            color: Color::WHITE,
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Cube with VoidMaterial
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        Transform::from_xyz(0.0, 0.0, 0.0),
        GlobalTransform::default(),
        MeshMaterial3d(wireframe_materials.add(VoidMaterial {
        color: Color::rgba(0.0, 0.0, 1.0, 0.5).into(),
            grid_thickness: 1.,
            grid_spacing: 0.1,
        })),
       
    ));
   
}


