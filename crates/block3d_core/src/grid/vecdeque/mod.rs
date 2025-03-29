pub mod iter;


#[derive(Debug, PartialEq)]
pub struct Grid<T: Clone + Copy> {
    pub(crate) items: Vec<T>,
    pub(crate) width: i32,
    pub(crate) height: i32,
    pub(crate) depth: i32,
}

// impl Grid3DLike for Grid<T> {
//     type GridItem = T;
//     type Position = GridPosition;

//     type Iter = GridIter<'_, T>;
//     type IterMut = GridIterMut<'_, T>;  

//     type RowIter = RowIter<'_, T>;
//     type RowIterMut = RowIterMut<'_, T>;

//     type ColumnIter = ColumnIter<'_, T>;
//     type ColumnIterMut = ColumnIterMut<'_, T>;

//     type DepthIter = DepthIter<'_, T>;
//     type DepthIterMut = DepthIterMut<'_, T>;

//     type LayerIter = LayerIter<'_, T>;
//     type LayerIterMut = LayerIterMut<'_, T>;

//     type NeighborsIter = NeighborsIter<'_, T>;
//     type NeighborsIterMut = NeighborsIterMut<'_, T>;

//     type BoundaryIter = BoundaryIter<'_, T>;
//     type BoundaryIterMut = BoundaryIterMut<'_, T>;  

//     fn dimensions(&self) -> (i32, i32, i32) {
//         (self.width, self.height, self.depth)
//     }

//     fn get(&self, pos: Self::Position) -> Option<&Self::GridItem> {
//         self.items.get(pos.into())
//     }
//     fn get_mut(&mut self, pos: Self::Position) -> Option<&mut Self::GridItem> {
//         self.items.get_mut(pos.into())
//     }
//     fn set(&mut self, pos: Self::Position, item: Self::GridItem) {
//         self.items[pos.into()] = item;
//     }

// }

// impl<T: Clone + Copy> Grid<T> {
//     pub fn new(width: i32, height: i32, depth: i32, default_value: T) -> Self {
//         if width == 0 || height == 0 || depth == 0 {
//             panic!("width and height can not be zero");
//         }
//         Self {
//             width,
//             height,
//             depth,
//             items: vec![default_value; (width * height * depth) as usize],
//         }
//     }
//     pub fn is_bounds<P: Into<GridPosition>>(&self, pos: P) -> bool {
//         let pos = pos.into();
//         pos.x < self.width && pos.y < self.height && pos.z < self.depth
//     }

//     pub fn iter(&self) -> GridIter<'_, T> {
//         GridIter {
//             grid_iter: self.items.iter(),
//             width: self.width,
//             height: self.height,
//         }
//     }
//     pub fn iter_mut(&mut self) -> GridIterMut<'_, T> {
//         GridIterMut {
//             grid_iter: self.items.iter_mut(),
//             width: self.width,
//             height: self.height,
//         }
//     }
//     pub fn row(&self, y: i32, z: i32) -> RowIter<'_, T> {
//         assert!(self.is_bounds((0, y, z)));
//         let start_idx = (y as usize * self.width as usize);
//         let end_idx = start_idx + self.width as usize;

//         RowIter {
//             row_iter: self.items[start_idx..end_idx].iter(),
//             idx: y,
//         }
//     }
//     pub fn row_mut(&mut self, y: i32, z: i32) -> RowIterMut<'_, T> {
//         assert!(self.is_bounds((0, y, z)));
//         let start_idx = y  * self.width;
//         let end_idx = start_idx + self.width;

//         RowIterMut {
//             row_iter: self.items[start_idx as usize..end_idx as usize].iter_mut(),
//             idx: y,
//         }
//     }
// }