#[derive(Debug, Default, PartialEq, Copy, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
#[serde(tag = "type")]
pub enum Cursor {
    #[default]
    Default,
    Grabbing,
    Crosshair,
    #[serde(rename_all = "camelCase")]
    Resize {
        rotation_deg: f32,
    },
    #[serde(rename_all = "camelCase")]
    Rotate {
        rotation_deg: f32,
    },
}
