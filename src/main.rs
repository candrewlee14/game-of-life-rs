use crossterm::event::{
    poll, read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, MouseButton,
    MouseEventKind,
};
use crossterm::{
    cursor,
    style::{self, Color, Colorize, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal, ExecutableCommand, QueueableCommand, Result,
};
use num::Integer;
use rand::prelude::*;
use rand_chacha::ChaCha20Rng;
use std::{
    io::{stdout, Stdout, Write},
    time::Duration,
};

struct Grid {
    width: usize,
    height: usize,
    grid: Vec<Vec<bool>>,
}
impl Grid {
    pub fn new(x: usize, y: usize) -> Self {
        let seed: <ChaCha20Rng as SeedableRng>::Seed = Default::default();
        let mut rng = ChaCha20Rng::from_seed(seed);

        Self {
            width: x,
            height: y,
            //grid: vec![vec![false; x as usize]; y as usize],
            grid: (0..y)
                .map(|_| (0..x).map(|_| rng.gen::<bool>()).collect())
                .collect(),
        }
    }
    fn decrement_wrap<T: Integer>(i: T, one_over_max: T) -> T {
        if i == T::zero() {
            return one_over_max - T::one();
        }
        return i - T::one();
    }
    fn increment_wrap<T: Integer>(i: T, one_over_max: T) -> T {
        if i == one_over_max - T::one() {
            return T::zero();
        }
        return i + T::one();
    }
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
    fn propogate(&mut self, run_rule: &'static (dyn Fn((bool, [bool; 8])) -> bool + Sync)) {
        let mut next_grid: Vec<Vec<bool>> =
            vec![vec![false; self.width as usize]; self.height as usize];
        for y in 0..self.height {
            for x in 0..self.width {
                next_grid[y][x] = run_rule(self.self_and_neighbors(x, y))
            }
        }
        self.grid = next_grid;
    }
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

fn wait_key() -> Result<()> {
    match read()? {
        Event::Key(_event) => Ok(()),
        _ => Ok(()),
    }
}

fn main() -> Result<()> {
    let mut stdout = stdout();
    stdout.execute(cursor::Hide)?;
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;
    stdout.execute(EnableMouseCapture)?;
    terminal::enable_raw_mode()?;
    let (max_x, max_y) = terminal::size()?;
    let mut screen = Grid::new(max_x as usize, max_y as usize);
    //for y in 5..10 {
    //    for x in 10..20 {
    //        screen.grid[y][x] = true;
    //    }
    //}
    let mut cursor_x: u16 = 0;
    let mut cursor_y: u16 = 0;
    let mut paused = false;
    screen.queue_print(&mut stdout, cursor_x, cursor_y)?;
    stdout.flush()?;
    loop {
        let mut cursor_moved = false;
        let mut add_cell_here = false;
        if poll(Duration::from_millis(10))? {
            match read()? {
                Event::Key(event) => match event.code {
                    KeyCode::Char('q') | KeyCode::Esc => {
                        break;
                    }
                    KeyCode::Char(' ') => {
                        paused = !paused;
                    }
                    KeyCode::Enter => {
                        add_cell_here = true;
                    }
                    KeyCode::Right => {
                        cursor_x = Grid::increment_wrap(cursor_x, screen.width as u16);
                        cursor_moved = true;
                    }
                    KeyCode::Left => {
                        cursor_x = Grid::decrement_wrap(cursor_x, screen.width as u16);
                        cursor_moved = true
                    }
                    KeyCode::Down => {
                        cursor_y = Grid::increment_wrap(cursor_y, screen.height as u16);
                        cursor_moved = true
                    }
                    KeyCode::Up => {
                        cursor_y = Grid::decrement_wrap(cursor_y, screen.height as u16);
                        cursor_moved = true
                    }
                    _ => (),
                },
                Event::Mouse(event) => {
                    cursor_x = event.column;
                    cursor_y = event.row;
                    cursor_moved = true;
                    if event.kind == MouseEventKind::Down(MouseButton::Left)
                        || event.kind == MouseEventKind::Drag(MouseButton::Left)
                    {
                        add_cell_here = true;
                    }
                }
                _ => (),
            }
        }
        if !paused {
            screen.propogate(&|(cell, arr)| {
                let neighbor_count = arr
                    .iter()
                    .fold(0, |acc, item| if *item { acc + 1 } else { acc });
                if neighbor_count == 3 {
                    return true;
                }
                if cell && neighbor_count == 2 {
                    return true;
                }
                return false;
            });
        }
        if !paused || cursor_moved {
            screen.queue_print(&mut stdout, cursor_x, cursor_y)?;
        }
        stdout.queue(cursor::MoveTo(cursor_x, cursor_y))?;
        if add_cell_here {
            screen.grid[cursor_y as usize][cursor_x as usize] = true;
            stdout
                .queue(SetBackgroundColor(Color::DarkGrey))?
                .queue(Print('X'.to_string()))?;
        } else if cursor_moved {
            //if cursor_moved {
            stdout
                .queue(SetBackgroundColor(Color::Red))?
                .queue(Print(' '.to_string()))?;
        }
        stdout.flush()?;
    }
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;
    stdout.execute(DisableMouseCapture)?;
    Ok(())
}
