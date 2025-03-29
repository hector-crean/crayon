use crate::grid::grid_position::Grid2DPosition;

pub struct NeighboursIter {
    position: Grid2DPosition,
    index: usize,
}

impl NeighboursIter {
    pub fn new(position: Grid2DPosition) -> Self {
        NeighboursIter { position, index: 0 }
    }
}

impl Iterator for NeighboursIter {
    type Item = Grid2DPosition;

    fn next(&mut self) -> Option<Self::Item> {
        let directions = [
            (0, -1), // North
            (1, 0),  // East
            (0, 1),  // South
            (-1, 0), // West
        ];

        if self.index < directions.len() {
            let (dx, dy) = directions[self.index];
            self.index += 1;
            Some(Grid2DPosition {
                x: self.position.x + dx,
                y: self.position.y + dy,
            })
        } else {
            None
        }
    }
}
