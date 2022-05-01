use anyhow::{Context, Result};
use std::{
	self,
	fs::File,
	io::{prelude::*, BufReader},
	path::Path,
};

const BIN_SIZE: usize = 12;

fn main() -> Result<()> {
	let input = get_input("input.txt")?;
	let mut input = input.iter();
	let mut ones = vec![0u32; BIN_SIZE];

	while let Some(u) = input.next() {
		for i in 0..BIN_SIZE {
			if u & 1 << i != 0 {
				ones[BIN_SIZE - 1 - i] += 1;
			}
		}
	}

	let gamma_rate_string = ones
		.into_iter()
		.map(|n| if n > 500 { '1' } else { '0' })
		.collect::<String>();

	let gamma_rate = u32::from_str_radix(&gamma_rate_string, 2)?;
	let epsilon_rate = !gamma_rate & 2u32.pow(BIN_SIZE as u32) - 1;

	println!("answer: {}", gamma_rate * epsilon_rate);
	Ok(())
}

fn get_input(filename: impl AsRef<Path>) -> Result<Vec<u32>> {
	let file = File::open(filename).with_context(|| "Can't open file")?;
	BufReader::new(file)
		.lines()
		.map(|l| {
			u32::from_str_radix(&l.unwrap(), 2)
				.with_context(|| "Can't parse line into interger as binary")
		})
		.collect()
}
