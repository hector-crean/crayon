




#[derive(Debug, Default, Copy, Clone, serde::Deserialize, serde::Serialize)]
#[serde(tag = "type")]
pub enum InteractionMode {
    /// Default canvas mode. Nothing is happening.
    #[default]
    None,
    Commenting
}



#[derive(Debug, Default, PartialEq, Eq, Copy, Clone, serde::Serialize,)]
pub enum InteractionModeLabel {
    #[default]
    None,
    Commenting,
}

impl From<&InteractionMode> for InteractionModeLabel {
    fn from(interaction_mode: &InteractionMode) -> Self {
        match interaction_mode {
            InteractionMode::None => Self::None,
            InteractionMode::Commenting { .. } => Self::Commenting,
        }
    }
}
