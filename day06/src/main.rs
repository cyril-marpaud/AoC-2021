use anyhow::{Context, Result};
use std::{
	self,
	fs::File,
	io::{prelude::*, BufReader},
	path::Path,
};

#[derive(Debug)]
struct Lanternfishes {
	numbers: [usize; Lanternfishes::TIMER_MAX + 1],
}

impl Lanternfishes {
	const TIMER_MAX: usize = 8;

	fn new() -> Self {
		Lanternfishes {
			numbers: [0; Lanternfishes::TIMER_MAX + 1],
		}
	}

	fn simulate_one_day(&mut self) {
		let reset_fishes = self.numbers[0];
		self.numbers.rotate_left(1);
		self.numbers[6] += reset_fishes;
	}
}

fn main() -> Result<()> {
	let mut fishes = get_input("input.txt")?;
	let simulation_days = 256;

	(1..=simulation_days).for_each(|_| {
		fishes.simulate_one_day();
	});

	println!("answer: {}", fishes.numbers.iter().sum::<usize>());
	Ok(())
}

fn get_input(filename: impl AsRef<Path>) -> Result<Lanternfishes> {
	let file = File::open(filename).with_context(|| "Can't open file")?;
	let mut lines = BufReader::new(file).lines().map(Result::unwrap);

	let mut fishes = Lanternfishes::new();

	lines
		.next()
		.unwrap()
		.split(',')
		.for_each(|f| fishes.numbers[f.parse::<usize>().unwrap()] += 1);

	Ok(fishes)
}
