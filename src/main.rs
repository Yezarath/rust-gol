/*
** Rust implementation of the Conway's Game of Life.
** Data Structures should be a hash table to store the cells
*/

mod board;
mod display;
mod parser;

use board::Board;

use clap::Parser;
use display::Display;
use std::{
	sync::{Arc, Mutex},
	thread,
	time::{Duration, SystemTime as ST},
};

#[derive(Parser)]
#[command(version, about)]
struct Args {
	/// Map file to load.
	#[arg(short, long, default_value = "map.gol")]
	map: String,
	/// Speed of the simulation in milliseconds.
	#[arg(short, long, default_value = "16")]
	speed: u64,
	/// Number of cycles to simulate before starting the game.
	#[arg(short, long, default_value = "0")]
	cycle: u64,
}

fn to_sleep(speed: u64, start: ST) {
	let duration = match ST::now().duration_since(start) {
		| Ok(d) => d.as_millis() as u64,
		| _ => speed,
	};
	if duration < speed {
		thread::sleep(Duration::from_millis(speed - duration));
	}
}

fn repeat_fn_ms(last: &mut ST, millis: u128, mut f: impl FnMut()) {
	let now = ST::now();
	let elapsed = match now.duration_since(*last) {
		| Ok(duration) => duration.as_millis(),
		| _ => 0,
	};
	if elapsed >= millis {
		f();
		*last = now;
	}
}

fn main() {
	let args = Args::parse();
	let mut term =
		termsize::get().unwrap_or_else(|| termsize::Size { cols: 30, rows: 5 });

	let mut board = Board::new();
	parser::parse_map_file(&args.map, &mut board);
	board.insert_cell(4, 4);

	let mut display = Display::new(0, 0, ((term.cols as i128) / 2) - 2, 29);
	display.clear();

	// mutex and arc to share the board and display between threads
	let board = Arc::new(Mutex::new(board));
	let display = Arc::new(Mutex::new(display));

	let board_for_sim = board.clone();
	let disp_for_sim = display.clone();
	let sim_thread = thread::spawn(move || {
		let mut board = board_for_sim.lock().unwrap();
		let mut display = disp_for_sim.lock().unwrap();
		board.cycle_n(args.cycle, &mut display);
		std::mem::drop(display);
		std::mem::drop(board);
		loop {
			to_sleep(args.speed, {
				let start = ST::now();
				let mut board = board_for_sim.lock().unwrap();
				board.cycle_once();
				start
			});
		}
	});

	// let disp_for_event = display.clone();
	let event_thread = thread::spawn(move || {
		// read stdin for events
		loop {
			// TODO: terminal in raw mode, might screw the display class, will see.
		}
	});

	std::thread::sleep(Duration::from_millis(100));
	let disp_thread = thread::spawn(move || {
		let mut start = ST::now();
		loop {
			to_sleep(args.speed, {
				repeat_fn_ms(&mut start, 60, || {
					match termsize::get() {
						| Some(t) => {
							if term.cols != t.cols && t.cols >= 20 {
								term = t;
								let mut d = display.lock().unwrap();
								d.clear();
								d.set_width(((term.cols as i128) / 2) - 2);
							}
						}
						| _ => {}
					};
				});
				let mut board = board.lock().unwrap();
				let mut display = display.lock().unwrap();
				board.print_board(&mut display);
				ST::now()
			});
		}
	});

	event_thread.join().unwrap();
	std::process::exit(0);
	sim_thread.join().unwrap();
	disp_thread.join().unwrap();
}
