use anyhow::{Context, Result};
use std::{
	self,
	fs::File,
	io::{prelude::*, BufReader},
	path::Path,
};

#[derive(Debug)]
struct Lanternfish {
	timer: usize,
}

impl Lanternfish {
	fn new(t: Option<usize>) -> Self {
		Lanternfish {
			timer: t.unwrap_or(8),
		}
	}

	fn simulate_one_day(&mut self) -> Option<Lanternfish> {
		if self.timer == 0 {
			self.timer = 6;
			Some(Lanternfish::new(None))
		} else {
			self.timer -= 1;
			None
		}
	}
}

fn main() -> Result<()> {
	let mut fishes = get_input("input.txt")?;
	let mut new_fishes = Vec::new();
	let simulation_days = 80;

	(1..=simulation_days).for_each(|_| {
		fishes.iter_mut().for_each(|f| {
			if let Some(fish) = f.simulate_one_day() {
				new_fishes.push(fish)
			}
		});
		fishes.append(&mut new_fishes);
	});

	println!("answer: {}", fishes.len());
	Ok(())
}

fn get_input(filename: impl AsRef<Path>) -> Result<Vec<Lanternfish>> {
	let file = File::open(filename).with_context(|| "Can't open file")?;
	let mut lines = BufReader::new(file).lines().map(Result::unwrap);

	Ok(lines
		.next()
		.unwrap()
		.split(',')
		.map(|t| Lanternfish::new(Some(t.parse().unwrap())))
		.collect())
}
