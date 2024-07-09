use crate::shape::ShapeVariant;

#[derive(
    Debug, Default, PartialEq, Eq, Copy, Clone, serde::Serialize, serde::Deserialize, specta::Type,
)]
#[serde(tag = "type")]
pub enum InteractionTool {
    #[default]
    /// When the user wants to select nodes and move them around.
    Select,
    /// When the user wants to insert new shape nodes.
    Shape { variant: ShapeVariant },
}
