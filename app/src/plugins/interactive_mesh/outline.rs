
use bevy_mod_outline::OutlineVolume;
use bevy::{picking::focus::PickingInteraction, prelude::*};

use super::selection::Selectable;


pub struct MeshOutlinePlugin;

impl Plugin for MeshOutlinePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy_mod_outline::OutlinePlugin)
        .add_systems(Update, (Self::setup_outline, Self::update_outline));
    }
}

impl MeshOutlinePlugin {
    fn setup_outline(mut commands: Commands, query: Query<(Entity, &Selectable), (With<Selectable>, Added<Selectable>)>) {
        for (entity, selectable) in query.iter() {
            commands.entity(entity).insert(OutlineVolume {
                visible: selectable.is_selected,
                width: 2.0,
                colour: Color::WHITE,
            });
        }
    }
    fn update_outline(
        commands: Commands,
        mut button_query: Query<(Entity, &Selectable, &PickingInteraction, &mut OutlineVolume), (Changed<Selectable>, Changed<PickingInteraction>)>,
    ) {
        for (entity, selectable, interaction, mut outline) in button_query.iter_mut() {
            info!("outline: {:?}", outline.visible);
           
           outline.visible = selectable.is_selected;
            // match *interaction {
            //     PickingInteraction::Pressed => {
            //         material.0 = active_material_handle.clone();
            //     }
            //     PickingInteraction::Hovered => {
            //         material.0 = active_material_handle.clone();
            //     }
            //     PickingInteraction::None => {
            //         material.0 = inactive_material_handle.clone();
            //     }
    
            // }
    
        }
    }
    
}