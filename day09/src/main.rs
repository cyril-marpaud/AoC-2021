use anyhow::{Context, Error, Result};
use std::{self, cmp::Ordering, fs::File, io::prelude::Read, path::Path, str::FromStr};

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

	fn get_adjacent_coords(&self, p: usize) -> Vec<(usize, &u8)> {
		let mut locations = vec![];

		if p > (Floor::WIDTH - 1) {
			locations.push((p - Floor::WIDTH, &self.heights[p - Floor::WIDTH])); // top
		}
		if p % Floor::WIDTH != 0 {
			locations.push((p - 1, &self.heights[p - 1])); // left
		}
		if (p + 1) % Floor::WIDTH != 0 {
			locations.push((p + 1, &self.heights[p + 1])); // right
		}
		if p < (Floor::WIDTH * (Floor::HEIGHT - 1) - 1) {
			locations.push((p + Floor::WIDTH, &self.heights[p + Floor::WIDTH])); // bottom
		}

		locations
	}

	fn get_low_points(&self) -> Vec<Vec<usize>> {
		self
			.heights
			.iter()
			.enumerate()
			.filter(|(p, &h)| {
				self
					.get_adjacent_coords(*p)
					.iter()
					.all(|(_, &adj_h)| adj_h > h)
			})
			.map(|(p, _)| vec![p])
			.collect()
	}

	fn fill_basin<'a>(&self, basin: &'a mut Vec<usize>) -> &'a Vec<usize> {
		let mut temp_basin = basin
			.iter()
			.map(|pos| {
				self
					.get_adjacent_coords(*pos)
					.into_iter()
					.filter(|(p, &h)| h != 9 && !basin.contains(p))
					.map(|(p, _)| p)
					.collect::<Vec<_>>()
			})
			.collect::<Vec<Vec<_>>>()
			.concat();

		if temp_basin.len() == 0 {
			return basin;
		}

		basin.append(&mut temp_basin);
		basin.sort();
		basin.dedup();

		self.fill_basin(basin)
	}
}

fn main() -> Result<()> {
	let floor = get_input("input.txt")?;

	let mut low_points = floor.get_low_points();

	let mut basins = low_points
		.iter_mut()
		.map(|b| floor.fill_basin(b))
		.collect::<Vec<_>>();

	basins.sort_by(|a, b| match a.len() as i32 - b.len() as i32 {
		// inverted because we want them in descending order
		l if l < 0 => Ordering::Greater,
		l if l > 0 => Ordering::Less,
		_ => Ordering::Equal,
	});

	println!(
		"answer: {:?}",
		basins.iter().take(3).map(|b| b.len()).product::<usize>()
	);

	Ok(())
}

fn get_input(filename: impl AsRef<Path>) -> Result<Floor> {
	let mut file = File::open(filename).with_context(|| "Can't open file")?;
	let mut buffer = String::new();
	file.read_to_string(&mut buffer)?;

	Ok(buffer.parse().unwrap())
}
