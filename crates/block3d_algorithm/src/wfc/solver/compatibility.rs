use std::collections::HashMap;

use block3d_core::block::Block3DLike;
use petgraph::graph::NodeIndex;
use block3d_core::connection::ConnectorInterface;
use block3d_core::face::Face;

use super::{state::NodeState};


pub struct CompatibilityRule<T: Block3DLike> {
    pub check: Box<dyn Fn(&NodeState<T>, &NodeState<T>) -> bool>,
    pub description: Option<String>,
}
impl<T: Block3DLike> CompatibilityRule<T> {
    pub fn new(check: Box<dyn Fn(&NodeState<T>, &NodeState<T>) -> bool>, description: Option<String>) -> Self {
        Self { check, description }
    }
    
    /// Creates a compatibility rule that checks if two blocks can connect properly
    pub fn lego_connectivity() -> Self {
        let check = Box::new(|state1: &NodeState<T>, state2: &NodeState<T>| {
            // Simple example: check if the faces can connect
            // In a real implementation, you would check specific connection types
            // based on the block faces and their orientation
            
            // For instance, check if state1 has a stud and state2 has a tube
            let state1_has_stud = state1.block.faces().any(|face| {
                matches!(face.oriented_interface().interface, ConnectorInterface::Stud)
            });
            
            let state2_has_tube = state2.block.faces().any(|face| {
                matches!(face.oriented_interface().interface, ConnectorInterface::Tube)
            });
            
            // Check orientation compatibility (simplified)
            let orientation_compatible = (state1.orientation as u8 + state2.orientation as u8) % 4 == 0;
            
            // Simple compatibility check
            (state1_has_stud && state2_has_tube) && orientation_compatible
        });
        
        Self::new(check, Some("Lego block connectivity rule".to_string()))
    }
}


pub struct CompatibilityTable<T: Block3DLike> {
    rules: Vec<CompatibilityRule<T>>,
    cache: HashMap<(NodeIndex, NodeIndex), bool>,
}

impl<T: Block3DLike> Default for CompatibilityTable<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Block3DLike> CompatibilityTable<T> {
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
            cache: HashMap::new(),
        }
    }

    fn rebuild_cache(&mut self) {
        self.cache.clear();
        // Cache will be rebuilt lazily when checking compatibility
    }

    pub fn add_rule(&mut self, rule: CompatibilityRule<T>) {
        self.rules.push(rule);
        self.rebuild_cache();
    }

    pub fn is_compatible(&mut self, node1: NodeIndex, node2: NodeIndex, state1: &NodeState<T>, state2: &NodeState<T>) -> bool {
        if let Some(&cached_result) = self.cache.get(&(node1, node2)) {
            return cached_result;
        }

        let result = self.rules.iter().all(|rule| (rule.check)(state1, state2));
        self.cache.insert((node1, node2), result);
        result
    }
}