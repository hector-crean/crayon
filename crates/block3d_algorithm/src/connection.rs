use block3d_core::connection::OrientedInterface;
use block3d_core::Orientation;
use petgraph::graph::NodeIndex;

/// Represents a connection point on a block
#[derive(Clone, Debug)]
pub struct ConnectionPoint {
    /// The interface type (stud, tube, etc.) and its orientation
    pub interface: OrientedInterface,
    /// Offset from block origin (x,y,z) in local space
    pub position_offset: (f32, f32, f32),
    /// Reference to another node if connected
    pub connected_to: Option<(NodeIndex, String)>,
}

impl ConnectionPoint {
    /// Creates a new connection point
    pub fn new(interface: OrientedInterface, position_offset: (f32, f32, f32)) -> Self {
        Self {
            interface,
            position_offset,
            connected_to: None,
        }
    }
    
    /// Checks if this connection point is compatible with a given interface
    pub fn is_compatible_with(&self, other_interface: &OrientedInterface) -> bool {
        // Basic compatibility check - implement detailed rules based on specific interfaces
        match (&self.interface.interface, &other_interface.interface) {
            (block3d_core::connection::ConnectorInterface::Stud, block3d_core::connection::ConnectorInterface::Tube) => true,
            (block3d_core::connection::ConnectorInterface::Tube, block3d_core::connection::ConnectorInterface::Stud) => true,
            // Add other compatible pairs as needed
            _ => false,
        }
    }
    
    /// Gets the world position of this connection point based on block position and orientation
    pub fn world_position(&self, block_position: (f32, f32, f32), block_orientation: Orientation) -> (f32, f32, f32) {
        // Apply rotation based on block orientation
        let rotated_offset = match block_orientation {
            Orientation::O0 => self.position_offset,
            Orientation::O90 => (self.position_offset.2, self.position_offset.1, -self.position_offset.0),
            Orientation::O180 => (-self.position_offset.0, self.position_offset.1, -self.position_offset.2),
            Orientation::O270 => (-self.position_offset.2, self.position_offset.1, self.position_offset.0),
        };
        
        // Apply translation
        (
            block_position.0 + rotated_offset.0,
            block_position.1 + rotated_offset.1,
            block_position.2 + rotated_offset.2,
        )
    }
} 