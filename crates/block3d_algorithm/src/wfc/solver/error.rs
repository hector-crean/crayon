use petgraph::graph::NodeIndex;
use thiserror::Error;



#[derive(Debug, Error)]
pub enum WFCError {
    #[error("No valid states available for node {0:?}")]
    NoValidStates(NodeIndex),
    
    #[error("Propagation failed: {0:?}")]
    PropagationFailed(String),
    
    #[error("Incomplete collapse for node {0:?}")]
    IncompleteCollapse(NodeIndex),
    
    #[error("Invalid state: {0:?}")]
    InvalidState(String),

    #[error("No valid states after applying invariants for node {0:?}")]
    NoValidStatesAfterInvariants(NodeIndex),

    #[error("Heuristic failed to select state for node {0:?}")]
    HeuristicFailure(NodeIndex),

    #[error("Multiple states remain for uncollapsed node {0:?}: expected 1, found {1}")]
    MultipleStatesRemain(NodeIndex, usize),

    #[error("No solution found")]
    NoSolution,

    #[error("Node not found in graph")]
    NodeNotFound(NodeIndex),

    #[error("Node not found at position {0:?}")]
    NodeNotFoundAtPosition((usize, usize, usize)),
}
