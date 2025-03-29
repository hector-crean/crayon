use bevy::winit::cursor::CursorIcon;
use bevy::{prelude::*, winit::cursor::CustomCursor};
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_asset_loader::loading_state::LoadingState;
use bevy_asset_loader::loading_state::LoadingStateAppExt;
use bevy_asset_loader::prelude::ConfigureLoadingState;
use strum::EnumIter;
use strum::EnumProperty;
use strum::IntoStaticStr;
pub mod presence_cursor;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum CursorsLoadingState {
    #[default]
    Loading,
    Loaded,
}

#[derive(
    States, Clone, EnumIter, Default, IntoStaticStr, EnumProperty, Hash, Eq, PartialEq, Debug,
)]
pub enum CursorState {
    #[default]
    ChoosePoint,
}

impl CursorState {
    fn cursor_icon(&self, cursor_assets: &CursorAssets) -> CursorIcon {
        let cursor = match self {
            CursorState::ChoosePoint => CustomCursor::Image {
                handle: cursor_assets.navigation.clone(),
                hotspot: (23, 0),
            },
        };

        CursorIcon::Custom(cursor)
    }
}




#[derive(AssetCollection, Resource)]
struct CursorAssets {
    #[asset(path = "icons/navigation_24px.png")]
    navigation: Handle<Image>,
}



pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<CursorsLoadingState>()
            .init_state::<CursorState>()
            .add_loading_state(
                LoadingState::new(CursorsLoadingState::Loading)
                    .continue_to_state(CursorsLoadingState::Loaded)
                    .load_collection::<CursorAssets>(),
            )
            .add_systems(
                OnEnter(CursorsLoadingState::Loaded),
                Self::setup_cursor,
            )
            .add_systems(
                Update,
                Self::update_cursor
                    .run_if(in_state(CursorsLoadingState::Loaded))
                    .run_if(state_changed::<CursorState>),
            );
    }
}

impl CursorPlugin {
    fn setup_cursor(
        mut commands: Commands,
        window: Single<Entity, With<Window>>,
        cursor_assets: Res<CursorAssets>,
        cursor_state: Res<State<CursorState>>,
    ) {
        let cursor_state = cursor_state.get();
        let cursor = cursor_state.cursor_icon(&cursor_assets);
        commands.entity(*window).insert(cursor);
    }
    fn update_cursor(
        mut commands: Commands,
        window: Single<Entity, With<Window>>,
        cursor_assets: Res<CursorAssets>,
        cursor_state: Res<State<CursorState>>,
    ) {
        let cursor_state = cursor_state.get();
        let cursor = cursor_state.cursor_icon(&cursor_assets);
        commands.entity(*window).insert(cursor);
    }
}
