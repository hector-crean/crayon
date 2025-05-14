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
                    // Store position in the node state
                    let node_idx = graph.add_node(NodeState::with_position(
                        T::default(), 
                        Orientation::default(), 
                        (x, y, z)
                    ));
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

    /// Returns a nicely formatted string representation of the graph
    pub fn pretty_print(&self) -> String {
        use std::fmt::Write;
        let mut output = String::new();
        
        let node_count = self.node_count();
        let edge_count = self.edge_count();
        
        writeln!(&mut output, "WFCGraph with {} nodes and {} edges", node_count, edge_count).unwrap();
        writeln!(&mut output, "--------------------------------------------").unwrap();
        
        // Find max dimensions to determine grid size
        let mut max_x = 0;
        let mut max_y = 0;
        let mut max_z = 0;
        
        for idx in self.node_indices() {
            if let Some(state) = self.node_weight(idx) {
                let (x, y, z) = state.position;
                max_x = max_x.max(x);
                max_y = max_y.max(y);
                max_z = max_z.max(z);
            }
        }
        
        // Add 1 to max dimensions to get size
        let grid_x = max_x + 1;
        let grid_y = max_y + 1;
        let grid_z = max_z + 1;
        
        writeln!(&mut output, "Grid dimensions: {}x{}x{}", grid_x, grid_y, grid_z).unwrap();
        
        // Add a visual grid representation
        writeln!(&mut output, "\nVisual Grid Representation:").unwrap();
        
        // Create a 3D grid for visualization
        let mut grid_data = vec![vec![vec![None; grid_z]; grid_y]; grid_x];
        
        // Populate the grid with block kinds
        for idx in self.node_indices() {
            if let Some(state) = self.node_weight(idx) {
                let (x, y, z) = state.position;
                grid_data[x][y][z] = Some(state.block.block_kind());
            }
        }
        
        // For each layer (z-axis)
        for z in 0..grid_z {
            writeln!(&mut output, "\nLayer {}", z).unwrap();
            
            // Draw horizontal line
            for _ in 0..grid_x + 2 {
                write!(&mut output, "+-").unwrap();
            }
            writeln!(&mut output, "+").unwrap();
            
            // Draw each row (y-axis)
            for y in 0..grid_y {
                write!(&mut output, "|").unwrap();
                
                // Draw each cell (x-axis)
                for x in 0..grid_x {
                    // Get the block at this position and display a symbol
                    match grid_data[x][y][z] {
                        Some(block_kind) => {
                            let symbol = match block_kind {
                                block3d_core::block::BlockKind::Wall => "W",
                                block3d_core::block::BlockKind::Floor => "F",
                                block3d_core::block::BlockKind::Door => "D",
                                block3d_core::block::BlockKind::Window => "X",
                                block3d_core::block::BlockKind::Ceiling => "C",
                                block3d_core::block::BlockKind::Column => "O",
                                block3d_core::block::BlockKind::Stairs => "S",
                                block3d_core::block::BlockKind::Void => "V",
                            };
                            write!(&mut output, "{}|", symbol).unwrap();
                        },
                        None => write!(&mut output, " |").unwrap(),
                    }
                }
                writeln!(&mut output).unwrap();
                
                // Draw horizontal line
                for _ in 0..grid_x + 2 {
                    write!(&mut output, "+-").unwrap();
                }
                writeln!(&mut output, "+").unwrap();
            }
        }
        
        // Detailed node information
        writeln!(&mut output, "\nDetailed Node Information:").unwrap();
        for idx in self.node_indices() {
            if let Some(state) = self.node_weight(idx) {
                // Use stored position
                let (x, y, z) = state.position;
                
                // Node info
                writeln!(&mut output, "Node {}: Position ({}, {}, {})", idx.index(), x, y, z).unwrap();
                // Get the actual block size using the size() method
                let (width, height, depth) = state.block.size();
                writeln!(&mut output, "  - Block: {} ({}, {}, {})", state.block.block_kind(), width, height, depth).unwrap();
                writeln!(&mut output, "  - Orientation: {:?}", state.orientation).unwrap();
                
                // List faces briefly
                let face_count = state.block.faces().count();
                writeln!(&mut output, "  - Faces: {} interfaces", face_count).unwrap();
                
                // Add a separator between nodes
                writeln!(&mut output, "  ---").unwrap();
            }
        }
        
        // Add connectivity information
        writeln!(&mut output, "\nConnectivity Summary:").unwrap();
        let mut connections_by_type = std::collections::HashMap::new();
        
        for edge in self.edge_indices() {
            if let Some((a, b)) = self.edge_endpoints(edge) {
                if let (Some(state_a), Some(state_b)) = (self.node_weight(a), self.node_weight(b)) {
                    let key = format!("{} -> {}", state_a.block.block_kind(), state_b.block.block_kind());
                    *connections_by_type.entry(key).or_insert(0) += 1;
                }
            }
        }
        
        for (connection_type, count) in connections_by_type {
            writeln!(&mut output, "  - {}: {} connections", connection_type, count).unwrap();
        }
        
        output
    }
}
