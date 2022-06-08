extern crate pathfinding;

use std::{self, fs::File, io::prelude::Read, path::Path, str::FromStr};

use anyhow::{Context, Error, Result};
use pathfinding::prelude::dijkstra;

#[derive(Debug)]
struct Cavern {
	risks: [u32; Cavern::SMALL_SIZE],
	extended_risks: Box<[u32; Cavern::BIG_SIZE]>,
}

impl Cavern {
	const FACTOR: usize = 5;
	const SMALL_SIDE: usize = 100;
	const BIG_SIDE: usize = Cavern::SMALL_SIDE * Cavern::FACTOR;
	const SMALL_SIZE: usize = Cavern::SMALL_SIDE * Cavern::SMALL_SIDE;
	const BIG_SIZE: usize = Cavern::SMALL_SIZE * Cavern::FACTOR * Cavern::FACTOR;

	fn new() -> Self {
		Cavern {
			risks: [0; Cavern::SMALL_SIZE],
			extended_risks: Box::new([0; Cavern::BIG_SIZE]),
		}
	}

	fn neighbors(&self, p: usize) -> Vec<(usize, u32)> {
		let mut neighbors = Vec::new();

		if p > (Cavern::BIG_SIDE - 1) {
			neighbors.push((
				p - Cavern::BIG_SIDE,
				self.extended_risks[p - Cavern::BIG_SIDE],
			)); // top
		}
		if p % Cavern::BIG_SIDE != 0 {
			neighbors.push((p - 1, self.extended_risks[p - 1])); // left
		}
		if (p + 1) % Cavern::BIG_SIDE != 0 {
			neighbors.push((p + 1, self.extended_risks[p + 1])); // right
		}
		if p < (Cavern::BIG_SIZE - Cavern::BIG_SIDE) {
			neighbors.push((
				p + Cavern::BIG_SIDE,
				self.extended_risks[p + Cavern::BIG_SIDE],
			)); // bottom
		}

		neighbors
	}

	fn extend_by_factor(&mut self) {
		self.risks.iter().enumerate().for_each(|(p, r)| {
			(0..Cavern::FACTOR).for_each(|x| {
				(0..Cavern::FACTOR).for_each(|y| {
					let position = Cavern::SMALL_SIDE * Cavern::FACTOR * (p / Cavern::SMALL_SIDE)
						+ p % Cavern::SMALL_SIDE + Cavern::SMALL_SIDE * y
						+ Cavern::SMALL_SIDE * Cavern::SMALL_SIDE * Cavern::FACTOR * x;
					let new_risk = (*r - 1 + (x + y) as u32) % 9 + 1;
					self.extended_risks[position] = new_risk;
				})
			});
		});
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
	let mut cavern = get_input("input.txt")?;
	cavern.extend_by_factor();

	static GOAL: usize = Cavern::BIG_SIZE - 1;
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
