use std::ops::{Deref, DerefMut};

use block3d_core::{block::Block3DLike, Orientation};
use petgraph::{graph::NodeIndex, Directed, Graph};

use super::solver::state::{EdgeState, NodeState};



#[derive(Debug, Clone)]
pub struct WFCGraph<T: Block3DLike>(pub Graph<NodeState<T>, EdgeState, Directed>);


impl<T: Block3DLike> Deref for WFCGraph<T> {
    type Target = Graph<NodeState<T>, EdgeState, Directed>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T: Block3DLike> DerefMut for WFCGraph<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Block3DLike> Default for WFCGraph<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Block3DLike> WFCGraph<T> {
    pub fn new() -> Self {
        Self(Graph::new())
    }
    pub fn inner(&self) -> &Graph<NodeState<T>, EdgeState, Directed> {
        &self.0
    }
    pub fn grid_graph(dimensions: (usize, usize, usize)) -> Self {
        let mut graph = WFCGraph::new();
        let (width, height, depth) = dimensions;
        
        // Create nodes for each position in the 3D grid
        let mut node_indices = vec![vec![vec![NodeIndex::default(); depth]; height]; width];
        
        // Create all nodes first
        for x in 0..width {
            for y in 0..height {
                for z in 0..depth {
                    let node_idx = graph.add_node(NodeState::new(T::default(), Orientation::default()));
                    node_indices[x][y][z] = node_idx;
                }
            }
        }
        
        // Connect adjacent nodes with edges
        for x in 0..width {
            for y in 0..height {
                for z in 0..depth {
                    let current = node_indices[x][y][z];
                    
                    // Connect to right neighbor
                    if x < width - 1 {
                        let right = node_indices[x + 1][y][z];
                        graph.add_edge(current, right, EdgeState {});
                    }
                    
                    // Connect to bottom neighbor
                    if y < height - 1 {
                        let bottom = node_indices[x][y + 1][z];
                        graph.add_edge(current, bottom, EdgeState {});
                    }
                    
                    // Connect to back neighbor
                    if z < depth - 1 {
                        let back = node_indices[x][y][z + 1];
                        graph.add_edge(current, back, EdgeState {});
                    }
                }
            }
        }
        
        graph
    }
}
