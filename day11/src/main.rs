use anyhow::{Context, Error, Result};
use core::{
	fmt::{Display, Formatter},
	str::FromStr,
};
use std::{self, collections::HashSet, fs::File, io::prelude::Read, path::Path};

#[derive(Clone, Copy)]
struct Octopus {
	energy_lvl: u8,
	has_flashed: bool,
}

struct Octopi {
	grid: [Octopus; Octopi::SIZE],
	flash_count: u32,
}

impl Display for Octopi {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
		write!(
			f,
			"{}",
			self
				.grid
				.chunks(Octopi::WIDTH)
				.map(|c| String::from_iter(c.iter().map(|o| format!("{}", o.energy_lvl))) + "\n")
				.collect::<String>()
		)?;
		Ok(())
	}
}

impl FromStr for Octopi {
	type Err = Error;
	fn from_str(s: &str) -> Result<Self> {
		let mut octopi = Octopi::new();

		s.replace('\n', "").chars().enumerate().for_each(|(i, o)| {
			octopi.grid[i].energy_lvl = o.to_digit(10).unwrap() as u8;
		});

		Ok(octopi)
	}
}

impl Octopi {
	const WIDTH: usize = 10;
	const HEIGHT: usize = 10;
	const SIZE: usize = Octopi::WIDTH * Octopi::HEIGHT;

	fn new() -> Self {
		Octopi {
			grid: [Octopus {
				energy_lvl: 0,
				has_flashed: false,
			}; Octopi::SIZE],
			flash_count: 0,
		}
	}

	fn flash(&mut self, p: &usize) {
		self.grid[*p].has_flashed = true;
		self
			.get_adjacent_coords(p)
			.iter()
			.for_each(|&o| self.grid[o].energy_lvl += 1);
		self.flash_count += 1;
	}

	fn get_adjacent_coords(&self, p: &usize) -> Vec<usize> {
		let mut locations = Vec::new();

		let (x, y) = ((p / Octopi::WIDTH) as i32, (p % Octopi::WIDTH) as i32);

		for i in -1..=1 {
			for j in -1..=1 {
				if (i != 0 || j != 0)
					&& x + i >= 0 && x + i < Octopi::WIDTH as i32
					&& y + j >= 0 && y + j < Octopi::HEIGHT as i32
				{
					locations.push(((x + i) * Octopi::WIDTH as i32 + (y + j)) as usize);
				}
			}
		}

		locations
	}

	fn get_flashing_octopi(&self) -> Option<Vec<usize>> {
		match self
			.grid
			.iter()
			.enumerate()
			.filter(|(_, o)| o.energy_lvl > 9 && !o.has_flashed)
			.map(|(i, _)| i)
			.collect::<Vec<_>>()
		{
			vec if !vec.is_empty() => Some(vec),
			_ => None,
		}
	}

	fn step_once(&mut self) -> bool {
		self.grid.iter_mut().for_each(|o| {
			o.has_flashed = false;
			o.energy_lvl += 1
		});

		let mut flashes: HashSet<usize> = HashSet::new();

		while let Some(octopi) = self.get_flashing_octopi() {
			flashes.extend(octopi.iter());
			octopi.iter().for_each(|o| self.flash(o))
		}

		self
			.grid
			.iter_mut()
			.filter(|o| o.has_flashed)
			.for_each(|o| o.energy_lvl = 0);

		if flashes.len() == Octopi::SIZE {
			return true;
		}
		false
	}
}

fn main() -> Result<()> {
	let mut octopi = get_input("input.txt")?;
	let mut step = 0;

	let first_sync_step = loop {
		step += 1;
		if octopi.step_once() {
			break step;
		}
	};

	println!("answer: {}", first_sync_step);

	Ok(())
}

fn get_input(filename: impl AsRef<Path>) -> Result<Octopi> {
	let mut file = File::open(filename).with_context(|| "Can't open file")?;
	let mut buffer = String::new();
	file.read_to_string(&mut buffer)?;

	Ok(buffer.parse().unwrap())
}
