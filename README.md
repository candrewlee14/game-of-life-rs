# game-of-life-rs

Conway's Game of Life implemented in Rust for the terminal (including mouse input).

## Controls

-   User can navigate the red cursor with the arrow keys or mouse (if your terminal supports mouse input)
-   User can flip on cells at the red cursor location using the `Enter` key or the Left mouse button
-   User can pause the game using the `Space` key, and continue moving the cursor and flipping on cells while game is paused

## Features

-   Optional user-provided RNG seeding with `-s` or `--seed` integer commmand line arguments
-   Option to start in empty sandbox mode using `-e` or `--empty` command line flag
-   Cross-platform terminal support with mouse input using crossterm crate
-   Optional parallelized calculation of next game frame using rayon crate (although this is likely slower than sequential, shown by benchmarks) using `-p` or `--parallel` flags
-   Included command line flags `--help` or `-h`
-   Benchmarking using criterion crate
