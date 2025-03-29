use bevy::{
    ecs::observer::Trigger,
    prelude::*,
};

#[derive(Debug, Resource, Default)]
pub struct SelectionPluginSettings {
    /// Should selection systems run?
    pub is_enabled: bool,
    /// A pointer clicks and nothing is beneath it, should everything be deselected?
    pub click_nothing_deselect_all: bool,
}

/// Tracks the current selection state of the entity.
#[derive(Component, Debug, Default, Clone, Reflect)]
#[reflect(Component, Default)]
pub struct Selectable {
    /// `true` if this entity is selected.
    pub is_selected: bool,
}

impl Selectable {
    pub fn toggle(&mut self) {
        self.is_selected = !self.is_selected;
    }

    fn toggle_selected() -> impl Fn(Trigger<Pointer<Click>>, Query<&mut Self>) {
        move |trigger, mut query| {
            if let Ok(mut selected) = query.get_mut(trigger.entity()) {
                selected.toggle()
            }
        }
    }
    
}

#[derive(Component)]
pub struct Selected;

#[derive(Component)]
pub struct NotSelected;




#[derive(Component, Debug, Default, Copy, Clone, Reflect)]
pub struct Deselecter;


/// Marker struct used to mark pickable entities for which you don't want to trigger a deselection
/// event when picked. This is useful for gizmos or other pickable UI entities.
#[derive(Component, Debug, Default, Copy, Clone, Reflect)]
#[reflect(Component, Default)]
pub struct NoDeselect;

pub struct SelectionPlugin;

impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(SelectionPluginSettings::default())
        .add_systems(Update, (Self::setup_selectable, Self::setup_deselecters, Self::update_selection_system));
    }
}

impl SelectionPlugin {

    fn update_selection_system(
        mut commands: Commands,
        changed_query: Query<(Entity, &Selectable), Or<(Added<Selectable>, Changed<Selectable>)>>,
        
    ) {
        for (entity, selectable) in changed_query.iter() {
            if selectable.is_selected {
                commands.entity(entity).insert(Selected);
            } else {
                commands.entity(entity).remove::<Selected>();
            }
        }
      
    }
    
    fn setup_selectable(mut commands: Commands, query: Query<Entity, Added<Selectable>>) {
        for entity in query.iter() {
            commands
                .entity(entity)
                .observe(Self::handle_click_on_selectable());
        }
    }



    fn setup_deselecters(mut commands: Commands, query: Query<Entity, Added<Deselecter>>) {
        for deselecter_entity in query.iter() {
            commands
                .entity(deselecter_entity)
                .observe(Self::handle_click_on_deselecter());
        }
    }

    fn handle_click_on_selectable() -> impl Fn(Trigger<Pointer<Click>>, Query<&mut Selectable, Without<NoDeselect>>) {
        move |trigger, mut query| {
            if let Ok(mut selected) = query.get_mut(trigger.entity()) {
                selected.toggle()
            }
        }
    }

    fn handle_click_on_deselecter(
    ) -> impl Fn(Trigger<Pointer<Click>>, Query<&mut Selectable, Without<NoDeselect>>) {
        move |_, mut query| {
            for mut selectable in query.iter_mut() {
                selectable.is_selected = false;
            }
        }
    }

  
    // fn click_on_nothing(
    //     pointer_downs: EventReader<Pointer<Click>>,
    //     pointer_query: Query<&PointerInteraction>,
    //     selectables_query: Query<&mut Selectable, Without<NoDeselect>>,
    // ) {
    //     for pointer in pointer_query.iter() {
    //         if pointer.is_empty() {
    //             for mut selected in selectables_query.iter_mut() {
    //                 selected.is_selected = false;
    //             }
    //         }
    //     }
    // }


}
