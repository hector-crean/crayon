#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Grid2DPosition {
    pub x: i32,
    pub y: i32,
}

impl Grid2DPosition {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn manhattan_distance(&self, other: &Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    /// Moves the position by the given deltas
    pub fn move_by(&mut self, dx: i32, dy: i32, dz: i32) {
        self.x += dx;
        self.y += dy;
    }

    /// Checks if another position is adjacent (Manhattan distance of 1)
    pub fn is_adjacent(&self, other: &Self) -> bool {
        self.manhattan_distance(other) == 1
    }
}

impl From<(i32, i32)> for Grid2DPosition {
    fn from((x, y): (i32, i32)) -> Self {
        Self { x, y, }
    }
}

impl From<Grid2DPosition> for (i32, i32) {
    fn from(pos: Grid2DPosition) -> Self {
        (pos.x, pos.y)
    }
}
