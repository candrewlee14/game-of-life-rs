use crossterm::event::{read, Event};
use crossterm::{
    cursor,
    style::{self, Color, Colorize, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal, ExecutableCommand, QueueableCommand, Result,
};
use std::io::{stdout, Stdout, Write};

fn make_color(i: u16) -> Color {
    Color::from((
        ((i * 7 + 43) % 256) as u8,
        ((i * 23 + 97) % 256) as u8,
        ((i * 19 + 13) % 256) as u8,
    ))
}

struct Grid {
    width: u16,
    height: u16,
    grid: Vec<Vec<(Color, char)>>,
}
impl Grid {
    pub fn new(x: u16, y: u16, color: Option<Color>) -> Self {
        Self {
            width: x,
            height: y,
            grid: vec![vec![(color.unwrap_or(Color::Black), ' '); x as usize]; y as usize],
        }
    }
    pub fn put_circle(&mut self, rad: u16, x_center: u16, y_center: u16 , color: Color) {
        // rad^2 = x*x + y*y
        // y = sqrt(rad^2-x*x)
        assert!(x_center < self.width);
        assert!(y_center < self.height);
        const CELL_W_TO_H_RATIO : f64 = 3.4;

        let rad_fl = rad as f64;
        let y_center_fl = y_center as f64;
        for x in -(rad as i32)..=rad as i32 {
            let world_x = x_center as i32 + x;
            if world_x < 0 || world_x >= self.width as i32 {
                continue;
            }
            let world_x_usize = world_x as usize;
            // add that extra divide because the vertical height of a cell is about double its
            // width
            let y_fl = (rad_fl * rad_fl - (x * x) as f64).sqrt() / CELL_W_TO_H_RATIO.sqrt();

            let pos_y = y_center_fl + y_fl;
            let pos_y_usize = pos_y.round() as usize;
            if pos_y_usize < self.height as usize {
                self.grid[pos_y_usize][world_x_usize] = (color, ' ');
            }
            let neg_y = y_center_fl - y_fl;
            if neg_y >= 0.0 {
                let neg_y_usize = neg_y.round() as usize;
                self.grid[neg_y_usize][world_x_usize] = (color, ' ');
            }
        }
        // draw by Ys because some are missed
        let x_center_fl = x_center as f64;
        for y in -(rad as i32)..=rad as i32 {
            let world_y = y_center as i32 + y;
            if world_y < 0 || world_y >= self.height as i32 {
                continue;
            }
            let world_y_usize = world_y as usize;
            // add that extra divide because the vertical height of a cell is about double its
            // width
            let x_fl = (rad_fl * rad_fl - ((y * y) as f64) * CELL_W_TO_H_RATIO).sqrt() ;

            let pos_x = x_center_fl + x_fl;
            let pos_x_usize = pos_x.round() as usize;
            if pos_x_usize < self.width as usize {
                self.grid[world_y_usize][pos_x_usize] = (color, ' ');
            }
            let neg_x = x_center_fl - x_fl;
            if neg_x >= 0.0 {
                let neg_x_usize = neg_x.round() as usize;
                self.grid[world_y_usize][neg_x_usize] = (color, ' ');
            }
        }
    }
    pub fn print(&self, stdout: &mut Stdout) -> Result<()> {
        for y in 0..self.height {
            for x in 0..self.width {
                let (color, content) = self.grid[y as usize][x as usize];
                stdout
                    .queue(cursor::MoveTo(x, y))?
                    .queue(style::SetBackgroundColor(color))?
                    .queue(Print(content.to_string()))?;
            }
        }
        stdout.flush()?;
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
    let (max_x, max_y) = terminal::size()?;
    let mut screen = Grid::new(max_x, max_y, None);
    screen.put_circle(max_y/3, max_x/2, max_y/2, Color::from((100, 50, 50)));
    screen.print(&mut stdout)?;
    wait_key()?;
    screen.put_circle(max_y/4, max_x/3, max_y/2, Color::from((50, 100, 50)));
    screen.print(&mut stdout)?;
    wait_key()?;
    screen.put_circle(max_y/4, max_x*2/3, max_y/2, Color::from((50, 50, 100)));
    screen.print(&mut stdout)?;
    wait_key()?;
    Ok(())
}
