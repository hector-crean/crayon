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

#[derive(Event)]
pub struct WFCSolveComplete {
    pub result: Result<WFCGraph<Block3D>, String>,
}

pub struct WFCPlugin;

#[derive(Resource, Deref)]
struct WFCReceiver(Receiver<WFCSolveComplete>);

#[derive(Resource, Deref)]
struct WFCSender(crossbeam_channel::Sender<WFCSolveComplete>);

impl Plugin for WFCPlugin {
    fn build(&self, app: &mut App) {
        let (tx, rx) = bounded(1);
        
        app
            .add_event::<WFCSolveRequest>()
            .add_event::<WFCSolveComplete>()
            .insert_resource(WFCReceiver(rx))
            .insert_resource(WFCSender(tx))
            .add_systems(Update, (handle_wfc_requests, process_wfc_results));
    }
}

fn handle_wfc_requests(
    mut solve_requests: EventReader<WFCSolveRequest>,
    sender: Res<WFCSender>,
) {
    let thread_pool = AsyncComputeTaskPool::get();

    for request in solve_requests.read() {

        let block_set = request.block_set.clone();
        let dimensions = request.dimensions;
        let tx = sender.clone();

        thread_pool.spawn(async move {
            let graph = WFCGraph::<Block3D>::grid_graph(dimensions);
            let heuristic = Box::new(WeightedRandomHeuristic);
            let invariants = vec![];
            
            let mut solver = WFCSolver::new(
                graph,
                block_set,
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

fn process_wfc_results(
    receiver: Res<WFCReceiver>,
    mut solve_complete: EventWriter<WFCSolveComplete>,
) {
    for result in receiver.try_iter() {
        solve_complete.send(result);
    }
}