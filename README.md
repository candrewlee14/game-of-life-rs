# game-of-life-rs
Conway's Game of Life implemented in Rust.

## Controls
- User can navigate the red cursor with the arrow keys or mouse (if your terminal supports mouse input)
- User can flip on cells at the red cursor location using the `Enter` key or the Left mouse button
- User can pause the game using the `Space` key, and continue moving the cursor and flipping on cells while game is paused

## Features
- Optional user-provided RNG seeding with `-s` or `--seed` commmand line arguments
- Cross-platform terminal support with mouse input with crossterm crate
- Parallelized generation advancing using rayon crate (although this is likely slower than sequential, shown by benchmarks)
- Benchmarking using criterion crate
