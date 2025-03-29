pub mod highlight;
pub mod outline;
pub mod selection;
pub mod drag;

use bevy::prelude::*;

use crate::state::camera::CameraModeImpl;


use drag::DragTransformPlugin;
use highlight::DefaultHighlightingPlugin;
use outline::MeshOutlinePlugin;
use selection::SelectionPlugin;

pub struct InteractiveMeshPlugin;

impl Plugin for InteractiveMeshPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            SelectionPlugin,
            DefaultHighlightingPlugin,
            MeshOutlinePlugin,
            DragTransformPlugin::<CameraModeImpl>::default(),
        ));
    }
}
