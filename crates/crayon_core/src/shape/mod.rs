


#[derive(
    Debug, Default, PartialEq, Eq, Copy, Clone, serde::Serialize, serde::Deserialize
)]
pub enum ShapeVariant {
    #[default]
    Rectangle,
    Ellipse,
    Star,
    Polygon,
}
