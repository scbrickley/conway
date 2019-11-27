extern crate ggez;

use ggez::event;
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::{Context, GameResult};

const CELL_SIZE: f32 = 10.0;
const TOP: f32 = -10.0;
const LEFT: f32 = -10.0;
const BOTTOM: f32 = 580.0;
const RIGHT: f32 = 780.0;

struct Cell {
    alive: bool,
}

struct MainState {
    grid: Vec<Vec<Cell>>,
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let mut grid: Vec<Vec<Cell>> = vec![];

        for _ in 0..=((RIGHT - LEFT) / CELL_SIZE) as i32 {
            let mut row = vec![];

            for _ in 0..=((BOTTOM - TOP) / CELL_SIZE) as i32 {
                row.push(Cell { alive: false });
            }

            grid.push(row);
        }

        let state = MainState { grid };
        Ok(state)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.grid[79][59].alive = true;
        self.grid[0][59].alive = true;
        self.grid[79][0].alive = true;
        self.grid[0][0].alive = true;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.1, 0.1, 1.0].into());

        let square = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(CELL_SIZE, CELL_SIZE, CELL_SIZE, CELL_SIZE),
            graphics::WHITE,
        )?;

        for row in 0..self.grid.len() {
            for column in 0..self.grid[row].len() {
                if self.grid[row][column].alive {
                    let x: f32 = row as f32 * CELL_SIZE + LEFT;
                    let y: f32 = column as f32 * CELL_SIZE + TOP;
                    graphics::draw(ctx, &square, (na::Point2::new(x, y),))?;
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
    let state = &mut MainState::new()?;
    event::run(ctx, event_loop, state)
}
