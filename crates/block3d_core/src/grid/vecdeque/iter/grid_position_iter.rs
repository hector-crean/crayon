use crate::grid::grid_position::Grid2DPosition;

pub struct Grid2DPositionsIter {  
    pub(crate) len: i32,
    pub(crate) width: i32,
    pub(crate) height: i32,
    pub(crate) idx: i32,
}

impl Iterator for Grid2DPositionsIter {
    type Item = Grid2DPosition;
    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.len {
            return None;
        }
        let rem = self.idx % (self.width * self.height);
        let y = rem / self.width;
        let x = rem % self.width;
        self.idx += 1;
        Some((x, y).into())
    }
}
