use bevy::prelude::*;
pub trait CameraRaycast {
    fn get_cursor_world_position(
        &self,
        transform: &GlobalTransform,
        cursor_pos: Vec2,
        plane_origin: Vec3,
        plane_normal: Vec3,
    ) -> Option<Vec3>;

    fn get_cursor_world_position_fixed_distance(
        &self,
        transform: &GlobalTransform,
        cursor_pos: Vec2,
        distance: f32,
    ) -> Option<Vec3>;
}

impl CameraRaycast for Camera {
    fn get_cursor_world_position(
        &self,
        transform: &GlobalTransform,
        cursor_pos: Vec2,
        plane_origin: Vec3,
        plane_normal: Vec3,
    ) -> Option<Vec3> {
        let ray = self.viewport_to_world(transform, cursor_pos).ok()?;
        let distance = ray.intersect_plane(plane_origin, InfinitePlane3d::new(plane_normal))?;
        Some(ray.get_point(distance))
    }

    fn get_cursor_world_position_fixed_distance(
        &self,
        transform: &GlobalTransform,
        cursor_pos: Vec2,
        distance: f32,
    ) -> Option<Vec3> {
        let ray = self.viewport_to_world(transform, cursor_pos).ok()?;
        Some(ray.get_point(distance))
    }
}
