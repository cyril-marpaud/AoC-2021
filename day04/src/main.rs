use anyhow::{Context, Result};
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
	data: [Option<u32>; N],
}

impl<const N: usize> FromStr for Board<N> {
	type Err = anyhow::Error;

	fn from_str(s: &str) -> Result<Self> {
		let data: [Option<u32>; N] = s
			.split_whitespace()
			.map(|n| Some(n.parse::<u32>().unwrap()))
			.collect::<Vec<Option<u32>>>()
			.try_into()
			.unwrap();

		Ok(Board { data })
	}
}

impl<const N: usize> Board<N> {
	fn has_number(&mut self, number: &u32) -> Option<(usize, usize)> {
		match self.data.iter().position(|&x| x == Some(*number)) {
			Some(p) => {
				self.data[p] = None;
				Some((p as usize / BOARD_WIDTH, p as usize % BOARD_WIDTH))
			}
			None => None,
		}
	}

	fn is_winning(&self, (row, col): (usize, usize)) -> bool {
		self.data
			.iter()
			.skip(BOARD_WIDTH * row)
			.take(BOARD_WIDTH)
			.all(Option::is_none)
			|| self
				.data
				.iter()
				.skip(col)
				.step_by(BOARD_WIDTH)
				.all(Option::is_none)
	}
}

fn main() -> Result<()> {
	let (draws, mut boards) = get_input("input.txt")?;
	let mut draws = draws.iter();

	// Determine the last winning board
	let mut board = loop {
		let draw = draws.next().unwrap();
		let mut temp_boards = Vec::new();

		for board in &mut boards {
			match board.has_number(draw) {
				None => temp_boards.push(Board { data: board.data }),
				Some(coords) if !board.is_winning(coords) => {
					temp_boards.push(Board { data: board.data })
				}
				_ => (),
			}
		}

		if temp_boards.len() != 1 {
			boards = temp_boards;
		} else {
			break temp_boards.pop().unwrap();
		}
	};

	// Play until the last board wins
	let (draw, board) = loop {
		let draw = draws.next().unwrap();
		match board.has_number(draw) {
			Some(coords) if board.is_winning(coords) => break (draw, board),
			_ => (),
		}
	};

	println!(
		"answer: {}",
		draw * board.data.iter().map(|n| n.unwrap_or(0)).sum::<u32>()
	);

	Ok(())
}

fn get_input(filename: impl AsRef<Path>) -> Result<(Vec<u32>, Vec<Board<BOARD_SIZE>>)> {
	let file = File::open(filename).with_context(|| "Can't open file")?;
	let mut lines = BufReader::new(file).lines().map(Result::unwrap);

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
