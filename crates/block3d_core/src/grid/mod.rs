use grid_position::Grid2DPosition;
pub mod grid_position;
pub mod vecdeque;

pub trait Grid2DLike<'grid> {
    /// The type of item stored in the grid.
    type GridItem: Clone + Copy + 'grid;

    /// The type representing a position in the grid.
    /// Must be convertible to a 3D grid position.
    type Position: Into<Grid2DPosition>;

    /// Iterators for traversing the grid.
    type Iter: Iterator<Item = Self::GridItem>;
    type IterMut: Iterator<Item = &'grid mut Self::GridItem>;

    type RowIter: Iterator<Item = Self::GridItem>;
    type RowIterMut: Iterator<Item = &'grid mut Self::GridItem>;

    type ColumnIter: Iterator<Item = Self::GridItem>;
    type ColumnIterMut: Iterator<Item = &'grid mut Self::GridItem>;


    type NeighborsIter: Iterator<Item = Self::GridItem>;
    type NeighborsIterMut: Iterator<Item = &'grid mut Self::GridItem>;

    type BoundaryIter: Iterator<Item = Self::GridItem>;
    type BoundaryIterMut: Iterator<Item = &'grid mut Self::GridItem>;

    /// Returns the dimensions of the grid as a tuple (width, height, depth).
    /// This enforces the 3D nature of the grid.
    fn dimensions(&self) -> (i32, i32, i32);

    /// Gets an item at the specified position.
    fn get(&self, pos: Self::Position) -> Option<&Self::GridItem>;

    /// Gets a mutable reference to an item at the specified position.
    fn get_mut(&mut self, pos: Self::Position) -> Option<&mut Self::GridItem>;

    /// Sets an item at the specified position.
    fn set(&mut self, pos: Self::Position, item: Self::GridItem);

    /// Returns an iterator over the grid items.
    fn iter(&self) -> Self::Iter;

    /// Returns a mutable iterator over the grid items.
    fn iter_mut(&mut self) -> Self::IterMut;

    /// Returns an iterator over the neighbours of a specified position.
    fn neighbours(&self, pos: Self::Position) -> Self::NeighborsIter;
    fn neighbours_mut(&mut self, pos: Self::Position) -> Self::NeighborsIterMut;

    // /// Returns an iterator over a row at the specified y and z coordinates.
    // fn row(&self, y: i32, z: i32) -> Self::RowIter;

    // /// Returns a mutable iterator over a row at the specified y and z coordinates.
    // fn row_mut(&mut self, y: i32, z: i32) -> Self::RowIterMut;

    // fn depth(&self, z: i32) -> Self::DepthIter;
    // fn depth_mut(&mut self, z: i32) -> Self::DepthIterMut;

    // fn column(&self, x: i32, z: i32) -> Self::ColumnIter;
    // fn column_mut(&mut self, x: i32, z: i32) -> Self::ColumnIterMut;

    // /// Returns an iterator over a layer at the specified z coordinate.
    // fn layer(&self, z: i32) -> Self::LayerIter;
    // fn layer_mut(&mut self, z: i32) -> Self::LayerIterMut;

    // /// Returns an iterator over the boundary items of the grid.
    // fn boundary(&self) -> Self::BoundaryIter;
    // fn boundary_mut(&mut self) -> Self::BoundaryIterMut;
}
