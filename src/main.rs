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

        for _ in 0..=(RIGHT / CELL_SIZE) as i32 {
            let mut row = vec![];

            for _ in 0..=(BOTTOM / CELL_SIZE) as i32 {
                row.push(Cell { alive: false });
            }

            cells.push(row);
        }

        let state = Grid { cells };
        Ok(state)
    }

/*
    fn count_neighbors(&self, x: usize, y: usize) -> u32 {
        
    }
*/
}

impl event::EventHandler for Grid {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.cells[79][59].alive = true;
        self.cells[0][59].alive = true;
        self.cells[79][0].alive = true;
        self.cells[0][0].alive = true;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.1, 0.1, 1.0].into());

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
    let grid = &mut Grid::new()?;
    event::run(ctx, event_loop, grid)
}
