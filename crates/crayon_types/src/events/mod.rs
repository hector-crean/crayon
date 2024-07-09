use bevy::{
    ecs::{entity::Entity, event::Event},
    math::Vec2,
};

use crate::interaction::{cursor::Cursor, mode::InteractionMode, tool::InteractionTool};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct InteractionToolChangeEvent {
    pub interaction_tool: InteractionTool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct InteractionModeChangeEvent {
    pub interaction_mode: InteractionMode,
}

#[derive(
    Debug, Default, PartialEq, Copy, Clone, serde::Serialize, serde::Deserialize, specta::Type,
)]
pub struct CursorChangeEvent {
    pub cursor: Cursor,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type, Event)]
#[serde(tag = "type")]
pub enum DrayonEvent {
    InteractionModeChange(InteractionModeChangeEvent),
    InteractionToolChange(InteractionToolChangeEvent),
    CursorChange(CursorChangeEvent),
}
