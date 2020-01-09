extern crate ggez;

use ggez::event;
use ggez::event::{ MouseButton, KeyCode, KeyMods };
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::{ Context, GameResult };

pub const CELL_SIZE: f32 = 10.0;
pub const BOTTOM: f32 = 590.0;
pub const RIGHT: f32 = 790.0;

pub struct Cell {
	pub alive: bool,
}

impl Cell {
	fn toggle_alive(&mut self) {
		match self.alive {
			true => self.alive = false,
			false => self.alive = true
		}
	}
}

pub struct Grid {
	pub cells: Vec<Vec<Cell>>,
	pub paused: bool
}

impl Grid {
	pub fn new() -> GameResult<Grid> {
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
						_ => println!("This code should be unreachable. Something has gone wrong."),
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
