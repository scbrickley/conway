extern crate ggez;

use ggez::event;
use ggez::event::{ MouseButton, KeyCode, KeyMods };
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::{ Context, GameResult };
use ggez::conf::WindowSetup;

const CELL_SIZE: f32 = 10.0;
const BOTTOM: f32 = 590.0;
const RIGHT: f32 = 790.0;

struct Cell {
    alive: bool,
}

impl Cell {
    fn toggle_alive(&mut self) {
        match self.alive {
            true => self.alive = false,
            false => self.alive = true
        }
    }
}

struct Grid {
    cells: Vec<Vec<Cell>>,
    paused: bool
}

impl Grid {
    fn new() -> GameResult<Grid> {
        let mut cells: Vec<Vec<Cell>> = vec![];

        for _ in 0..=(RIGHT / CELL_SIZE) as usize {
            let mut row = vec![];

            for _ in 0..=(BOTTOM / CELL_SIZE) as usize {
                row.push(Cell { alive: false });
            }

            cells.push(row);
        }

        let state = Grid { cells, paused: true };
        Ok(state)
    }

    fn glider_gun() -> GameResult<Grid> {
        let mut cells: Vec<Vec<Cell>> = vec![];

        for _ in 0..=(RIGHT / CELL_SIZE) as usize {
            let mut row = vec![];

            for _ in 0..=(BOTTOM / CELL_SIZE) as usize {
                row.push(Cell { alive: false });
            }

            cells.push(row);
        }

        cells[10][10].alive = true;
        cells[10][11].alive = true;
        cells[11][11].alive = true;
        cells[11][10].alive = true;

        cells[19][10].alive = true;
        cells[20][10].alive = true;
        cells[20][11].alive = true;
        cells[19][12].alive = true;
        cells[18][11].alive = true;
        cells[18][12].alive = true;

        cells[26][12].alive = true;
        cells[26][13].alive = true;
        cells[26][14].alive = true;
        cells[27][12].alive = true;
        cells[28][13].alive = true;

        cells[34][20].alive = true;
        cells[34][21].alive = true;
        cells[35][22].alive = true;
        cells[35][20].alive = true;
        cells[36][20].alive = true;

        cells[32][10].alive = true;
        cells[32][9].alive = true;
        cells[33][10].alive = true;
        cells[33][8].alive = true;
        cells[34][8].alive = true;
        cells[34][9].alive = true;

        cells[44][9].alive = true;
        cells[44][8].alive = true;
        cells[45][8].alive = true;
        cells[45][9].alive = true;

        cells[45][15].alive = true;
        cells[45][16].alive = true;
        cells[45][17].alive = true;
        cells[46][15].alive = true;
        cells[47][16].alive = true;

        let state = Grid { cells, paused: false };
        Ok(state)
    }

    fn exploder() -> GameResult<Grid> {
        let mut cells: Vec<Vec<Cell>> = vec![];

        for _ in 0..=(RIGHT / CELL_SIZE) as usize {
            let mut row = vec![];

            for _ in 0..=(BOTTOM / CELL_SIZE) as usize {
                row.push(Cell { alive: false });
            }

            cells.push(row);
        }

        cells[10][10].alive = true;
        cells[10][11].alive = true;
        cells[10][12].alive = true;
        cells[10][13].alive = true;
        cells[10][14].alive = true;
        cells[14][10].alive = true;
        cells[14][11].alive = true;
        cells[14][12].alive = true;
        cells[14][13].alive = true;
        cells[14][14].alive = true;
        cells[12][10].alive = true;
        cells[12][14].alive = true;

        let state = Grid { cells, paused: false };
        Ok(state)
    }

    fn glider() -> GameResult<Grid> {
        let mut cells: Vec<Vec<Cell>> = vec![];

        for _ in 0..=(RIGHT / CELL_SIZE) as usize {
            let mut row = vec![];

            for _ in 0..=(BOTTOM / CELL_SIZE) as usize {
                row.push(Cell { alive: false });
            }

            cells.push(row);
        }

        cells[10][20].alive = true;
        cells[11][20].alive = true;
        cells[12][20].alive = true;
        cells[12][21].alive = true;
        cells[11][22].alive = true;

        let state = Grid { cells, paused: false };
        Ok(state)
    }

    fn bar() -> GameResult<Grid> {
        let mut cells: Vec<Vec<Cell>> = vec![];

        for _ in 0..=(RIGHT / CELL_SIZE) as usize {
            let mut row = vec![];

            for _ in 0..=(BOTTOM / CELL_SIZE) as usize {
                row.push(Cell { alive: false });
            }

            cells.push(row);
        }

        cells[10][20].alive = true;
        cells[10][19].alive = true;
        cells[10][18].alive = true;

        let state = Grid { cells, paused: false };
        Ok(state)
    }

    fn count_neighbors(&self, x: i32, y: i32) -> u32 {
        let mut neighbors = 0;

        for row_mod in -1i32..=1i32 {
            for col_mod in -1i32..=1i32 {
                if row_mod == 0 && col_mod == 0 {
                    continue;
                }
                let (wrapped_x, wrapped_y) = wrap_coords(x + row_mod, y + col_mod);
                if self.cells[wrapped_x][wrapped_y].alive {
                    neighbors += 1
                }
            }
        }

        neighbors
    }

    fn toggle_pause(&mut self) {
        match self.paused {
            true => self.paused = false,
            false => self.paused = true
        }
    }
}

impl event::EventHandler for Grid {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if self.paused { return Ok(()) };

        let mut new_grid = Grid::new()?;

        for row in 0..self.cells.len() {
            for col in 0..self.cells[row].len() {
                match self.count_neighbors(row as i32, col as i32) {
                    0 | 1 => new_grid.cells[row][col].alive = false,
                    2 => new_grid.cells[row][col].alive = self.cells[row][col].alive,
                    3 => new_grid.cells[row][col].alive = true,
                    4..=8 => new_grid.cells[row][col].alive = false,
                    _ => println!("FUCK"),
                }
            }
        }
        self.cells = new_grid.cells;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.9, 0.2, 0.2, 1.0].into());

        let square = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(0.0, 0.0, CELL_SIZE, CELL_SIZE),
            graphics::WHITE,
        )?;

        for row in 0..self.cells.len() {
            for column in 0..self.cells[row].len() {
                let x: f32 = row as f32 * CELL_SIZE;
                let y: f32 = column as f32 * CELL_SIZE;
                if self.cells[row][column].alive {
                    graphics::draw(ctx, &square, (na::Point2::new(x, y),))?;
                }
            }
        }

        graphics::present(ctx)?;
        Ok(())
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, _button: MouseButton, x: f32, y: f32) {
        let row = (x / CELL_SIZE) as usize;
        let column = (y / CELL_SIZE) as usize;

        self.cells[row][column].toggle_alive();
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        _repeat: bool,
    ) { 
        match keycode {
            KeyCode::Space => self.toggle_pause(),
            KeyCode::Q | KeyCode::Escape => event::quit(ctx),
            _ => println!("No mapping for the {:?} key", keycode)
        }
    }
}

fn main() -> GameResult {
    let mut grid: Grid;
    let argv: Vec<String> = std::env::args().collect();

    if argv.len() == 2 {
        grid = match &argv[1][..] {
            "exploder" => Grid::exploder().unwrap(),
            "glider" => Grid::glider().unwrap(),
            "gun" => Grid::glider_gun().unwrap(),
            "bar" => Grid::bar().unwrap(),
            _ => Grid::new().unwrap()
        };
    } else {
        grid = Grid::new().unwrap();
    }

    let window_setup = WindowSetup::default().title("Conway's Game of Life").vsync(true);

    let cb = ggez::ContextBuilder::new("super_simple", "ggez")
        .window_setup(window_setup);
    let (ctx, event_loop) = &mut cb.build()?;
    event::run(ctx, event_loop, &mut grid)
}

fn wrap_coords(mut x: i32, mut y: i32) -> (usize, usize) {
    if x > (RIGHT / CELL_SIZE) as i32 {
        x = 0;
    } else if x < 0 {
        x = (RIGHT / CELL_SIZE) as i32;
    }

    if y > (BOTTOM / CELL_SIZE) as i32 {
        y = 0;
    } else if y < 0 {
        y = (BOTTOM / CELL_SIZE) as i32;
    }

    (x as usize, y as usize)
}

#[test]
fn test_wrap() {
    assert_eq!(wrap_coords(100, 100), (0, 0));
    assert_eq!(wrap_coords(-100, -100), (79, 59));
}

#[test]
fn test_neighbor_count() {
    let grid = Grid::bar().unwrap();

    let neighbors = grid.count_neighbors(10, 19);

    assert_eq!(neighbors, 2u32);
}
