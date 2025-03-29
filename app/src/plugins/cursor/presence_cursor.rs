use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::app_plugin::GroundPlane;
use bevy::render::primitives::Aabb;
use bevy::math::bounding::{Aabb3d, RayCast3d};
use bevy::math::bounding::BoundingVolume;

#[derive(Component)]
pub struct PresenceCursor;

pub struct PresenceCursorPlugin;

impl Plugin for PresenceCursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_cursor)
           .add_systems(Update, update_cursor_position);
    }
}

fn setup_cursor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PresenceCursor,
        Mesh3d(meshes.add(Sphere { 
            radius: 0.1,
            ..default()
        })),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::rgba(0.0, 0.8, 1.0, 0.5),
            alpha_mode: AlphaMode::Blend,
            ..default()
        })),
        Transform::from_xyz(0.0, 0.1, 0.0),
    ));
}


fn update_cursor_position(
    camera_query: Query<(&Camera, &GlobalTransform)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut cursor_query: Query<&mut Transform, With<PresenceCursor>>,
    ground: Query<&GlobalTransform, With<GroundPlane>>,
    // Query for meshes with their AABBs
    intersection_query: Query<(Entity, &GlobalTransform, &Aabb)>,
) {
    let Ok(window) = window_query.get_single() else { return };
    let Ok((camera, camera_transform)) = camera_query.get_single() else { return };
    
    let Some(cursor_position) = window.cursor_position() else { return };
    let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position) else { return };
    
    // Create ray cast for intersection testing
    let ray_cast = RayCast3d::new(
        ray.origin,
        ray.direction,
        f32::MAX
    );

    // Check for intersections with meshes first
    let mut closest_intersection = None;
    for (_entity, transform, aabb) in &intersection_query {
        // Transform AABB to world space
        // let aabb3d = Aabb3d::new(
        //     aabb.min().into(),
        //     aabb.max().into()
        // );
        // let world_aabb = aabb3d.transform(transform.compute_matrix());
        
        // if let Some(distance) = ray_cast.aabb_intersection_at(&world_aabb) {
        //     if closest_intersection.is_none() || distance < closest_intersection.unwrap() {
        //         closest_intersection = Some(distance);
        //     }
        // }
    }

    // Get ground plane intersection
    let ground_transform = ground.single();
    let plane = InfinitePlane3d::new(ground_transform.up());
    
    if let Some(ground_distance) = ray.intersect_plane(ground_transform.translation(), plane) {
        // Only use ground intersection if it's closer than any mesh intersection
        if closest_intersection.is_none() || ground_distance < closest_intersection.unwrap() {
            let world_position = ray.get_point(ground_distance);
            if let Ok(mut cursor_transform) = cursor_query.get_single_mut() {
                cursor_transform.translation = world_position + Vec3::Y * 0.1;
            }
        }
    }
}

