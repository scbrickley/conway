extern crate ggez;

use ggez::event;
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::{Context, GameResult};

const CELL_SIZE: f32 = 10.0;
const BOTTOM: f32 = 590.0;
const RIGHT: f32 = 790.0;

struct Cell {
    alive: bool,
}

struct Grid {
    cells: Vec<Vec<Cell>>,
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

        let state = Grid { cells };
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

        let state = Grid { cells };
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

        let state = Grid { cells };
        Ok(state)
    }

    fn count_neighbors(&self, x: i32, y: i32) -> u32 {
        let mut neighbors = 0;

        for row_mod in -1i32..=1i32 {
            for col_mod in -1i32..=1i32 {
                if row_mod == 0 && col_mod == 0 { continue }
                let (wrapped_x, wrapped_y) = wrap_coords(x + row_mod, y + col_mod);
                if self.cells[wrapped_x][wrapped_y].alive {
                    neighbors += 1
                }
            }
        }

        neighbors
    }
}

impl event::EventHandler for Grid {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        let mut new_grid = Grid::new()?;

        for row in 0..self.cells.len() {
            for col in 0..self.cells[row].len() {
                match self.count_neighbors(row as i32, col as i32) {
                    0|1 => new_grid.cells[row][col].alive = false,
                    2 => new_grid.cells[row][col].alive =  self.cells[row][col].alive,
                    3 => new_grid.cells[row][col].alive = true,
                    4..=8 => new_grid.cells[row][col].alive = false,
                    _ => println!("FUCK")
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
                if self.cells[row][column].alive {
                    let x: f32 = row as f32 * CELL_SIZE;
                    let y: f32 = column as f32 * CELL_SIZE;
                    // I have no idea why a trailing comma needs to be here.
                    //                                                 |
                    //                                                 v
                    graphics::draw(ctx, &square, (na::Point2::new(x, y),))?;
                    // But it was included in the example code that this project was
                    // based on, and the program won't compile without it.
                }
            }
        }

        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let (ctx, event_loop) = &mut cb.build()?;
    let grid = &mut Grid::glider()?;
    event::run(ctx, event_loop, grid)
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
