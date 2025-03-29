use bevy::prelude::*;



#[derive(Event, Debug)]
pub enum TransformEvent {
    Translate((Entity, Vec3)),
    Rotate((Entity, Quat)),
    Scale((Entity, Vec3)),
}
