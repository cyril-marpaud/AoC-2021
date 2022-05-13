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
}

fn main() -> Result<()> {
	let floor = get_input("input.txt")?;

	println!("floor: {:?}", floor);

	Ok(())
}

fn get_input(filename: impl AsRef<Path>) -> Result<Floor> {
	let mut file = File::open(filename).with_context(|| "Can't open file")?;
	let mut buffer = String::new();
	file.read_to_string(&mut buffer)?;

	Ok(buffer.parse().unwrap())
}
