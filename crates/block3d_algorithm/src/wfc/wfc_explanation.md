# Wave Function Collapse (WFC) Algorithm Documentation

This document explains the implementation of the Wave Function Collapse algorithm
found in `wfc.rs`.

## Overview

Wave Function Collapse is an algorithm inspired by quantum mechanics, where a system
starts in a superposition of all possible states and gradually collapses into
definite states. In our case, it's used for procedural generation of 3D structures
using LEGO-like blocks.

## Core Components

```rust
pub struct WFCSolver<T: WFCBlock> {
    states: HashMap<Position, Vec<(T, Rotation)>>,  // All possible states
    collapsed: HashSet<Position>,                   // Decided positions
    dimensions: (i32, i32, i32),                   // Grid size
}
```

## Algorithm Steps

### 1. Initialization

The grid starts with all positions containing all possible blocks in all possible
rotations:

```rust
pub fn initialize(&mut self, possible_blocks: Vec<T>) {
    // Each position starts with all possibilities
    for x in 0..self.dimensions.0 {
        for y in 0..self.dimensions.1 {
            for z in 0..self.dimensions.2 {
                // Initialize with all blocks and rotations
                // ...
            }
        }
    }
}
```

### 2. Main Loop

The algorithm repeatedly:
1. Finds the position with lowest entropy (fewest possibilities)
2. Collapses it to a single state
3. Propagates the constraints to neighbors

```rust
pub fn solve(&mut self) -> Result<HashMap<Position, (T, Rotation)>, String> {
    while let Some(pos) = self.find_lowest_entropy() {
        self.collapse_position(pos)?;
    }
    // ...
}
```

### 3. Constraint Propagation

When a position is collapsed, the algorithm:
- Updates neighboring positions' possible states
- Ensures all states are compatible with their neighbors
- Continues propagating if any changes occur

## Block Connections

The implementation uses three main connection types:
```rust
pub enum ConnectionType {
    Stud,        // Top connector (like LEGO studs)
    AntiStud,    // Bottom connector (connects to studs)
    Smooth,      // Flat surface
}
```

## Current Limitations and Potential Improvements

1. **Random Selection**
   - Current: Uses first available state
   - Improvement: Implement proper random selection:
   ```rust
   use rand::seq::SliceRandom;
   let selected = states.choose(&mut rand::thread_rng()).unwrap().clone();
   ```

2. **Backtracking**
   - Current: No backtracking when reaching impossible states
   - Improvement: Add stack-based backtracking system

3. **Performance**
   - Current: Sequential processing
   - Improvement: Add parallel processing for constraint propagation

4. **Rotation Handling**
   - Current: Basic 90-degree rotations
   - Improvement: Add support for arbitrary rotations and symmetries

## Example Usage

```rust
// Create a solver
let mut solver = WFCSolver::new((10, 10, 10));

// Create some blocks
let block = LegoBlock {
    width: 1,
    height: 1,
    depth: 1,
    connection_points: HashMap::new(),
};

// Initialize and solve
solver.initialize(vec![block]);
let solution = solver.solve()?;
```

## Testing

The implementation includes tests for:
- Solver initialization
- Block connections
- Invalid connections
- Smooth surface connections
- Neighbor detection

Additional test cases could include:
- Full solve process
- Error handling
- Complex rotation scenarios
- Backtracking scenarios

## References

- Original WFC: https://github.com/mxgmn/WaveFunctionCollapse
- Quantum Mechanics analogy: https://en.wikipedia.org/wiki/Wave_function_collapse
``` 