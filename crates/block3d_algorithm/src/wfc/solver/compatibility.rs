use std::collections::HashMap;

use block3d_core::block::Block3DLike;
use petgraph::graph::NodeIndex;

use super::{state::NodeState};


pub struct CompatibilityRule<T: Block3DLike> {
    pub check: Box<dyn Fn(&NodeState<T>, &NodeState<T>) -> bool>,
    pub description: Option<String>,
}
impl<T: Block3DLike> CompatibilityRule<T> {
    pub fn new(check: Box<dyn Fn(&NodeState<T>, &NodeState<T>) -> bool>, description: Option<String>) -> Self {
        Self { check, description }
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