use anyhow::{Context, Result};
use std::{
	self,
	cmp::Ordering,
	fs::File,
	io::{prelude::*, BufReader},
	path::Path,
};

const BIN_SIZE: usize = 12;

fn main() -> Result<()> {
	let input = get_input("input.txt")?;

	let most_common_bits = u32::from_str_radix(&get_most_common_bits(input), 2)?;
	let least_common_bits = !most_common_bits & ((1u32 << BIN_SIZE as u32) - 1);

	println!("answer: {}", most_common_bits * least_common_bits);
	Ok(())
}

fn get_most_common_bits(input: Vec<u32>) -> String {
	let input = input.iter();
	let mut ones = vec![0u32; BIN_SIZE];

	for u in input {
		for i in 0..BIN_SIZE {
			if u & 1 << i != 0 {
				ones[BIN_SIZE - 1 - i] += 1;
			}
		}
	}

	ones
		.iter()
		.map(|n| match n.cmp(&500) {
			Ordering::Greater | Ordering::Equal => '1',
			Ordering::Less => '0',
		})
		.collect::<String>()
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
