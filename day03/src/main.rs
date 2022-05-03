use anyhow::{Context, Result};
use std::{
	self,
	cmp::Ordering,
	fs::File,
	io::{prelude::*, BufReader},
	path::Path,
};

enum BitCriteria {
	Oxygen,
	CO2,
}

const BIN_SIZE: usize = 12;

fn main() -> Result<()> {
	let input = get_input("input.txt")?;

	let oxygen_rating = get_rating(&input[..], BitCriteria::Oxygen);
	let co2_rating = get_rating(&input[..], BitCriteria::CO2);

	println!("answer: {}", oxygen_rating * co2_rating);
	Ok(())
}

fn get_rating(input: &[u32], crit: BitCriteria) -> u32 {
	let mut i = 1;
	let mut vec_input = input.to_vec();

	while vec_input.len() != 1 {
		let common_bits = get_common_bits(&vec_input, &crit);
		let mut temp_vec_input = Vec::new();

		for el in vec_input {
			if (el >> (BIN_SIZE - i)) & 1 == (common_bits >> (BIN_SIZE - i)) & 1 {
				temp_vec_input.push(el);
			}
		}

		vec_input = temp_vec_input;
		i += 1;
	}

	vec_input[0]
}

fn get_common_bits(input: &[u32], crit: &BitCriteria) -> u32 {
	let half_input_length = (input.len() as f32) / 2f32;
	let input = input.iter();
	let mut ones = vec![0f32; BIN_SIZE];

	for u in input {
		for i in 0..BIN_SIZE {
			if u & 1 << i != 0 {
				ones[BIN_SIZE - 1 - i] += 1f32;
			}
		}
	}

	let mut common_bits = u32::from_str_radix(
		&ones
			.iter()
			.map(|n| match n.partial_cmp(&half_input_length) {
				Some(Ordering::Greater) => '1',
				Some(Ordering::Equal) => '1',
				Some(Ordering::Less) => '0',
				_ => panic!(),
			})
			.collect::<String>(),
		2,
	)
	.unwrap();

	if let BitCriteria::CO2 = crit {
		common_bits = !common_bits & ((1 << BIN_SIZE) - 1);
	}

	common_bits
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
