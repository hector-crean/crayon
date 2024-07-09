#[derive(
    Debug, Default, PartialEq, Eq, Copy, Clone, serde::Serialize, serde::Deserialize, specta::Type,
)]
pub enum ShapeVariant {
    #[default]
    Rectangle,
    Ellipse,
    Star,
    Polygon,
}
