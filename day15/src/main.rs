extern crate pathfinding;

use std::{self, fs::File, io::prelude::Read, path::Path, str::FromStr};

use anyhow::{Context, Error, Result};
use pathfinding::prelude::dijkstra;

#[derive(Debug)]
struct Cavern {
	risks: [u32; Cavern::SIZE],
}

impl Cavern {
	const WIDTH: usize = 100;
	const HEIGHT: usize = 100;
	const SIZE: usize = Cavern::WIDTH * Cavern::HEIGHT;

	fn new() -> Self {
		Cavern {
			risks: [0; Cavern::SIZE],
		}
	}

	fn neighbors(&self, p: usize) -> Vec<(usize, u32)> {
		let mut neighbors = vec![];

		if p > (Cavern::WIDTH - 1) {
			neighbors.push((p - Cavern::WIDTH, self.risks[p - Cavern::WIDTH])); // top
		}
		if p % Cavern::WIDTH != 0 {
			neighbors.push((p - 1, self.risks[p - 1])); // left
		}
		if (p + 1) % Cavern::WIDTH != 0 {
			neighbors.push((p + 1, self.risks[p + 1])); // right
		}
		if p < (Cavern::SIZE - Cavern::WIDTH) {
			neighbors.push((p + Cavern::WIDTH, self.risks[p + Cavern::WIDTH])); // bottom
		}

		neighbors
	}
}

impl FromStr for Cavern {
	type Err = Error;
	fn from_str(s: &str) -> Result<Self> {
		let mut cavern = Cavern::new();

		s.replace('\n', "")
			.chars()
			.enumerate()
			.for_each(|(i, c)| cavern.risks[i] = c.to_digit(10).unwrap() as u32);

		Ok(cavern)
	}
}
fn main() -> Result<()> {
	let cavern = get_input("input.txt")?;

	static GOAL: usize = Cavern::SIZE - 1;
	let result = dijkstra(&0, |&p| cavern.neighbors(p), |&p| p == GOAL);
	println!("answer: {:?}", result.unwrap().1);

	Ok(())
}

fn get_input(filename: impl AsRef<Path>) -> Result<Cavern> {
	let mut file = File::open(filename).with_context(|| "Can't open file")?;
	let mut buffer = String::new();
	file.read_to_string(&mut buffer)?;

	Ok(buffer.parse().unwrap())
}
