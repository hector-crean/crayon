use bevy::{prelude::*, render::camera::Camera};


pub trait CameraController: Component
where
    Self: Sized,
{
    fn update_camera_transform_system(
        query: Query<(&Self, &mut Transform), (Changed<Self>, With<Camera>)>,
    );
}

pub trait CameraMode: Resource + Default + PartialEq + Eq {
    fn is_locked(&self) -> bool;
    fn lock(&mut self);
    fn unlock(&mut self);
}

// We may have several cameras : how do we switch between active cameras?

pub trait CameraRig: Resource {
    fn add_camera(&mut self, camera: Camera);
    fn remove_camera(&mut self, camera: &Camera);
    fn update(&mut self, cameras: &mut Query<&mut Camera>);
    //add/remove controller marker components? We want there to be only one controller component per camera bundle?
}

pub struct DefaultCameraRig {
    cameras: Vec<Entity>,
}

impl DefaultCameraRig {
    pub fn new() -> Self {
        Self {
            cameras: Vec::new(),
        }
    }
}
