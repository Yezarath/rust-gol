use crate::display::Display;
use indicatif::{ProgressBar, ProgressStyle};
use std::collections::HashMap;

#[derive(PartialEq, Clone)]
pub enum CellState {
	Alive,
}

pub type Cell = (i128, i128);

pub struct Board {
	map: HashMap<Cell, CellState>,
	cycle_count: u128,
}

// this function is used to get the neighbors of a cell
// it returns an array of 8 cells
// the neighbors are the cells that are adjacent to the cell
// if the cell is at the edge of the board,
//		it will wrap around to the other side
fn get_neighbors(c: &Cell) -> [Cell; 8] {
	[
		(c.0 - 1, c.1 - 1),
		(c.0 - 1, c.1),
		(c.0 - 1, c.1 + 1),
		(c.0, c.1 - 1),
		(c.0, c.1 + 1),
		(c.0 + 1, c.1 - 1),
		(c.0 + 1, c.1),
		(c.0 + 1, c.1 + 1),
	]
}

impl Board {
	pub fn new() -> Board {
		Board { map: HashMap::new(), cycle_count: 0 }
	}
}

impl Board {
	pub fn print_board(&mut self, disp: &mut Display) -> usize {
		let mut live_cells_view: usize = 0;

		disp.top_border();
		Display::get_view_range(&disp, false).for_each(|y| {
			disp.left_border();
			Display::get_view_range(&disp, true).for_each(|x| {
				let is_alive = match self.map.get(&(x, y)) {
					| Some(CellState::Alive) => true,
					| _ => false,
				};
				disp.cell(x, &is_alive);
				live_cells_view += is_alive as usize;
			});
			disp.right_border();
		});
		disp.bottom_border();
		disp.footer(self.map.len(), live_cells_view, self.cycle_count);
		disp.flush();
		live_cells_view
	}

	pub fn cycle_once(&mut self) {
		let mut new_map: HashMap<Cell, CellState> = HashMap::new();

		let mut r4: HashMap<Cell, u8> = HashMap::new();
		self.map.keys().for_each(|c| {
			let mut count: u8 = 0;
			for neighbor in get_neighbors(&c) {
				match self.map.contains_key(&neighbor) {
					| true => count += 1,
					| false => *(r4.entry(neighbor).or_insert(0)) += 1,
				}
			}
			if count == 2 || count == 3 {
				new_map.insert(*c, CellState::Alive);
			}
		});

		r4.iter().for_each(|(c, count)| {
			if *count == 3 {
				new_map.insert(*c, CellState::Alive);
			}
		});
		self.map = new_map;
		self.cycle_count += 1;
	}

	pub fn cycle_n(&mut self, n: u64, disp: &mut Display) {
		let pb = ProgressBar::new(n);
		let f = format!(
			"{{msg:{}}}\n[{{elapsed:{}}}] {{bar:{}}} {{pos:{}}}/{{len:{}}}",
			"4.red.underlined", ">4.red", "40.grey/red", ">4.red", ">4.red"
		);
		let style = ProgressStyle::default_bar()
			.template(f.as_str())
			.unwrap()
			.progress_chars("▓░░");
		pb.set_style(style);
		pb.set_message(format!("Simulating {} cycles..", n));
		for _ in 0..n {
			self.cycle_once();
			pb.inc(1);
		}
		disp.clear();
	}
}

#[allow(dead_code)]
impl Board {
	pub fn map_is_empty(&self) -> bool {
		self.map.len() == 0
	}

	pub fn get_map(&self) -> &HashMap<Cell, CellState> {
		&self.map
	}

	pub fn insert_cell(&mut self, x: i128, y: i128) {
		self.map.insert((x, y), CellState::Alive);
	}
}
