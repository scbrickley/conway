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

    // THE FUNCTION BELOW IS REALLY UGLY. FIND A WAY TO WRAP TO THE OTHER SIDE
    // WHEN INDEXING NEAR THE EDGE OF THE GRID. MAKE SURE IT DOESN'T
    // LOOK LIKE SHIT
    /*
        fn count_neighbors(&self, x: usize, y: usize) -> u32 {
            let mut nieghbors = 0

            match (x, y) {
                (0, 0) =>,
                (self.grid.len(), self.grid[0].len()) =>,
                (self.grid.len(), 0) =>,
                (0, self.grid[0].len()) =>,
                (0, y) =>,
                (self.grid.len(), y) =>,
                (x, 0) =>,
                (x, self.grid[0].len()) =>,
                _ => {
                    for row_mod in -1..=1 {
                        for col_mod in -1..=1 {
                            if self.cells[x + row_mod][y + col_mod].alive {
                                neighbors += 1
                            }
                        }
                    }
                }
            }
        }
    */
}

impl event::EventHandler for Grid {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.cells[30][30].alive = true;
        self.cells[31][30].alive = true;
        self.cells[32][30].alive = true;
        self.cells[32][29].alive = true;
        self.cells[31][28].alive = true;

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
    let grid = &mut Grid::new()?;
    event::run(ctx, event_loop, grid)
}
