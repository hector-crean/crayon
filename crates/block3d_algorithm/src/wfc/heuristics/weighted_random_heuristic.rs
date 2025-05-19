use std::collections::HashSet;
use rand::{Rng, distributions::WeightedIndex, prelude::Distribution};
use block3d_core::block::{Block3DLike, BlockKind};
use petgraph::graph::NodeIndex;
use crate::wfc::solver::state::NodeState;
use super::Heuristic;

pub struct WeightedRandomHeuristic;

impl<T: Block3DLike> Heuristic<T> for WeightedRandomHeuristic {
    fn select_node_to_collapse(&self, solver_states: &[Vec<NodeState<T>>], collapsed: &HashSet<NodeIndex>) -> Option<NodeIndex> {
        // Find uncollapsed nodes with their entropy (number of possible states)
        let mut candidates: Vec<(NodeIndex, usize)> = solver_states.iter()
            .enumerate()
            .filter_map(|(idx, states)| {
                let node_idx = NodeIndex::new(idx);
                if !collapsed.contains(&node_idx) && !states.is_empty() {
                    Some((node_idx, states.len()))
                } else {
                    None
                }
            })
            .collect();

        if candidates.is_empty() {
            return None;
        }

        // Sort by entropy (number of possible states)
        candidates.sort_by_key(|(_node, entropy)| *entropy);

        // Get the minimum entropy
        let min_entropy = candidates[0].1;

        // Filter to only nodes with minimum entropy
        let min_entropy_candidates: Vec<NodeIndex> = candidates
            .into_iter()
            .filter(|(_, entropy)| *entropy == min_entropy)
            .map(|(node, _)| node)
            .collect();

        // Randomly select from minimum entropy nodes
        let mut rng = rand::thread_rng();
        min_entropy_candidates.get(rng.gen_range(0..min_entropy_candidates.len()))
            .copied()
    }

    fn select_state_for_node(&self, _node: NodeIndex, valid_states: &[NodeState<T>]) -> Option<NodeState<T>> {
        if valid_states.is_empty() {
            return None;
        }

        // Create a weighted selection based on block preferences
        let mut rng = rand::thread_rng();
        
        // Assign weights to different block types
        let weights: Vec<f32> = valid_states.iter().map(|state| {
            match state.block.block_kind() {
                // Prioritize structural elements
                BlockKind::Wall => 10.0,
                BlockKind::Floor => 8.0,
                
                // Special elements are less common
                BlockKind::Door => 3.0,  
                BlockKind::Window => 3.0,
                
                // Other block types
                _ => 5.0,
            }
        }).collect();
        
        // Create a weighted distribution
        if let Ok(dist) = WeightedIndex::new(&weights) {
            let selected_idx = dist.sample(&mut rng);
            return valid_states.get(selected_idx).cloned();
        }
        
        // Fallback to simple random selection if weighting fails
        valid_states.get(rng.gen_range(0..valid_states.len())).cloned()
    }
}