extern crate ggez;

use ggez::GameResult;

use super::cell_grid::{ Cell, Grid, RIGHT, BOTTOM, CELL_SIZE };

impl Grid {
	pub fn glider_gun() -> GameResult<Grid> {
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

	pub fn exploder() -> GameResult<Grid> {
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

	pub fn glider() -> GameResult<Grid> {
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

	pub fn bar() -> GameResult<Grid> {
		let mut cells: Vec<Vec<Cell>> = vec![];

		for _ in 0..=(RIGHT / CELL_SIZE) as usize {
			let mut row = vec![];

			for _ in 0..=(BOTTOM / CELL_SIZE) as usize {
				row.push(Cell { alive: false });
			}

			cells.push(row);
		}

		cells[10][27].alive = true;
		cells[10][26].alive = true;
		cells[10][25].alive = true;
		cells[10][24].alive = true;
		cells[10][23].alive = true;
		cells[10][22].alive = true;
		cells[10][21].alive = true;
		cells[10][20].alive = true;
		cells[10][19].alive = true;
		cells[10][18].alive = true;

		let state = Grid { cells, paused: false };
		Ok(state)
	}
}
