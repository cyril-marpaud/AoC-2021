use anyhow::{anyhow, Context, Result};
use std::{
	self,
	fs::File,
	io::{prelude::*, BufReader},
	path::Path,
	str::FromStr,
};

const BOARD_WIDTH: usize = 5;
const BOARD_HEIGHT: usize = 5;
const BOARD_SIZE: usize = BOARD_WIDTH * BOARD_HEIGHT;

#[derive(Debug)]
struct Board<const N: usize> {
	data: [u32; N],
}

impl<const N: usize> FromStr for Board<N> {
	type Err = anyhow::Error;

	fn from_str(s: &str) -> Result<Self> {
		let data: [u32; N] = s
			.split_whitespace()
			.map(|n| n.parse::<u32>().unwrap())
			.collect::<Vec<u32>>()
			.try_into()
			.unwrap();

		Ok(Board { data })
	}
}

fn main() -> Result<()> {
	let (draws, boards) = get_input("input.txt")?;

	println!("draws: {:?}", draws);
	println!("boards: {:?}", boards);

	Ok(())
}

fn get_input(filename: impl AsRef<Path>) -> Result<(Vec<u32>, Vec<Board<BOARD_SIZE>>)> {
	let file = File::open(filename).with_context(|| "Can't open file")?;
	let mut lines = BufReader::new(file).lines().map(|l| l.unwrap());

	Ok((
		lines
			.next()
			.unwrap()
			.split(',')
			.map(|n| n.parse().unwrap())
			.collect(),
		lines
			.filter(|l| !l.is_empty())
			.collect::<Vec<String>>()
			.chunks(BOARD_HEIGHT)
			.map(|c| c.join(" ").parse().unwrap())
			.collect(),
	))
}
