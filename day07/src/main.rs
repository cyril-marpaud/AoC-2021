use anyhow::{Context, Result};
use std::{
	self,
	fs::File,
	io::{prelude::*, BufReader},
	path::Path,
};

#[derive(Debug)]
struct Crabs {
	distribution: [usize; Crabs::MAX_POSITION + 1],
}

impl Crabs {
	const MAX_POSITION: usize = 1788;

	fn new() -> Self {
		Crabs {
			distribution: [0; Crabs::MAX_POSITION + 1],
		}
	}

	fn compute_fuel_comsumption(&self, pos: usize) -> usize {
		self.distribution
			.iter()
			.enumerate()
			.map(|(p, &nb_crabs)| {
				let dist = pos.abs_diff(p);
				dist * (dist + 1) / 2 * nb_crabs // n first ints' sum is n(n+1)/2
			})
			.sum()
	}
}

fn main() -> Result<()> {
	let crabs = get_input("input.txt")?;

	let min_fuel = (0usize..=Crabs::MAX_POSITION)
		.map(|p| crabs.compute_fuel_comsumption(p))
		.min()
		.unwrap();

	println!("answer: {}", min_fuel);

	Ok(())
}

fn get_input(filename: impl AsRef<Path>) -> Result<Crabs> {
	let file = File::open(filename).with_context(|| "Can't open file")?;
	let mut lines = BufReader::new(file).lines().map(Result::unwrap);

	let mut crabs = Crabs::new();

	lines
		.next()
		.unwrap()
		.split(',')
		.for_each(|c| crabs.distribution[c.parse::<usize>().unwrap()] += 1);

	Ok(crabs)
}
