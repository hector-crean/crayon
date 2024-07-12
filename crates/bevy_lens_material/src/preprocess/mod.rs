use bevy::{
    prelude::*,
    render::{
        extract_component::{ExtractComponent, ExtractComponentPlugin},
        graph::CameraDriverLabel,
        render_asset::RenderAssets,
        render_graph::RenderGraph,
        render_resource::{
            BindGroup, BindGroupEntries, BindGroupLayout, CachedComputePipelineId, PipelineCache,
            ShaderType,
        },
        renderer::RenderDevice,
        texture::GpuImage,
        Render, RenderApp, RenderSet,
    },
};

#[derive(Clone)]
pub(crate) enum PreprocessTaskType {
    Split {
        tile: Handle<Image>,
        top_left: Vec2,
        bottom_right: Vec2,
    },
    Stitch {
        neighbour_nodes: [AtlasNode; 8],
    },
    Downsample {
        child_nodes: [AtlasNode; 4],
    },
    Save,
    Barrier,
}

/// The global coordinate and identifier of a node.
#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq, ShaderType)]
pub struct NodeCoordinate {
    /// The side of the cube sphere the node is located on.
    pub side: u32,
    /// The lod of the node, where 0 is the highest level of detail with the smallest size
    /// and highest resolution
    pub lod: u32,
    /// The x position of the node in node sizes.
    pub x: u32,
    /// The y position of the node in node sizes.
    pub y: u32,
}

#[derive(Copy, Clone, Debug, Default, ShaderType)]
pub struct AtlasNode {
    pub(crate) coordinate: NodeCoordinate,
    #[size(16)]
    pub(crate) atlas_index: u32,
}

#[derive(Copy, Clone, Debug, Default)]
pub struct AtlasNodeAttachment {
    pub(crate) coordinate: NodeCoordinate,
    pub(crate) atlas_index: u32,
    pub(crate) attachment_index: u32,
}

#[derive(Clone)]
pub(crate) struct PreprocessTask {
    pub(crate) node: AtlasNodeAttachment,
    pub(crate) task_type: PreprocessTaskType,
}

enum LensDistortionState {
    Loading,
    Processing,
    Finished,
}

struct LensDistortionNode {
    state: GameOfLifeState,
}

impl Default for LensDistortionNode {
    fn default() -> Self {
        Self {
            state: LensDistortionState::Loading,
        }
    }
}

impl render_graph::Node for LensDistortionNode {
    fn update(&mut self, world: &mut World) {
        let pipeline = world.resource::<LensDistortionPipeline>();
        let pipeline_cache = world.resource::<PipelineCache>();

        // if the corresponding pipeline has loaded, transition to the next stage
        match self.state {
            LensDistortionState::Loading => {
                match pipeline_cache.get_compute_pipeline_state(pipeline.init_pipeline) {
                    CachedPipelineState::Ok(_) => {
                        self.state = LensDistortionState::Init;
                    }
                    CachedPipelineState::Err(err) => {
                        panic!("Initializing assets/{SHADER_ASSET_PATH}:\n{err}")
                    }
                    _ => {}
                }
            }
            LensDistortionState::Init => {
                if let CachedPipelineState::Ok(_) =
                    pipeline_cache.get_compute_pipeline_state(pipeline.update_pipeline)
                {
                    self.state = LensDistortionState::Update(1);
                }
            }
            GameOfLifeState::Update(0) => {
                self.state = GameOfLifeState::Update(1);
            }
            GameOfLifeState::Update(1) => {
                self.state = GameOfLifeState::Update(0);
            }
            GameOfLifeState::Update(_) => unreachable!(),
        }
    }

    fn run(
        &self,
        _graph: &mut render_graph::RenderGraphContext,
        render_context: &mut RenderContext,
        world: &World,
    ) -> Result<(), render_graph::NodeRunError> {
        let bind_groups = &world.resource::<GameOfLifeImageBindGroups>().0;

        let pipeline_cache = world.resource::<PipelineCache>();
        let pipeline = world.resource::<LensDistortionPipeline>();

        let mut pass = render_context
            .command_encoder()
            .begin_compute_pass(&ComputePassDescriptor::default());

        // select the pipeline based on the current state
        match self.state {
            GameOfLifeState::Loading => {}
            GameOfLifeState::Init => {
                let init_pipeline = pipeline_cache
                    .get_compute_pipeline(pipeline.init_pipeline)
                    .unwrap();
                pass.set_bind_group(0, &bind_groups[0], &[]);
                pass.set_pipeline(init_pipeline);
                pass.dispatch_workgroups(SIZE.0 / WORKGROUP_SIZE, SIZE.1 / WORKGROUP_SIZE, 1);
            }
            GameOfLifeState::Update(index) => {
                let update_pipeline = pipeline_cache
                    .get_compute_pipeline(pipeline.update_pipeline)
                    .unwrap();
                pass.set_bind_group(0, &bind_groups[index], &[]);
                pass.set_pipeline(update_pipeline);
                pass.dispatch_workgroups(SIZE.0 / WORKGROUP_SIZE, SIZE.1 / WORKGROUP_SIZE, 1);
            }
        }

        Ok(())
    }
}

#[derive(Default, ShaderType)]
pub struct LensDistortionMeta {
    k: i32,
}

#[derive(Resource)]
pub struct LensDistortionPipeline {
    texture_bind_group_layout: BindGroupLayout,
    init_pipeline: CachedComputePipelineId,
    update_pipeline: CachedComputePipelineId,
}

#[derive(Clone, Component, ExtractComponent)]
pub struct LensDistortion {
    texture_a: Handle<Image>,
    texture_b: Handle<Image>,
}

#[derive(Component)]
struct LensDistortionBindGroups([BindGroup; 2]);

impl LensDistortionBindGroups {
    fn prepare(
        mut commands: Commands,
        pipeline: Res<LensDistortionPipeline>,
        gpu_images: Res<RenderAssets<GpuImage>>,
        query: Query<(Entity, &LensDistortion)>,
        render_device: Res<RenderDevice>,
    ) {
        let (entity, game_of_life_images) = query.single();
        let view_a = gpu_images.get(&game_of_life_images.texture_a).unwrap();
        let view_b = gpu_images.get(&game_of_life_images.texture_b).unwrap();
        let bind_group_0 = render_device.create_bind_group(
            None,
            &pipeline.texture_bind_group_layout,
            &BindGroupEntries::sequential((&view_a.texture_view, &view_b.texture_view)),
        );
        let bind_group_1 = render_device.create_bind_group(
            None,
            &pipeline.texture_bind_group_layout,
            &BindGroupEntries::sequential((&view_b.texture_view, &view_a.texture_view)),
        );
        commands
            .entity(entity)
            .insert(LensDistortionBindGroups([bind_group_0, bind_group_1]));
    }
}

pub struct LensDistortionPreprocessPlugin;

impl Plugin for LensDistortionPreprocessPlugin {
    fn build(&self, app: &mut App) {
        // Extract the game of life image component from the main world into the render world
        // for operation on by the compute shader and display on the sprite.
        app.add_plugins((ExtractComponentPlugin::<LensDistortion>::default()));

        let render_app = app.sub_app_mut(RenderApp);
        render_app.add_systems(
            Render,
            LensDistortionBindGroups::prepare.in_set(RenderSet::PrepareBindGroups),
        );
        let mut render_graph = render_app.world_mut().resource_mut::<RenderGraph>();
        render_graph.add_node(GameOfLifeLabel, GameOfLifeNode::default());
        render_graph.add_node_edge(GameOfLifeLabel, bevy::render::graph::CameraDriverLabel);
    }
    fn finish(&self, _app: &mut App) {
        let render_app = app.sub_app_mut(RenderApp);
        render_app.init_resource::<LensDistortionPipeline>();
    }
}
