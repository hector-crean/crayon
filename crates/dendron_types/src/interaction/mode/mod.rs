// use bevy::math::Vec2;
#[derive(Debug, Copy, Clone, serde::Deserialize, specta::Type)]
pub struct Vec2 {
    x: f32,
    y: f32,
}
use crate::shape::ShapeVariant;

#[derive(Debug, Copy, Clone, serde::Deserialize, specta::Type)]
pub struct XYWH {
    pub position: Vec2,
    pub size: Vec2,
}

#[derive(Debug, Default, Copy, Clone, serde::Deserialize, serde::Serialize, specta::Type)]
#[serde(tag = "type")]
pub enum InteractionMode {
    /// Default canvas mode. Nothing is happening.
    #[default]
    None,
    Inserting {
        shape_variant: ShapeVariant,
    },
}

#[derive(Debug, Default, PartialEq, Eq, Copy, Clone, serde::Serialize, specta::Type)]
pub enum InteractionModeLabel {
    #[default]
    None,
    Inserting,
}

impl From<&InteractionMode> for InteractionModeLabel {
    fn from(interaction_mode: &InteractionMode) -> Self {
        match interaction_mode {
            InteractionMode::None => Self::None,
            InteractionMode::Inserting { .. } => Self::Inserting,
        }
    }
}
