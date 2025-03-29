use std::collections::HashSet;
use block3d_algorithm::wfc::{graph::WFCGraph, heuristics::weighted_random_heuristic::WeightedRandomHeuristic, solver::WFCSolver};
use block3d_core::block::{lego_block::LegoBlock, Block3D, BlockKind};






fn main() {
    let dimensions = (2, 2, 1);

    let graph = WFCGraph::<Block3D>::grid_graph(dimensions);
    
    // Create a set of initial blocks
    let mut block_set = HashSet::<Block3D>::new();
    
    // Add 1x1x1 blocks in different orientations
    block_set.insert(Block3D::Lego(LegoBlock::new((1, 1, 1), BlockKind::Wall, vec![])));
    block_set.insert(Block3D::Lego(LegoBlock::new((1, 1, 1), BlockKind::Floor, vec![])));
    block_set.insert(Block3D::Lego(LegoBlock::new((1, 1, 1), BlockKind::Door, vec![])));
    block_set.insert(Block3D::Lego(LegoBlock::new((1, 1, 1), BlockKind::Window, vec![])));


    let invariants = vec![];
    let heuristic = Box::new(WeightedRandomHeuristic);
    let mut solver = WFCSolver::new(graph, block_set, invariants, heuristic, vec![]);
 
    match solver.solve() {
        Ok(solution) => {
            println!("Solution found!");
            println!("Solution graph:");
            println!("{:?}", solver.graph);
        },
        Err(e) => println!("Error: {}", e),
    }
}
