use anyhow::{Context, Result};
use std::{
	self,
	fs::File,
	io::{prelude::*, BufReader},
	path::Path,
};

fn main() -> Result<()> {
	let output_values = get_input("input.txt")?;

	println!(
		"answer: {}",
		output_values.iter().fold(0, |acc, el| {
			acc + el
				.iter()
				.filter(|sel| sel.len() == 2 || sel.len() == 3 || sel.len() == 4 || sel.len() == 7)
				.count()
		})
	);

	Ok(())
}

fn get_input(filename: impl AsRef<Path>) -> Result<Vec<Vec<String>>> {
	let file = File::open(filename).with_context(|| "Can't open file")?;
	let lines = BufReader::new(file).lines().map(Result::unwrap);

	Ok(lines
		.map(|l| {
			l.split(" | ")
				.skip(1)
				.next()
				.unwrap()
				.split_whitespace()
				.map(|s| s.to_string())
				.collect()
		})
		.collect())
}
