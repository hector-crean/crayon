use bevy::{color::palettes, prelude::*};
use bevy_polyline::prelude::{Polyline, PolylineBundle, PolylineHandle, PolylineMaterial, PolylineMaterialHandle};
use std::time::Duration;
use crate::plugins::{pointer_events::DoubleClick, tool::markup_tool::MarkupToolState};


#[derive(Component)]
pub struct DraftPreviewPolyline;

/// The top-level states for the polyline drawing tool, subordinate to `MarkupToolState::Polyline`.
#[derive(Default, SubStates, Debug, Clone, Eq, PartialEq, Hash)]
#[source(MarkupToolState = MarkupToolState::Polyline)]
pub enum PolylineMarkupState {
    /// Waiting for the first click to place the initial point of a new polyline.
    #[default]
    AwaitingFirstClick,

    /// After placing the first point, we keep adding more points with each click until double-click finishes.
    Drafting,

    /// The user has double-clicked to finish the polyline. We might do post-processing or a small delay here.
    Finished,
}

/// An ECS component that tracks the current "lifecycle" of a single polyline entity.
#[derive(Component)]
pub enum PolylineState {
    /// We are still adding points (the user is clicking in the world). 
    Drafting { points: Vec<Vec3> },

    /// The user has finished drafting. This polyline is final (no more points added).
    Completed { points: Vec<Vec3> },

    /// Potential future usage if you want to re-enter a polyline for editing.
    Editing { points: Vec<Vec3> },
}

impl Default for PolylineState {
    fn default() -> Self {
        PolylineState::Drafting { points: vec![] }
    }
}

/// The main plugin that handles creating and updating polylines based on user clicks.
pub struct PolylineMarkupPlugin;

impl Plugin for PolylineMarkupPlugin {
    fn build(&self, app: &mut App) {
        app
            // We want to use these sub-states in the `MarkupToolState::Polyline` context.
            .init_state::<PolylineMarkupState>()
            // Simple plugin setup
            .add_systems(Startup, Self::setup)
            // Systems for the main update stage
            .add_systems(
                Update,
                (
                    // Start drafting on the first user click
                    Self::start_draft_on_first_click
                        .run_if(in_state(PolylineMarkupState::AwaitingFirstClick))
                        // We only respond to single clicks, not double-click
                        .run_if(not(on_event::<Pointer<DoubleClick>>)),

                    // Add points with each click
                    Self::add_points_on_click
                        .run_if(in_state(PolylineMarkupState::Drafting))
                        .run_if(on_event::<Pointer<Click>>)
                        .run_if(not(on_event::<Pointer<DoubleClick>>)),

                    // Log or debug state transitions
                    Self::debug_state_transition
                        .run_if(on_event::<StateTransitionEvent<PolylineMarkupState>>),
                ),
            )
            // We run the finishing logic in PostUpdate so it's a separate pass:
            .add_systems(
                PostUpdate,
                (
                    // If the user double-clicked, complete the polyline
                    Self::finish_draft_on_double_click
                        .run_if(on_event::<Pointer<DoubleClick>>)
                        .run_if(in_state(PolylineMarkupState::Drafting)),

                    // Once we’re in the "Finished" state, we wait a bit before resetting 
                    Self::return_to_idle_after_finish.run_if(in_state(PolylineMarkupState::Finished)),
                ),
            )
            // We update the actual polyline mesh last, so it sees all the changes from above
            .add_systems(
                Last,
                Self::update_polyline_mesh
                    .run_if(in_state(PolylineMarkupState::Drafting)),
            )
            // Clean up polylines that remain incomplete or convert them to "completed"
            .add_systems(OnExit(MarkupToolState::Polyline), Self::cleanup_polylines);
    }
}

impl PolylineMarkupPlugin {
    /// Called once at plugin setup time.
    fn setup() {
        info!("PolylineMarkupPlugin: setup complete");
    }

    /// Logs all state transitions for debugging.
    fn debug_state_transition(
        mut state_reader: EventReader<StateTransitionEvent<PolylineMarkupState>>,
    ) {
        for event in state_reader.read() {
            info!(
                "PolylineMarkupState changed from {:?} to {:?}",
                event.exited, event.entered
            );
        }
    }

    // ────────────────────────────────────────────────────────────────
    //  PHASE 1: Place the first point
    // ────────────────────────────────────────────────────────────────

    /// Spawns a new polyline and transitions from `AwaitingFirstClick` to `Drafting`.
    fn start_draft_on_first_click(
        mut commands: Commands,
        mut pointer_click_events: EventReader<Pointer<Click>>,
        mut next_state: ResMut<NextState<PolylineMarkupState>>,
        mut polylines: ResMut<Assets<Polyline>>,
        mut polyline_materials: ResMut<Assets<PolylineMaterial>>,
    ) {
        for click in pointer_click_events.read() {
            if let Some(position) = click.hit.position {
                // Create the `PolylineState::Drafting` component with the initial point.
                let points = vec![position];

                // Build and store the polyline
                let polyline_handle = polylines.add(Polyline {
                    vertices: points.clone(),
                });
                let material_handle = polyline_materials.add(PolylineMaterial {
                    width: 2.0,
                    color: palettes::tailwind::BLUE_500.into(),
                    perspective: false,
                    depth_bias: -0.0002,
                });

                // Spawn entity with the newly created polyline.
                commands.spawn((
                    PolylineState::Drafting { points },
                    PolylineBundle {
                        polyline: PolylineHandle(polyline_handle),
                        material: PolylineMaterialHandle(material_handle),
                        ..Default::default()
                    },
                ));

                // Transition to the `Drafting` state so subsequent clicks add points.
                next_state.set(PolylineMarkupState::Drafting);
            }
        }
    }

    // ────────────────────────────────────────────────────────────────
    //  PHASE 2: Add more points (one at a time) until double-click
    // ────────────────────────────────────────────────────────────────

    /// When in the `Drafting` state, each single-click adds a new point to the polyline.
    fn add_points_on_click(
        mut pointer_click_events: EventReader<Pointer<Click>>,
        mut polyline_query: Query<&mut PolylineState>,
    ) {
        // We expect only 1 polyline in the "Drafting" state, but if you support multiple,
        // just repeat for each. 
        for mut polyline_state in &mut polyline_query {
            if let PolylineState::Drafting { points } = &*polyline_state {
                let mut new_points = points.clone();

                for click in pointer_click_events.read() {
                    if let Some(position) = click.hit.position {
                        new_points.push(position);
                        info!("Added point: {:?}", position);
                    }
                }

                if new_points.len() > points.len() {
                    *polyline_state = PolylineState::Drafting {
                        points: new_points,
                    };
                }
            }
        }
    }

    /// If we detect a `DoubleClick` while in `Drafting`, we mark the polyline as `Completed`.
    fn finish_draft_on_double_click(
        double_click_events: EventReader<Pointer<DoubleClick>>,
        mut polyline_query: Query<&mut PolylineState>,
        mut next_state: ResMut<NextState<PolylineMarkupState>>,
    ) {
        if double_click_events.is_empty() {
            return;
        }

        for mut polyline_state in &mut polyline_query {
            if let PolylineState::Drafting { points } = &*polyline_state {
                let final_points = points.clone();
                *polyline_state = PolylineState::Completed { points: final_points };

                // Move plugin state to "Finished" after double-click.
                next_state.set(PolylineMarkupState::Finished);
                info!("DoubleClick => Completed polyline");
            }
        }
    }

    // ────────────────────────────────────────────────────────────────
    //  PHASE 3: After finishing, return to idle (AwaitingFirstClick)
    // ────────────────────────────────────────────────────────────────

    /// Once we’re in `Finished`, we can wait a second (or no time) before returning to `AwaitingFirstClick`.
    /// For example, you might show some on-screen message or finalize your data.
    fn return_to_idle_after_finish(
        time: Res<Time>,
        mut next_state: ResMut<NextState<PolylineMarkupState>>,
        mut timer: Local<Timer>,
    ) {
        // If the timer is not yet initialized, do so now.
        if timer.elapsed() == Duration::ZERO && timer.duration().is_zero() {
            timer.set_duration(Duration::from_secs_f32(1.0)); // 1 second
            timer.reset();
        }

        // Tick the timer. Once done, go back to `AwaitingFirstClick`.
        timer.tick(time.delta());
        if timer.finished() {
            timer.reset();
            next_state.set(PolylineMarkupState::AwaitingFirstClick);
            info!("Returning to AwaitingFirstClick after finishing a polyline");
        }
    }

    // ────────────────────────────────────────────────────────────────
    //  SYSTEM: Update the mesh to include newly added points.
    // ────────────────────────────────────────────────────────────────

    /// When the ECS `PolylineState` changes, we update the underlying `Polyline` asset 
    /// so it visually reflects the new points.
    fn update_polyline_mesh(
        query: Query<(&PolylineHandle, &PolylineState), Changed<PolylineState>>,
        mut polylines: ResMut<Assets<Polyline>>,
    ) {
        for (polyline_handle, polyline_state) in &query {
            if let PolylineState::Drafting { points } = polyline_state {
                if let Some(polyline) = polylines.get_mut(&polyline_handle.0) {
                    polyline.vertices = points.clone();
                    info!("Updated draft polyline with {} points", points.len());
                } else {
                    warn!(
                        "Polyline asset not found for handle: {:?}",
                        polyline_handle
                    );
                }
            }
        }
    }

    // ────────────────────────────────────────────────────────────────
    //  On Exit: Clean up or finalize polylines
    // ────────────────────────────────────────────────────────────────

    /// When the `MarkupToolState::Polyline` sub-state machine exits, 
    /// we optionally remove incomplete polylines or finalize them. 
    fn cleanup_polylines(mut commands: Commands, polyline_query: Query<(Entity, &PolylineState)>) {
        for (entity, polyline_state) in &polyline_query {
            match polyline_state {
                PolylineState::Drafting { points } => {
                    if points.is_empty() {
                        // Remove incomplete polylines
                        commands.entity(entity).despawn();
                        info!("Discarded empty polyline draft: {:?}", entity);
                    } else {
                        // Convert to completed for good measure
                        commands
                            .entity(entity)
                            .insert(PolylineState::Completed { points: points.clone() });
                        info!("Forcing incomplete polyline to 'Completed': {:?}", entity);
                    }
                }
                PolylineState::Editing { .. } => {
                    // If your code never transitions editing polylines to another state, 
                    // decide how to handle them here. For now, just log:
                    info!("Cleanup found an 'Editing' polyline, entity: {:?}", entity);
                }
                PolylineState::Completed { .. } => {
                    // Already completed. Nothing to do.
                }
            }
        }
    }
}