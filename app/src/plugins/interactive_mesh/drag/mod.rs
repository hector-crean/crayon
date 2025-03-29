use bevy::prelude::*;
use bevy_camera::CameraMode;





/// A small struct that holds camera-related transforms
/// for converting between world, view, NDC, etc.
pub struct CameraRayHelper {
    pub view_mat: Mat4,
    pub inv_view_mat: Mat4,
    pub proj_mat: Mat4,
    pub inv_proj_mat: Mat4,
    pub logical_viewport_size: Vec2,
}

impl CameraRayHelper {
    /// Build a `CameraRayHelper` from the `camera` and its `GlobalTransform`.
    pub fn from_camera(camera: &Camera, camera_transform: &GlobalTransform) -> Option<Self> {
        let Some(viewport_size) = camera.logical_viewport_size() else {
            return None;
        };

        let affine = camera_transform.affine();
        let view_mat = Mat4::from(affine);
        let inv_view_mat = view_mat.inverse();
        let proj_mat = Camera::clip_from_view(camera);
        let inv_proj_mat = proj_mat.inverse();

        Some(Self {
            view_mat,
            inv_view_mat,
            proj_mat,
            inv_proj_mat,
            logical_viewport_size: viewport_size,
        })
    }

    /// Convert a 2D cursor position into a world-space position on a “view plane”
    /// given a reference world-position (`p_world`).
    pub fn world_position_on_view_plane(
        &self,
        p_world: Vec3,
        cursor_pos: Vec2,
    ) -> Vec3 {
        // 1) transform p_world into view space
        let p_view = self.inv_view_mat.transform_point3(p_world);

        // 2) convert cursor pos to NDC
        let cursor_ndc = compute_cursor_position_ndc(cursor_pos, self.logical_viewport_size);

        // 3) compute ray direction in view space
        let ray_view = compute_ray_direction_view(cursor_ndc, self.inv_proj_mat);

        // 4) find the intersection in view space
        let intersection_view = compute_intersection_view(p_view.z, ray_view);

        // 5) convert intersection back to world space
        self.view_mat.transform_point3(intersection_view)
    }
}

/// Helper for converting (x, y) in screen coords to normalized device coords
pub fn compute_cursor_position_ndc(cursor_position: Vec2, viewport_size: Vec2) -> Vec2 {
    let ndc_x = (cursor_position.x / viewport_size.x) * 2.0 - 1.0;
    // Note: y is often flipped, so we invert it
    let ndc_y = 1.0 - (cursor_position.y / viewport_size.y) * 2.0;
    Vec2::new(ndc_x, ndc_y)
}

/// Given a cursor in NDC, invert the projection to get a ray direction in view space
pub fn compute_ray_direction_view(ndc_pos: Vec2, inv_projection: Mat4) -> Vec3 {
    let ray_dir_ndc = ndc_pos.extend(1.0); // z=1 -> forward direction
    inv_projection.project_point3(ray_dir_ndc)
}

/// Intersection in view space: scale the ray so it hits the plane z = p_view_z
pub fn compute_intersection_view(p_view_z: f32, ray_dir_view: Vec3) -> Vec3 {
    (p_view_z / ray_dir_view.z) * ray_dir_view
}


#[derive(Component)]
#[derive(Default)]
pub struct Draggable {
    dragging: bool,
    drag_start_entity_position: Option<Vec3>,
    drag_start_pointer_position: Option<Vec3>,
}
impl Draggable {
    pub fn new() -> Self {
        Self {
            dragging: false,
            drag_start_entity_position: None,
            drag_start_pointer_position: None,
        }
    }
    fn cursor_offset(&self) -> Vec3 {
        self.drag_start_entity_position.unwrap() - self.drag_start_pointer_position.unwrap()
    }
      // Optionally, a method to do a standard “drag translation”:
      pub fn compute_drag_translation(
        &self,
        cursor_pos: Vec2,
        camera_ray: &CameraRayHelper,
    ) -> Option<Vec3> {
        let Some(entity_pos) = self.drag_start_entity_position else { return None; };
        let Some(start_pointer_pos) = self.drag_start_pointer_position else { return None; };

        // The “plane intersection” at the drag-start pointer pos vs. new pointer pos
        let new_pos = camera_ray.world_position_on_view_plane(
            entity_pos,
            cursor_pos,
        ) + self.cursor_offset();

        Some(new_pos)
    }
}
#[derive(Default)]
pub struct DragTransformPlugin<T: CameraMode>(pub T);


impl<T: CameraMode + Send + Sync + 'static> Plugin for DragTransformPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, Self::sync_drag_system);
    }
}

impl<T: CameraMode + Send + Sync + 'static> DragTransformPlugin<T> {
    fn sync_drag_system(mut commands: Commands, draggable_query: Query<Entity, Added<Draggable>>) {
        for entity in draggable_query.iter() {
            commands.entity(entity).observe(Self::on_drag_start);
            commands.entity(entity).observe(Self::on_drag);
            commands.entity(entity).observe(Self::on_drag_end);
        }
    }
    fn on_drag_start(
        drag_start: Trigger<Pointer<DragStart>>,
        commands: Commands,
        mut draggable_query: Query<(Entity, &mut Draggable, &Transform)>,
        mut camera_controller: ResMut<T>,

    ) {
        if let Ok((entity, mut draggable, transform)) = draggable_query.get_mut(drag_start.entity())
        {
            // commands.entity(entity).insert(NoDeselect);
            draggable.drag_start_entity_position = Some(transform.translation);
            draggable.drag_start_pointer_position = drag_start.hit.position;

            camera_controller.lock();

        }
    }
    fn on_drag(
        drag: Trigger<Pointer<Drag>>,
        mut draggable_query: Query<(Entity, &mut Draggable, &mut Transform)>,
        camera_query: Query<(&GlobalTransform, &Camera)>,
    ) {
        let (camera_transform, camera) = camera_query.single();
        let Ok((entity, draggable, mut transform)) = draggable_query.get_mut(drag.entity()) else {
            return;
        };

        let Some(camera_ray) = CameraRayHelper::from_camera(camera, camera_transform) else {
            return;
        };

        if let Some(new_translation) = draggable.compute_drag_translation(
            drag.pointer_location.position,
            &camera_ray
        ) {
            let translation = new_translation;
            transform.translation = translation;
        }
    }
    fn on_drag_end(
        drag_end: Trigger<Pointer<DragEnd>>,
        commands: Commands,
        mut draggable_query: Query<(Entity, &mut Draggable, &mut Transform)>,
        mut camera_controller: ResMut<T>,
    ) {
        let Ok((entity, mut draggable, transform)) = draggable_query.get_mut(drag_end.entity()) else {
            return;
        };
        draggable.dragging = false;

        // commands.entity(entity).remove::<NoDeselect>();

        camera_controller.unlock();
    }
}
