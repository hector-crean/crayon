use block3d_core::{block::Block3DLike, Orientation};
use strum::IntoEnumIterator;
use std::collections::HashMap;
use crate::connection::ConnectionPoint;


#[derive(Clone, Debug)]
pub struct NodeState<T: Block3DLike> {
    pub block: T,
    pub orientation: Orientation,
    pub position: (usize, usize, usize),  // Store x, y, z coordinates
    // New fields for connection-based approach
    pub connections: HashMap<String, ConnectionPoint>,
    pub is_connected: bool,
}

impl<T: Block3DLike> NodeState<T> {
    pub fn new(block: T, orientation: Orientation) -> Self {
        Self {
            block,
            orientation,
            position: (0, 0, 0),  // Default position
            connections: HashMap::new(),
            is_connected: false,
        }
    }
    
    pub fn with_position(block: T, orientation: Orientation, position: (usize, usize, usize)) -> Self {
        let mut state = Self::new(block, orientation);
        state.position = position;
        
        // Initialize connection points based on block's interfaces
        state.initialize_connections();
        
        state
    }
    
    // New method to initialize connection points based on the block's interfaces
    fn initialize_connections(&mut self) {
        // Clear existing connections
        self.connections.clear();
        
        // Get the block's interface definitions
        let faces = self.block.faces();
        
        // Create connection points for each face
        for (index, face) in faces.enumerate() {
            let interface = face.oriented_interface();
            // Create a connection point with a unique ID
            let conn_id = format!("conn_{}", index);
                
            // Calculate position offset based on face position and block size
            let size = self.block.size();
            let pos_offset = match index {
                0 => (0.0, size.1 as f32, 0.0), // Top face
                1 => (0.0, 0.0, 0.0),           // Bottom face
                // Add other faces as needed
                _ => (0.0, 0.0, 0.0),
            };
            
            // Create connection point
            let conn_point = ConnectionPoint::new(
                interface.clone(),
                pos_offset,
            );
            
            self.connections.insert(conn_id, conn_point);
        }
    }
    
    // Check if this node can connect to another node
    pub fn can_connect_to(&self, other: &NodeState<T>) -> Option<(String, String)> {
        for (self_id, self_conn) in &self.connections {
            for (other_id, other_conn) in &other.connections {
                if self_conn.is_compatible_with(&other_conn.interface) {
                    return Some((self_id.clone(), other_id.clone()));
                }
            }
        }
        None
    }
    
    // Calculate world position of the block in floating point for precise positioning
    pub fn world_position(&self) -> (f32, f32, f32) {
        (self.position.0 as f32, self.position.1 as f32, self.position.2 as f32)
    }
    
    // Check if this block collides with another block
    pub fn collides_with(&self, other: &NodeState<T>) -> bool {
        // Simple AABB collision check - can be improved with more precise collision detection
        let self_pos = self.world_position();
        let other_pos = other.world_position();
        
        let self_size = (
            self.block.size().0 as f32,
            self.block.size().1 as f32,
            self.block.size().2 as f32
        );
        
        let other_size = (
            other.block.size().0 as f32,
            other.block.size().1 as f32,
            other.block.size().2 as f32
        );
        
        // AABB collision check
        !(self_pos.0 + self_size.0 <= other_pos.0 ||
          other_pos.0 + other_size.0 <= self_pos.0 ||
          self_pos.1 + self_size.1 <= other_pos.1 ||
          other_pos.1 + other_size.1 <= self_pos.1 ||
          self_pos.2 + self_size.2 <= other_pos.2 ||
          other_pos.2 + other_size.2 <= self_pos.2)
    }
    
    fn orientation_iter() -> impl Iterator<Item = Orientation> {
        Orientation::iter()
    }
}


#[derive(Debug, Clone)]
pub struct EdgeState {
    
}

