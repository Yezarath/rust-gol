#![allow(dead_code)] // Disable dead_code warning for this function

use crate::board::Board;
use std::{
	fs::{read_to_string, File},
	io::Write,
};

pub fn parse_map_file(file: &str, b: &mut Board) {
	let file = read_to_string(file).expect("Could not read file");

	file.split('|').enumerate().for_each(|(_, line)| {
		match line.split_once(',') {
			| Some((xstr, ystr)) => {
				b.insert_cell(
					xstr.parse::<i128>().expect("Could not parse {xstr}"),
					ystr.parse::<i128>().expect("Could not parse {ystr}"),
				);
			}
			| None => (),
		}
	});
}

pub fn save_map_file(file: &str, b: &Board) {
	let mut file: File = File::create(file).expect("Could not create file");

	let map = b.get_map();

	map.iter().for_each(|((x, y), _)| {
		file.write(format!("{x},{y}|").as_bytes())
			.expect("Could not write to file");
	});
}
