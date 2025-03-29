use super::{Grid2DPositions, Grid2DPositionsEnumerator};

pub struct RowIter<'a, T> {
    pub(crate) row_iter: std::slice::Iter<'a, T>,
    pub(crate) idx: i32,
}

impl<'a, T> Iterator for RowIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.row_iter.next()
    }
}

impl<T: 'static> Grid2DPositionsEnumerator for RowIter<'_, T> {
    fn grid_positions(self) -> Grid2DPositions<Self> {
        Grid2DPositions {
            next_pos: |inner, prev_pos| match prev_pos {
                None => (0, inner.idx).into(),
                Some(p) => (p.x + 1, p.y).into(),
            },
            prev_position: None,
            inner: self,
        }
    }
}

pub struct RowIterMut<'a, T> {
    pub(crate) row_iter: std::slice::IterMut<'a, T>,
    pub(crate) idx: i32,
}

impl<'a, T> Iterator for RowIterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.row_iter.next()
    }
}

impl<T: 'static> Grid2DPositionsEnumerator for RowIterMut<'_, T> {
    fn grid_positions(self) -> Grid2DPositions<Self> {
        Grid2DPositions {
            next_pos: |inner, prev_pos| match prev_pos {
                None => (0, inner.idx).into(),
                Some(p) => (p.x + 1, p.y).into(),
            },
            prev_position: None,
            inner: self,
        }
    }
}

#[cfg(test)]
mod tests {
 

 
}