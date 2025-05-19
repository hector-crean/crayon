use bevy::prelude::*;
use bevy::tasks::AsyncComputeTaskPool;
use bevy_crayon_materials::wireframe::VoidMaterial;
use bevy_mod_outline::{OutlineMode, OutlineStencil, OutlineVolume};
use block3d_algorithm::{
    wfc::graph::WFCGraph, wfc::heuristics::weighted_random_heuristic::WeightedRandomHeuristic,
    wfc::solver::WFCSolver,
};
use block3d_core::block::Block3DLike;
use block3d_core::{
    block::{lego_block::LegoBlock, Block3D, BlockKind},
    connection::{ConnectorInterface, OrientedInterface},
    face::Face,
    Orientation,
};
use crossbeam_channel::{bounded, Receiver};
use std::collections::{HashMap, HashSet};

// Component to store block information for debugging
#[derive(Component)]
struct BlockInfo {
    kind: BlockKind,
    position: (usize, usize, usize),
    orientation: Orientation,
    size: (u32, u32, u32),
}

impl BlockInfo {
    fn new(kind: BlockKind, position: (usize, usize, usize), orientation: Orientation, size: (u32, u32, u32)) -> Self {
        Self { kind, position, orientation, size }
    }
}


// Helper function to get block colors
fn get_block_color(kind: BlockKind) -> Color {
    match kind {
        BlockKind::Wall => Color::rgb(0.75, 0.75, 0.75),
        BlockKind::Floor => Color::rgb(0.55, 0.27, 0.07),
        BlockKind::Window => Color::rgb(0.75, 0.75, 0.75),
        BlockKind::Door => Color::rgb(0.75, 0.75, 0.75),
        BlockKind::Ceiling => Color::rgb(0.75, 0.75, 0.75),
        BlockKind::Void => Color::NONE,
        _ => Color::rgb(0.75, 0.75, 0.75),
    }
}

const SHAPES_X_EXTENT: f32 = 14.0;
const EXTRUSION_X_EXTENT: f32 = 16.0;
const Z_EXTENT: f32 = 5.0;


#[derive(Resource, Deref)]
struct WFCReceiver(Receiver<WFCSolveComplete>);

#[derive(Resource, Deref)]
struct WFCSender(crossbeam_channel::Sender<WFCSolveComplete>);

// Resource to store the current WFC graph
#[derive(Resource, Default, Clone)] // Clone is for easy access in the async task
pub struct CurrentWFCGraph(pub Option<WFCGraph<Block3D>>);

// Resource to store the block set used for WFC
#[derive(Resource, Clone)]
pub struct WFCBlockSet(pub HashSet<Block3D>);

impl Default for WFCBlockSet {
    fn default() -> Self {
        Self(WFCPlugin::default_lego_set())
    }
}

#[derive(Event)]
pub enum WFCSolveRequest {
    // Initialize the grid with dimensions and possible blocks
    Initialize {
        dimensions: (usize, usize, usize),
        block_set: HashSet<Block3D>,
    },

    // Fix a specific block at a position
    PlaceBlock {
        position: (usize, usize, usize),
        block: Block3D, // The specific block to place
    },

    // Get possible blocks for a position (useful for hover preview)
    GetPossibleBlocks {
        position: (usize, usize, usize),
    },

    // Collapse a specific position using WFC's heuristics
    CollapseNode {
        position: (usize, usize, usize),
    },

    // Clear a previously placed block
    ClearNode {
        position: (usize, usize, usize),
    },

    // Optional: Solve a specific region
    SolveRegion {
        min_bound: (usize, usize, usize),
        max_bound: (usize, usize, usize),
    },
}

#[derive(Event)]
pub enum WFCSolveComplete {
    // For Initialize and general solving operations
    GraphUpdated {
        result: Result<WFCGraph<Block3D>, String>,
    },

    // For GetPossibleBlocks queries
    PossibleBlocks {
        position: (usize, usize, usize),
        blocks: Result<Vec<Block3D>, String>,
    },
}

#[derive(Event)]
pub struct UpdateBlockSet {
    pub block_set: HashSet<Block3D>,
}

pub struct WFCPlugin {
    initial_block_set: HashSet<Block3D>,
}

impl Default for WFCPlugin {
    fn default() -> Self {
        Self { initial_block_set: Self::default_lego_set() }
    }
}

impl WFCPlugin {
    pub fn new(initial_block_set: HashSet<Block3D>) -> Self {
        Self { initial_block_set }
    }
    pub fn default_lego_set() -> HashSet<Block3D> {
        let mut block_set = HashSet::new();

        // Create standard faces
        let stud_face = Face::new(OrientedInterface {
            interface: ConnectorInterface::Stud,
            orientation: Orientation::O0,
        });
        let tube_face = Face::new(OrientedInterface {
            interface: ConnectorInterface::Tube,
            orientation: Orientation::O0,
        });

        // Standard 1x1x1 blocks
        for kind in [
            BlockKind::Wall,
            BlockKind::Floor,
            BlockKind::Door,
            BlockKind::Window,
        ] {
            block_set.insert(Block3D::Lego(LegoBlock::new(
                (1, 1, 1),
                kind,
                vec![stud_face.clone(), tube_face.clone()],
            )));
        }

        // 2x1x1 blocks
        for kind in [BlockKind::Wall, BlockKind::Floor] {
            block_set.insert(Block3D::Lego(LegoBlock::new(
                (2, 1, 1),
                kind,
                vec![stud_face.clone(), tube_face.clone()],
            )));
        }

        block_set
    }

    fn initialise_wfc(
        block_set: Res<WFCBlockSet>,
        mut solve_requests: EventWriter<WFCSolveRequest>,
    ) {
        info!("Initialising WFC");
        // Use a 3x3x3 grid for a more interesting structure
        solve_requests.send(WFCSolveRequest::Initialize {
            dimensions: (2, 2, 2), // Increased depth for more complex structures
            block_set: block_set.0.clone(),
        });
    }

    fn handle_block_set_updates(
        mut update_events: EventReader<UpdateBlockSet>,
        mut block_set: ResMut<WFCBlockSet>,
        current_graph: Res<CurrentWFCGraph>,
    ) {
        for event in update_events.read() {
            // Optionally validate the new block set
            if let Some(graph) = &current_graph.0 {
                // Maybe check if the new block set is compatible with the current graph state
                // Or trigger a graph reset if it's not compatible
                
                info!("Updating block set while graph exists - this may require reinitializing the graph");
            }

            block_set.0 = event.block_set.clone();
        }
    }

    fn handle_wfc_requests(
        mut solve_requests: EventReader<WFCSolveRequest>,
        sender: Res<WFCSender>,
        mut wfc_block_set: ResMut<WFCBlockSet>, // Access the block set resource
        current_graph: Res<CurrentWFCGraph>, // Access the current graph
    ) {
        let thread_pool = AsyncComputeTaskPool::get();

        for request in solve_requests.read() {
            match request {
                WFCSolveRequest::Initialize {
                    dimensions,
                    block_set,
                } => {
                    wfc_block_set.0 = block_set.clone(); // Store the block set
                    let block_set_clone = block_set.clone(); // Clone for the async task
                    let dimensions = dimensions.clone();
                    let tx = sender.clone();

                    thread_pool
                        .spawn(async move {
                            let graph = WFCGraph::<Block3D>::grid_graph(dimensions);
                            let heuristic = Box::new(WeightedRandomHeuristic);
                            let invariants = vec![];

                            let solver = WFCSolver::new(
                                graph,
                                block_set_clone, // Use the cloned block set
                                invariants,
                                heuristic,
                                vec![],
                            );

                            // Just initialize the graph without solving
                            let result = Ok(solver.graph);

                            tx.send(WFCSolveComplete::GraphUpdated { result }).ok();
                        })
                        .detach();
                }
                WFCSolveRequest::PlaceBlock { position, block } => {
                    if let Some(graph) = &current_graph.0 {
                        let graph_clone = graph.clone();
                        let block_clone = block.clone();
                        let position_clone = position.clone();
                        let tx = sender.clone();
                        let block_set = wfc_block_set.0.clone();

                        thread_pool
                            .spawn(async move {
                                let graph = graph_clone;
                                let heuristic = Box::new(WeightedRandomHeuristic);
                                let invariants = vec![];

                                let mut solver = WFCSolver::new(
                                    graph,
                                    block_set,
                                    invariants,
                                    heuristic,
                                    vec![],
                                );

                                let result = solver.set_node_at_position(position_clone, block_clone)
                                    .map(|_| solver.graph).map_err(|e| e.to_string());
                                

                                tx.send(WFCSolveComplete::GraphUpdated { result }).ok();
                            })
                            .detach();
                    } else {
                        error!("Cannot place block: WFC graph not initialized");
                    }
                }
                WFCSolveRequest::GetPossibleBlocks { position } => {
                    if let Some(graph) = &current_graph.0 {
                        let graph_clone = graph.clone();
                        let position_clone = position.clone();
                        let tx = sender.clone();
                        let block_set = wfc_block_set.0.clone();

                        
                    } else {
                        error!("Cannot get possible blocks: WFC graph not initialized");
                    }
                }
                WFCSolveRequest::CollapseNode { position } => {
                    if let Some(graph) = &current_graph.0 {
                        let graph_clone = graph.clone();
                        let position_clone = position.clone();
                        let tx = sender.clone();
                        let block_set = wfc_block_set.0.clone();

                        thread_pool
                            .spawn(async move {
                                let mut graph = graph_clone;
                                let heuristic = Box::new(WeightedRandomHeuristic);
                                let invariants = vec![];

                                let mut solver = WFCSolver::new(
                                    graph,
                                    block_set,
                                    invariants,
                                    heuristic,
                                    vec![],
                                );

                                // Find the node at the given position
                                match solver.collapse_node_at_position(position_clone) {
                                    Ok(_) => {
                                        tx.send(WFCSolveComplete::GraphUpdated { result: Ok(solver.graph) }).ok();
                                    }
                                    Err(e) => {
                                        tx.send(WFCSolveComplete::GraphUpdated { result: Err(e.to_string()) }).ok();
                                    }
                                }

                            })
                            .detach();
                    } else {
                        error!("Cannot collapse node: WFC graph not initialized");
                    }
                }
                WFCSolveRequest::ClearNode { position } => {
                    if let Some(graph) = &current_graph.0 {
                        let graph_clone = graph.clone();
                        let position_clone = position.clone();
                        let tx = sender.clone();
                        let block_set = wfc_block_set.0.clone();

                       
                    } else {
                        error!("Cannot clear node: WFC graph not initialized");
                    }
                }
                WFCSolveRequest::SolveRegion {
                    min_bound,
                    max_bound,
                } => {
                    if let Some(graph) = &current_graph.0 {
                        let graph_clone = graph.clone();
                        let min_bound = min_bound.clone();
                        let max_bound = max_bound.clone();
                        let tx = sender.clone();
                        let block_set = wfc_block_set.0.clone();

                      
                    } else {
                        error!("Cannot solve region: WFC graph not initialized");
                    }
                }
            }
        }
    }

    // Modify process_wfc_results to update the CurrentWFCGraph resource
    fn forward_wfc_solve_complete(
        receiver: Res<WFCReceiver>,
        mut solve_complete: EventWriter<WFCSolveComplete>,
        mut current_graph: ResMut<CurrentWFCGraph>, // Access the graph resource
    ) {
        for event_data in receiver.try_iter() {
            match &event_data {
                WFCSolveComplete::GraphUpdated { result } => match result {
                    Ok(graph) => {
                        current_graph.0 = Some(graph.clone());

                    }
                    Err(e) => {
                        error!("Error updating WFC graph: {}", e);
                    }
                },
                WFCSolveComplete::PossibleBlocks { position, blocks } => {
                    // Handle PossibleBlocks event
                }
            }
            solve_complete.send(event_data);
        }
    }

    // New system just for logging and debugging
    fn log_wfc_results(mut complete_events: EventReader<WFCSolveComplete>) {
        for event in complete_events.read() {
            match event {
                WFCSolveComplete::GraphUpdated { result } => match result {
                    Ok(graph) => {
                        // Create a summary of block types
                        let mut block_type_counts = HashMap::new();

                        for node_idx in graph.node_indices() {
                            if let Some(node_state) = graph.node_weight(node_idx) {
                                let block_kind = node_state.block.block_kind();
                                *block_type_counts.entry(block_kind).or_insert(0) += 1;
                            }
                        }

                        // Log summary and detailed info
                        info!("WFC solution found!");
                        info!("---------------------");
                        info!("Total nodes: {}", graph.node_count());

                        for (block_kind, count) in &block_type_counts {
                            info!("  - {}: {} blocks", block_kind, count);
                        }

                        // Detailed graph info
                        info!("\nDetailed graph structure:\n{}", graph.pretty_print());
                    }
                    Err(e) => {
                        error!("WFC generation failed: {}", e);
                    }
                },
                WFCSolveComplete::PossibleBlocks { position, blocks } => {
                    info!("Possible blocks at position {:?}: {:?}", position, blocks);
                }
            }
        }
    }

    // Separate system for geometry management
    fn update_geometry(
        mut complete_events: EventReader<WFCSolveComplete>,
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut standard_materials: ResMut<Assets<StandardMaterial>>,
        mut void_materials: ResMut<Assets<VoidMaterial>>,
    ) {
        for event in complete_events.read() {
            match event {
                WFCSolveComplete::GraphUpdated { result } => match result {
                    Ok(graph) => {
                        // For each node in the graph, create/update visual representation
                        for node_idx in graph.node_indices() {
                            if let Some(node_state) = graph.node_weight(node_idx) {
                                let (x, y, z) = node_state.position;
                                let block = &node_state.block;
                                let orientation = node_state.orientation;
                                let block_kind = block.block_kind();
                                let size = block.size();

                                let position = Vec3::new(x as f32, y as f32, z as f32);


                                let (width, height, depth) = size;
                                let block_size = Vec3::new(width as f32, height as f32, depth as f32);
                                let mesh = meshes.add(Cuboid::new(block_size.x, block_size.y, block_size.z));

                                let rotation = match orientation {
                                    Orientation::O0 => Quat::from_rotation_y(0.0),
                                    Orientation::O90 => Quat::from_rotation_y(std::f32::consts::FRAC_PI_2),
                                    Orientation::O180 => Quat::from_rotation_y(std::f32::consts::PI),
                                    Orientation::O270 => Quat::from_rotation_y(3.0 * std::f32::consts::FRAC_PI_2),
                                };

                                let entity = commands.spawn((
                                    Mesh3d(mesh),
                                    Transform::from_translation(position).with_rotation(rotation),
                                    GlobalTransform::default(),
                                    BlockInfo::new(block_kind, (x, y, z), orientation, size),
                                    MeshMaterial3d(standard_materials.add(StandardMaterial {
                                        base_color: get_block_color(block_kind),
                                        alpha_mode: AlphaMode::Opaque,
                                        ..default()
                                    })),
                                    OutlineVolume {
                                        visible: match block_kind {
                                            BlockKind::Void => true,
                                            _ => false,
                                        },
                                        colour: Color::srgba(1.0, 0.0, 1.0, 0.3),
                                        width: 15.0,
                                    },
                                    OutlineStencil {
                                        enabled: match block_kind {
                                            BlockKind::Void => true,
                                            _ => false,
                                        },
                                        offset: 0.0,
                                    },
         
                                )).id();
          
                            }
                        }
                    }
                    Err(_) => {} // Geometry errors already logged in log_wfc_results
                },
                WFCSolveComplete::PossibleBlocks { .. } => {
                    // Handle visualization of possible blocks if needed
                }
            }
        }
    }
}

impl Plugin for WFCPlugin {
    fn build(&self, app: &mut App) {
        let (tx, rx) = bounded(1);

        app.add_event::<WFCSolveRequest>()
            .add_event::<WFCSolveComplete>()
            .add_event::<UpdateBlockSet>()
            .insert_resource(WFCReceiver(rx))
            .insert_resource(WFCSender(tx))
            .init_resource::<CurrentWFCGraph>()
            .init_resource::<WFCBlockSet>()
            .add_systems(PostStartup, Self::initialise_wfc)
            .add_systems(
                Update,
                (
                    // First, handle any block set updates
                    Self::handle_block_set_updates
                        .run_if(on_event::<UpdateBlockSet>),

                    // Then handle any new WFC requests
                    Self::handle_wfc_requests,

                    // Process results and update graph state
                    Self::forward_wfc_solve_complete,

                    // Finally, handle visualization and logging
                    (
                        Self::log_wfc_results.run_if(on_event::<WFCSolveComplete>),
                        Self::update_geometry.run_if(resource_exists_and_changed::<CurrentWFCGraph>),
                    )
                        .after(Self::forward_wfc_solve_complete)
                        ,
                )
                    .chain()
            );
    }
}










fn create_and_run_solver<F, T>(
    graph: WFCGraph<Block3D>,
    block_set: HashSet<Block3D>,
    operation: F,
) -> Result<WFCGraph<Block3D>, String>
where
    F: FnOnce(&mut WFCSolver<Block3D>) -> Result<T, String>,
{
    let heuristic = Box::new(WeightedRandomHeuristic);
    let invariants = vec![];

    let mut solver = WFCSolver::new(
        graph,
        block_set,
        invariants,
        heuristic,
        vec![],
    );

    operation(&mut solver).map(|_| solver.graph).map_err(|e| e.to_string())
}


fn spawn_graph_operation<F, G, T>(
    thread_pool: AsyncComputeTaskPool,
    graph: WFCGraph<Block3D>,
    block_set: HashSet<Block3D>,
    tx: crossbeam_channel::Sender<WFCSolveComplete>,
    operation: F,
    complete_mapper: G,
)
where
    F: FnOnce(&mut WFCSolver<Block3D>) -> Result<T, String> + Send + 'static,
    G: FnOnce(Result<WFCGraph<Block3D>, String>) -> WFCSolveComplete + Send + 'static,
    T: Send + 'static,
{
    thread_pool
        .spawn(async move {
            let result = create_and_run_solver(graph, block_set, operation);
            tx.send(complete_mapper(result)).ok();
        })
        .detach();
}