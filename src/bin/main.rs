use std::io::{Write, stdout};
use crossterm::event::{
    poll, read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, MouseButton,
    MouseEventKind,
};
use crossterm::{
    cursor,
    style::{self, Color, Print, SetBackgroundColor},
    terminal, ExecutableCommand, QueueableCommand, Result,
};
use std::time::Duration;
use game_of_life_rs::game::Grid;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "game-of-life-rs", about ="A Rust terminal implementation of Conway's Game of Life (with mouse support)")]
struct Opt {
    /// Seed (u64) for the initial state random generation. If no seed argument is provided, initial state will be randomized for each run.
    #[structopt(short, long)]
    seed: Option<u64>,
} 

fn main() -> Result<()> {
    let opt = Opt::from_args();
    let mut stdout = stdout();
    let (max_x, max_y) = terminal::size()?;
    let mut screen : Grid = match opt.seed {
        Some(seed) => Grid::new_seeded(max_x as usize, max_y as usize, seed),
        None => Grid::new(max_x as usize, max_y as usize),
    };
    stdout.execute(cursor::Hide)?;
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;
    stdout.execute(EnableMouseCapture)?;
    terminal::enable_raw_mode()?;
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
            screen.propogate_par(&|(cell, arr)| {
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
            screen.set_cell(cursor_x as usize, cursor_y as usize, true);
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
