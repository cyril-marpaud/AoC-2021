use anyhow::{Context, Error, Result};
use std::{self, fs::File, io::prelude::Read, path::Path, str::FromStr};

#[derive(Debug)]
struct Floor {
	heights: [u8; Floor::SIZE],
}

impl FromStr for Floor {
	type Err = Error;
	fn from_str(s: &str) -> Result<Self> {
		let mut floor = Floor::new();

		s.replace('\n', "")
			.chars()
			.enumerate()
			.for_each(|(i, c)| floor.heights[i] = c.to_digit(10).unwrap() as u8);

		Ok(floor)
	}
}

impl Floor {
	const WIDTH: usize = 100;
	const HEIGHT: usize = 100;
	const SIZE: usize = Floor::WIDTH * Floor::HEIGHT;

	fn new() -> Self {
		Floor {
			heights: [0; Floor::SIZE],
		}
	}

	fn get_adjacent_locations(p: usize) -> Vec<usize> {
		let mut locations = vec![];

		if p > (Floor::WIDTH - 1) {
			locations.push(p - Floor::WIDTH); // top
		}
		if p < (Floor::WIDTH * (Floor::HEIGHT - 1) - 1) {
			locations.push(p + Floor::WIDTH); // bottom
		}
		if p % Floor::WIDTH != 0 {
			locations.push(p - 1); // left
		}
		if (p + 1) % Floor::WIDTH != 0 {
			locations.push(p + 1); // right
		}

		locations
	}

	fn get_low_points(&self) -> Vec<(usize, &u8)> {
		self
			.heights
			.iter()
			.enumerate()
			.filter(|(p, &h)| {
				let adj = Floor::get_adjacent_locations(*p);
				adj.iter()
					.map(|&loc| self.heights[loc])
					.all(|adj_h| adj_h > h)
			})
			.collect()
	}
}

fn main() -> Result<()> {
	let floor = get_input("input.txt")?;

	println!(
		"answer: {}",
		floor
			.get_low_points()
			.iter()
			.map(|(_, &h)| h as u32 + 1)
			.sum::<u32>()
	);

	Ok(())
}

fn get_input(filename: impl AsRef<Path>) -> Result<Floor> {
	let mut file = File::open(filename).with_context(|| "Can't open file")?;
	let mut buffer = String::new();
	file.read_to_string(&mut buffer)?;

	Ok(buffer.parse().unwrap())
}
