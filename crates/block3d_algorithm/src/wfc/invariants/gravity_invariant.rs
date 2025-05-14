use block3d_core::block::Block3DLike;
use petgraph::graph::NodeIndex;

use crate::wfc::invariants::WFCInvariant;
use crate::wfc::solver::{WFCSolver, state::NodeState};

pub struct GravityInvariant;

impl<T: Block3DLike> WFCInvariant<T> for GravityInvariant {
    fn check(&self, node: NodeIndex, state: &NodeState<T>, solver: &WFCSolver<T>) -> bool {
        // Extract the node's coordinates by finding its neighbors
        let graph = &solver.graph;
        
        // Check if this is a bottom layer node (y-coordinate is 0)
        // To determine this, we'll check if it has neighbors below it
        let has_node_below = graph.neighbors(node)
            .any(|neighbor| {
                // Find if any neighbor is below this node
                // In a grid, a node below will have a lower y coordinate
                // Since we don't have explicit coordinates, we'll rely on edge directions
                // For this implementation, we'll consider that edges going "down" are those
                // where y increases (based on grid_graph implementation)
                
                // Check if this neighbor is below by comparing indices
                // This is a simplified approach assuming grid_graph creates indices in row-major order
                neighbor.index() > node.index() && 
                (neighbor.index() - node.index()) <= graph.node_count() / 3
            });
            
        if !has_node_below {
            // This is a bottom layer node, always valid
            return true;
        }
        
        // For nodes not on the bottom layer, check if they have support
        // A node has support if it has neighbors below it that are collapsed
        let has_support = graph.neighbors(node)
            .filter(|neighbor| {
                // Same logic as above to find nodes below
                neighbor.index() > node.index() && 
                (neighbor.index() - node.index()) <= graph.node_count() / 3
            })
            .any(|below_neighbor| {
                // Check if the below neighbor is collapsed
                solver.collapsed.contains(&below_neighbor)
            });
            
        has_support
    }

    fn propagate(&self, node: NodeIndex, solver: &WFCSolver<T>) -> Vec<NodeIndex> {
        // When a node is collapsed, we need to propagate constraints to nodes above it
        let graph = &solver.graph;
        
        // Find nodes above the current node
        graph.neighbors(node)
            .filter(|neighbor| {
                // Nodes above will have a lower index in our grid layout
                // (opposite of the check logic)
                neighbor.index() < node.index() && 
                (node.index() - neighbor.index()) <= graph.node_count() / 3
            })
            .collect()
    }
}