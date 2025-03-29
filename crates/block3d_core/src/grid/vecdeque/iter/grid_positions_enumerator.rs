use crate::grid::grid_position::Grid2DPosition;


pub struct Grid2DPositions<I> {
    pub(crate) inner: I,
    pub(crate) next_pos: fn(&I, Option<Grid2DPosition>) -> Grid2DPosition,
    pub(crate) prev_position: Option<Grid2DPosition>,
}

pub trait Grid2DPositionsEnumerator
where
    Self: Sized,
{
    fn grid_positions(self) -> Grid2DPositions<Self>;
}

impl<I: Iterator> Iterator for Grid2DPositions<I> {
    type Item = (Grid2DPosition, I::Item);
    fn next(&mut self) -> Option<Self::Item> {
        let next_pos = (self.next_pos)(&self.inner, self.prev_position);
        self.prev_position = Some(next_pos);
        Some((next_pos, self.inner.next()?))
    }
}