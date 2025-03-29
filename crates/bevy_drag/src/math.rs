use bevy::prelude::*;

pub fn world_position_view_plane_intersection_world(
    p_world: Vec3,
    cursor_position: Vec2,
    logical_viewport_size: Vec2,
    view: Mat4,
    inverse_view: Mat4,
    inverse_projection: Mat4,
) -> Vec3 {
    let p_view = inverse_view.transform_point3(p_world);

    // Convert cursor position to NDC (top-left origin)
    let cursor_position_ndc = compute_cursor_position_ndc(cursor_position, logical_viewport_size);

    let ray_dir_view = compute_ray_direction_view(cursor_position_ndc, inverse_projection);
    let intersection_view = compute_intersection_view(p_view.z, ray_dir_view);

    view.transform_point3(intersection_view)
}

pub fn compute_cursor_position_ndc(cursor_position: Vec2, logical_viewport_size: Vec2) -> Vec2 {
    let ndc_x = (cursor_position.x / logical_viewport_size.x) * 2.0 - 1.0;
    let ndc_y = 1.0 - (cursor_position.y / logical_viewport_size.y) * 2.0;
    Vec2::new(ndc_x, ndc_y)
}

pub fn compute_ray_direction_view(cursor_position_ndc: Vec2, inverse_projection: Mat4) -> Vec3 {
    let ray_dir_ndc = cursor_position_ndc.extend(1.0);
    inverse_projection.project_point3(ray_dir_ndc)
}

pub fn compute_intersection_view(p_view_z: f32, ray_dir_view: Vec3) -> Vec3 {
    (p_view_z / ray_dir_view.z) * ray_dir_view
}
