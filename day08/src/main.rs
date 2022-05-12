use anyhow::{Context, Result};
use std::{
	self,
	fs::File,
	io::{prelude::*, BufReader},
	path::Path,
};

fn main() -> Result<()> {
	let output_values = get_input("input.txt")?;

	println!("out: {:#?}", output_values);

	Ok(())
}

fn get_input(filename: impl AsRef<Path>) -> Result<Vec<(Vec<String>, Vec<String>)>> {
	let file = File::open(filename).with_context(|| "Can't open file")?;
	let lines = BufReader::new(file).lines().map(Result::unwrap);

	Ok(lines
		.map(|line| {
			let mut dbl_vec = line
				.split('|')
				.map(|half_line| {
					half_line
						.split_whitespace()
						.map(|digit| digit.to_string())
						.collect()
				})
				.rev() // rev so that we can pop properly
				.collect::<Vec<Vec<String>>>();
			(dbl_vec.pop().unwrap(), dbl_vec.pop().unwrap())
		})
		.collect())
}
