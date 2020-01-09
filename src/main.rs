extern crate ggez;

use ggez::event;
use ggez::GameResult;
use ggez::conf::WindowSetup;

mod grid;

use grid::cell_grid::Grid;

fn main() -> GameResult {
	let mut grid: Grid;
	let argv: Vec<String> = std::env::args().collect();

	if argv.len() == 2 {
		grid = match &argv[1][..] {
			"exploder" => Grid::exploder().unwrap(),
			"glider" 	 => Grid::glider().unwrap(),
			"gun" 		 => Grid::glider_gun().unwrap(),
			"bar"      => Grid::bar().unwrap(),
			_ 				 => Grid::new().unwrap()
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
