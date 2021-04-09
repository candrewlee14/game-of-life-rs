use crossterm::{
    cursor,
    style::{self, Color, Print},
    QueueableCommand, Result,
};
use num::Integer;
use rand::prelude::*;
use rand_chacha::ChaCha20Rng;
use rayon::prelude::*;
use std::io::Stdout;

/// Grid object that holds 2D cell on/off matrix
pub struct Grid {
    pub width: usize,
    pub height: usize,
    grid: Vec<Vec<bool>>,
}
impl Grid {
    /// New grid with randomized initial state
    pub fn new(x: usize, y: usize) -> Self {
        let mut rng = ChaCha20Rng::from_entropy();

        Self {
            width: x,
            height: y,
            grid: (0..y)
                .map(|_| (0..x).map(|_| rng.gen::<bool>()).collect())
                .collect(),
        }
    }
    /// New grid with all cells turned off
    pub fn new_empty(x: usize, y: usize) -> Self {
        Self {
            width: x,
            height: y,
            grid: vec![vec![false; x as usize]; y as usize],
        }
    }
    /// New grid randomized using seed
    pub fn new_seeded(x: usize, y: usize, seed: u64) -> Self {
        let mut rng = ChaCha20Rng::seed_from_u64(seed);
        Self {
            width: x,
            height: y,
            grid: (0..y)
                .map(|_| (0..x).map(|_| rng.gen::<bool>()).collect())
                .collect(),
        }
    }
    /// Decrement with wrap (0 wraps to max)
    pub fn decrement_wrap<T: Integer>(i: T, one_over_max: T) -> T {
        if i == T::zero() {
            return one_over_max - T::one();
        }
        return i - T::one();
    }
    /// Increment with wrap (max wraps to 0)
    pub fn increment_wrap<T: Integer>(i: T, one_over_max: T) -> T {
        if i == one_over_max - T::one() {
            return T::zero();
        }
        return i + T::one();
    }
    /// Set cell at coordinates to on or off
    pub fn set_cell(&mut self, x: usize, y: usize, content: bool) {
        self.grid[y][x] = content;
    }
    /// Get cell at coordinates
    pub fn get_cell(&self, x: usize, y: usize) -> bool {
        self.grid[y][x] 
    }
    /// Returns a tuple with (cell at coords, [array of 8 neighbors row-by-row left-to-right])
    fn self_and_neighbors(&self, x: usize, y: usize) -> (bool, [bool; 8]) {
        let prev_x = Self::decrement_wrap(x, self.width);
        let next_x = Self::increment_wrap(x, self.width);
        let prev_y = Self::decrement_wrap(y, self.height);
        let next_y = Self::increment_wrap(y, self.height);
        (
            self.grid[y][x],
            [
                self.grid[prev_y][prev_x],
                self.grid[prev_y][x],
                self.grid[prev_y][next_x],
                self.grid[y][prev_x],
                self.grid[y][next_x],
                self.grid[next_y][prev_x],
                self.grid[next_y][x],
                self.grid[next_y][next_x],
            ],
        )
    }
    /// Calculate next game frame using the rule function parameter on each cell neighbor group
    pub fn propogate(&mut self, run_rule: &'static (dyn Fn((bool, [bool; 8])) -> bool + Sync)) {
        let mut next_grid: Vec<Vec<bool>> =
            vec![vec![false; self.width as usize]; self.height as usize];
        for y in 0..self.height {
            for x in 0..self.width {
                next_grid[y][x] = run_rule(self.self_and_neighbors(x, y))
            }
        }
        self.grid = next_grid;
    }
    /// Calculate next game frame in parallel using the rule function parameter on each cell neighbor group
    pub fn propogate_par(&mut self, run_rule: &'static (dyn Fn((bool, [bool; 8])) -> bool + Sync)) {
        let mut next_grid: Vec<Vec<bool>> =
            vec![vec![false; self.width as usize]; self.height as usize];
        next_grid
            .par_iter_mut()
            .enumerate()
            .for_each(|(y, next_row)| {
                next_row
                    .par_iter_mut()
                    .enumerate()
                    .for_each(|(x, next_cell)| {
                        *next_cell = run_rule(self.self_and_neighbors(x, y))
                    });
            });
        self.grid = next_grid;
    }
    /// Queue grid to be printed, stdout will need to be flushed after this to display
    pub fn queue_print(&self, stdout: &mut Stdout, cursor_x: u16, cursor_y: u16) -> Result<()> {
        for y in 0..self.height as u16 {
            for x in 0..self.width as u16 {
                let color = self.grid[y as usize][x as usize] as u8 * 255;
                if x != cursor_x || y != cursor_y {
                    stdout
                        .queue(cursor::MoveTo(x, y))?
                        .queue(style::SetBackgroundColor(Color::from((
                            color, color, color,
                        ))))?
                        .queue(Print(' '.to_string()))?;
                }
            }
        }
        Ok(())
    }
}

