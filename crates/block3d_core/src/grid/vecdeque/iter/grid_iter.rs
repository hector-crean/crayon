
pub struct GridIter<'a, T> {
    pub(crate) grid_iter: std::slice::Iter<'a, T>,
    pub(crate) width: i32,
    pub(crate) height: i32,
}

impl<'a, T> Iterator for GridIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.grid_iter.next()
    }
}

pub struct GridIterMut<'a, T> {
    pub(crate) grid_iter: std::slice::IterMut<'a, T>,
    pub(crate) width: i32,
    pub(crate) height: i32,
}

impl<'a, T> Iterator for GridIterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.grid_iter.next()
    }
}

