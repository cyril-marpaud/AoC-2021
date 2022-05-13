use anyhow::{Context, Error, Result};
use std::{
	self,
	fs::File,
	io::{prelude::*, BufReader},
	path::Path,
	str::FromStr,
};

#[derive(Debug)]
struct Display {
	digits: Vec<String>,
	output: Vec<String>,
}

impl FromStr for Display {
	type Err = Error;
	fn from_str(s: &str) -> Result<Self> {
		let mut dbl_vec = s
			.split('|')
			.map(|half_line| {
				half_line
					.split_whitespace()
					.map(|digit| digit.to_string())
					.collect()
			})
			.rev() // rev so that we can pop properly
			.collect::<Vec<Vec<String>>>();
		Ok(Display {
			digits: dbl_vec.pop().unwrap(),
			output: dbl_vec.pop().unwrap(),
		})
	}
}

fn main() -> Result<()> {
	let output_values = get_input("input.txt")?;

	println!("out: {:#?}", output_values);

	Ok(())
}

fn get_input(filename: impl AsRef<Path>) -> Result<Vec<Display>> {
	let file = File::open(filename).with_context(|| "Can't open file")?;
	let lines = BufReader::new(file).lines().map(Result::unwrap);

	Ok(lines.map(|l| l.parse().unwrap()).collect())
}
