
use petgraph::graph::NodeIndex;
use std::collections::{HashMap, HashSet};





/// Spatial grid for faster collision detection
pub struct SpatialGrid {
    cells: HashMap<(i32, i32, i32), Vec<NodeIndex>>,
    cell_size: f32,
}

impl SpatialGrid {
    pub fn new(cell_size: f32) -> Self {
        Self {
            cells: HashMap::new(),
            cell_size,
        }
    }
    
    pub fn grid_coords(&self, position: (f32, f32, f32)) -> (i32, i32, i32) {
        (
            (position.0 / self.cell_size).floor() as i32,
            (position.1 / self.cell_size).floor() as i32,
            (position.2 / self.cell_size).floor() as i32,
        )
    }
    
    pub fn add_node(&mut self, node_idx: NodeIndex, position: (f32, f32, f32), size: (f32, f32, f32)) {
        // Add to all cells the node overlaps
        let min_cell = self.grid_coords(position);
        let max_cell = self.grid_coords((
            position.0 + size.0,
            position.1 + size.1,
            position.2 + size.2,
        ));
        
        for x in min_cell.0..=max_cell.0 {
            for y in min_cell.1..=max_cell.1 {
                for z in min_cell.2..=max_cell.2 {
                    self.cells.entry((x, y, z)).or_insert_with(Vec::new).push(node_idx);
                }
            }
        }
    }
    
    pub fn potential_collisions(&self, position: (f32, f32, f32), size: (f32, f32, f32)) -> Vec<NodeIndex> {
        let mut result = HashSet::new();
        
        let min_cell = self.grid_coords(position);
        let max_cell = self.grid_coords((
            position.0 + size.0,
            position.1 + size.1,
            position.2 + size.2,
        ));
        
        for x in min_cell.0..=max_cell.0 {
            for y in min_cell.1..=max_cell.1 {
                for z in min_cell.2..=max_cell.2 {
                    if let Some(nodes) = self.cells.get(&(x, y, z)) {
                        result.extend(nodes.iter());
                    }
                }
            }
        }
        
        result.into_iter().collect()
    }
    
    pub fn remove_node(&mut self, node_idx: NodeIndex) {
        for nodes in self.cells.values_mut() {
            nodes.retain(|&n| n != node_idx);
        }
        
        // Clean up empty cells
        self.cells.retain(|_, nodes| !nodes.is_empty());
    }
}
