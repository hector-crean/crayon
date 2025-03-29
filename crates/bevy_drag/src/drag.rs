use crate::{
    controllers::transform_controller::{TransformBounds, TransformController, TransformControllerSettings, TransformMode},
    events::TransformEvent,
    math::world_position_view_plane_intersection_world,
};
use bevy::prelude::*;
use bevy_camera::CameraMode;

///This plugin requires the bevy_mod_picking plugin
#[derive(Default)]
pub struct DragTransformPlugin<T: CameraMode>(pub T);

impl<T: CameraMode + Send + Sync + 'static> Plugin for DragTransformPlugin<T> {
    fn build(&self, app: &mut App) {
        app.insert_resource::<TransformControllerSettings>(TransformControllerSettings::default())
            .insert_resource(T::default())
            .add_event::<TransformEvent>()
            .add_systems(PostUpdate, Self::emit_transform_events)
            .add_systems(
                Last,
                Self::consume_transform_events.run_if(Self::run_criteria),
            );
    }
}

impl<T: CameraMode + Send + Sync + 'static> DragTransformPlugin<T> {
    fn run_criteria(
        _camera_mode: Res<T>,
        controller_settings: Res<TransformControllerSettings>,
    ) -> bool {
        controller_settings.enabled
    }

 

    fn emit_transform_events(
        mut drag_evt_rdr: EventReader<Pointer<Drag>>,
        mut dragstart_evt_rdr: EventReader<Pointer<DragStart>>,
        mut dragend_evt_rdr: EventReader<Pointer<DragEnd>>,
        mut transformable_query: Query<(&mut TransformController, &Transform, &TransformBounds)>,
        raycast_camera_query: Query<(&GlobalTransform, &Camera)>,
        mut transform_event_wtr: EventWriter<TransformEvent>,
        mut camera_controller: ResMut<T>,
        transform_controller_settings: Res<TransformControllerSettings>
    ) {
        for ev in dragstart_evt_rdr.read() {
            let Pointer {
                target,
                pointer_id,
                pointer_location,
                event,
            } = ev;

            camera_controller.lock();

            if let Ok((mut controller, transform, transform_bounds)) = transformable_query.get_mut(*target) {
                controller.drag_start_entity_position = Some(transform.translation);
                controller.drag_start_pointer_position = event.hit.position;
            }
        }
        for ev in drag_evt_rdr.read() {
            let Pointer {
                target,
                pointer_id,
                pointer_location,
                event: drag,
            } = ev;
            
            
            match transformable_query.get(*target) {
                Ok((controller, transform, transform_bounds)) => {

                    let TransformController {
                        enabled,
                        drag_start_entity_position,
                        drag_start_pointer_position,
                        mode,
                    } = controller;

                    if !enabled {
                        continue;
                    }

                    
                    let (camera_transform, camera) = raycast_camera_query.single();
                    let logical_viewport_size = camera.logical_viewport_size().unwrap();
                    let camera_affine3a = camera_transform.affine();
                    let view_mat4 = Mat4::from(camera_affine3a);
                    let inverse_view_mat4 = view_mat4.inverse();
                    let proj_mat4 = Camera::clip_from_view(camera);
                    let inverse_proj_mat4: Mat4 = proj_mat4.inverse();

                    if let (Some(drag_start_entity_position), Some(drag_start_pointer_position)) =
                        (*drag_start_entity_position, *drag_start_pointer_position)
                    {
                        let offset = drag_start_entity_position - drag_start_pointer_position;

                        let translation = world_position_view_plane_intersection_world(
                            drag_start_pointer_position,
                            pointer_location.position,
                            logical_viewport_size,
                            view_mat4,
                            inverse_view_mat4,
                            inverse_proj_mat4,
                        ) + offset;

                        match *mode {
                            TransformMode::Translate => {
                                let event = TransformEvent::Translate((*target, translation));

                                transform_event_wtr.send(event);
                            }
                            TransformMode::Rotate => {
                                let quat = transform.rotation;
                                // let event = TransformEvent::Rotate(*target, )
                                // transform.rotate_local_y(drag.delta.x / 50.0)
                            },
                            TransformMode::Scale => {},
                            _ => {}
                        }
                    }
                }

                Err(err) => {
                    info!("Failed to drag entity {target:?} as error: {err:?}");

                }
            }
        }
        for ev in dragend_evt_rdr.read() {
            if let Ok((controller, transform, transform_bounds)) = transformable_query.get(ev.target) {
                let mut t = transform.translation;

                if let Some(grid_snapping) = transform_controller_settings.grid_snapping {
                    t = snap_to_grid(t, grid_snapping);
                }

                if !transform_bounds.contains(t) {
                    t.x = t.x.clamp(transform_bounds.min.x, transform_bounds.max.x);
                    t.y = t.y.clamp(transform_bounds.min.y, transform_bounds.max.y);
                    t.z = t.z.clamp(transform_bounds.min.z, transform_bounds.max.z);
                }

                let TransformController {
                    enabled,
                    drag_start_entity_position,
                    drag_start_pointer_position,
                    mode,
                } = controller;

                if !enabled {
                    continue;
                }

            

                match *mode {
                    TransformMode::Translate => {
                        let event = TransformEvent::Translate((ev.target, t));

                        transform_event_wtr.send(event);
                    }
                    TransformMode::Rotate => {
                        let quat = transform.rotation;
                        // let event = TransformEvent::Rotate(*target, )
                        // transform.rotate_local_y(drag.delta.x / 50.0)
                    },
                    TransformMode::Scale => {},
                    _ => {}
                }

            }

            camera_controller.unlock();
        }
    }

    fn consume_transform_events(
        mut transform_evts_rdr: EventReader<TransformEvent>,
        mut transformable_query: Query<(&TransformController, &mut Transform, &TransformBounds)>,
        transform_controller_settings: Res<TransformControllerSettings>
    ) {
        for evt in transform_evts_rdr.read() {
            match evt {
                TransformEvent::Translate((entity, translation)) => {
                    let Ok((controller, mut transform, transform_bounds)) = transformable_query.get_mut(*entity)
                    else {
                        return;
                    };

                    let mut t =  *translation;

                    // if let Some(grid_snapping) = transform_controller_settings.grid_snapping {
                    //     t = snap_to_grid(t, grid_snapping);
                    // }


                    if let Some(constraints) = transform_controller_settings.translation_constraints {
                        //return to original values if translation constrained
                        if !constraints.x { t.x = transform.translation.x};
                        if !constraints.y  { t.y = transform.translation.y };
                        if !constraints.z { t.z = transform.translation.z };

                    }
                    if !transform_bounds.contains(t) {
                        t.x = t.x.clamp(transform_bounds.min.x, transform_bounds.max.x);
                        t.y = t.y.clamp(transform_bounds.min.y, transform_bounds.max.y);
                        t.z = t.z.clamp(transform_bounds.min.z, transform_bounds.max.z);
                    }
                    transform.translation = t;
                }
                TransformEvent::Rotate((entity, rotation)) => {
                    let Ok((controller, mut transform, transform_bounds)) = transformable_query.get_mut(*entity)
                    else {
                        return;
                    };
                    transform.rotation = *rotation;
                }
                TransformEvent::Scale((entity, scale)) => {
                    let Ok((controller, mut transform, transform_bounds)) = transformable_query.get_mut(*entity)
                    else {
                        return;
                    };
                    transform.scale = *scale;

                }
            }
        }
    }
}

#[derive(Component)]
#[require(TransformController, PickingBehavior)]
#[derive(Default)]
pub struct Draggable {
    pub dragging: bool,
}



fn snap_to_grid(position: Vec3, grid_dimension: Vec3) -> Vec3 {
    let snapped_x = (position.x / grid_dimension.x).round() * grid_dimension.x;
    let snapped_y = (position.y / grid_dimension.y).round() * grid_dimension.y;
    let snapped_z = (position.z / grid_dimension.z).round() * grid_dimension.z;

    Vec3 { x: snapped_x, y: snapped_y, z: snapped_z }
}

