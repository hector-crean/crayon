use bevy::prelude::*;
use bevy::tasks::AsyncComputeTaskPool;
use std::collections::HashSet;
use block3d_algorithm::{
    wfc::heuristics::weighted_random_heuristic::WeightedRandomHeuristic, 
    wfc::solver::WFCSolver, 
    wfc::graph::WFCGraph
};
use block3d_core::block::Block3D;
use crossbeam_channel::{bounded, Receiver};

#[derive(Event)]
pub struct WFCSolveRequest {
    pub dimensions: (usize, usize, usize),
    pub block_set: HashSet<Block3D>,
}

// New event for user-triggered collapse
#[derive(Event)]
pub struct UserTriggeredCollapse {
    pub position: (usize, usize, usize), // The (x, y, z) position of the node to collapse
}

#[derive(Event)]
pub struct WFCSolveComplete {
    pub result: Result<WFCGraph<Block3D>, String>,
    // Optional: Add information about which nodes were changed, if needed for partial updates
    // pub changed_nodes: Option<Vec<NodeIndex>>
}

pub struct WFCPlugin;

#[derive(Resource, Deref)]
struct WFCReceiver(Receiver<WFCSolveComplete>);

#[derive(Resource, Deref)]
struct WFCSender(crossbeam_channel::Sender<WFCSolveComplete>);

// Resource to store the current WFC graph
#[derive(Resource, Default, Clone)] // Clone is for easy access in the async task
pub struct CurrentWFCGraph(pub Option<WFCGraph<Block3D>>);

// Resource to store the block set used for WFC
#[derive(Resource, Default, Clone)]
pub struct WFCBlockSet(pub HashSet<Block3D>);

impl Plugin for WFCPlugin {
    fn build(&self, app: &mut App) {
        let (tx, rx) = bounded(1);
        
        app
            .add_event::<WFCSolveRequest>()
            .add_event::<UserTriggeredCollapse>() // Add the new event
            .add_event::<WFCSolveComplete>()
            .insert_resource(WFCReceiver(rx))
            .insert_resource(WFCSender(tx))
            .init_resource::<CurrentWFCGraph>() // Initialize the new resource
            .init_resource::<WFCBlockSet>()      // Initialize the block set resource
            .add_systems(Update, (handle_wfc_requests, handle_user_triggered_collapse, process_wfc_results_and_update_graph)); // Renamed process_wfc_results
    }
}

fn handle_wfc_requests(
    mut solve_requests: EventReader<WFCSolveRequest>,
    sender: Res<WFCSender>,
    mut wfc_block_set: ResMut<WFCBlockSet>, // Access the block set resource
) {
    let thread_pool = AsyncComputeTaskPool::get();

    for request in solve_requests.read() {
        wfc_block_set.0 = request.block_set.clone(); // Store the block set
        let block_set_clone = request.block_set.clone(); // Clone for the async task
        let dimensions = request.dimensions;
        let tx = sender.clone();

        thread_pool.spawn(async move {
            let graph = WFCGraph::<Block3D>::grid_graph(dimensions);
            let heuristic = Box::new(WeightedRandomHeuristic);
            let invariants = vec![];
            
            let mut solver = WFCSolver::new(
                graph,
                block_set_clone, // Use the cloned block set
                invariants,
                heuristic,
                vec![],
            );

            let result = solver.solve()
                .map(|_| solver.graph)
                .map_err(|e| e.to_string());

            tx.send(WFCSolveComplete { result }).ok();
        }).detach();
    }
}

// System to handle user-triggered collapse events
fn handle_user_triggered_collapse(
    mut collapse_events: EventReader<UserTriggeredCollapse>,
    sender: Res<WFCSender>,
    current_graph: Res<CurrentWFCGraph>,
    block_set: Res<WFCBlockSet>,
) {
    let thread_pool = AsyncComputeTaskPool::get();

    for event in collapse_events.read() {
        let position_to_collapse = event.position;
        let tx = sender.clone();
        
        // Clone the current graph and block_set for the async task
        let graph_clone = current_graph.0.clone(); 
        let block_set_clone = block_set.0.clone();

        thread_pool.spawn(async move {
            if let Some(current_graph_instance) = graph_clone {
                let heuristic = Box::new(WeightedRandomHeuristic);
                let invariants = vec![];
                
                let mut solver = WFCSolver::new(
                    current_graph_instance, // Use the current graph state
                    block_set_clone,      // Use the stored block set
                    invariants,
                    heuristic,
                    vec![],
                );

                let node_to_collapse = solver.graph.node_indices().find(|&idx| {
                    solver.graph.node_weight(idx).map_or(false, |state| state.position == position_to_collapse)
                });

                if let Some(node_idx) = node_to_collapse {
                    let result = solver.collapse_specific_node(node_idx)
                        .map(|_| solver.graph) 
                        .map_err(|e| e.to_string());
                    tx.send(WFCSolveComplete { result }).ok();
                } else {
                    let _ = tx.send(WFCSolveComplete { result: Err("Node at position not found".to_string()) });
                }
            } else {
                // Handle case where there's no current graph (e.g., initial state)
                let _ = tx.send(WFCSolveComplete { result: Err("No current WFC graph available".to_string()) });
            }
        }).detach();
    }
}

// Modify process_wfc_results to update the CurrentWFCGraph resource
fn process_wfc_results_and_update_graph(
    receiver: Res<WFCReceiver>,
    mut solve_complete: EventWriter<WFCSolveComplete>,
    mut current_graph: ResMut<CurrentWFCGraph>, // Access the graph resource
) {
    for event_data in receiver.try_iter() {
        match &event_data.result {
            Ok(graph_result) => {
                current_graph.0 = Some(graph_result.clone()); // Update the resource
            }
            Err(_) => {
                // Optionally clear the graph or handle the error
                current_graph.0 = None;
            }
        }
        solve_complete.send(event_data); // Forward the event
    }
}