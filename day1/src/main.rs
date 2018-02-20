use std::collections::HashSet;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::str::FromStr;

extern crate itertools;
use itertools::Itertools;
use itertools::FoldWhile::{Continue, Done};

fn main() {
	let f = File::open("day1.txt").unwrap();
	let mut reader = BufReader::new(f);
	let mut buffer = String::new();

	// read a line into buffer
	reader.read_line(&mut buffer);
	let buffer = buffer.trim_right();
	let moves: Vec<&str> = buffer.split(", ").collect();
	let direction: i32 = 0;
	let directions = [(0,1), (1,0), (0,-1), (-1,0)];
	let position = (0, 0);
	let mut set = HashSet::new();
	
	set.insert(position);

	let (final_position, _) = match moves.iter().fold_while((position, direction), |acc, &x| {
		let new_direction = if x.starts_with('R') {
			(((acc.1 + 1) % 4) + 4) %4
		} else {
			(((acc.1 - 1) % 4) + 4) %4
		};
		let distance = i32::from_str(&(x[1..])).unwrap();
		let mut new_x:i32 = (acc.0).0;
		let mut new_y:i32 = (acc.0).1;
		for _ in 0..distance {
			new_x += directions[new_direction as usize].0;
			new_y += directions[new_direction as usize].1;
			let new_position = (new_x, new_y);
			if set.contains(&new_position) {
				return Done((new_position, new_direction));
			}
			set.insert(new_position);
		}
		let new_position = (new_x, new_y);
		Continue((new_position, new_direction))
	}) {
		Continue(t) => t,
		Done(t) => {
			println!("Done");
			t
		},
	};

	println!("{}, {}", final_position.0, final_position.1);
	println!("{}", final_position.0.abs() + final_position.1.abs());
}

fn direction_to_char(direction: i32) -> char {
	match direction {
		0 => 'N',
		1 => 'E',
		2 => 'S',
		3 => 'W',
		_ => 'G',
	}
}