use anyhow::{anyhow, Context, Result};
use std::{
	self,
	fs::File,
	io::{prelude::*, BufReader},
	ops::ControlFlow,
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

	let (draw, board) = match draws.iter().try_for_each(|d| {
		return boards.iter_mut().try_for_each(|b| {
			if let Some(coords) = b.has_number(d) {
				if b.is_winning(coords) {
					return ControlFlow::Break((d, b.data));
				}
			}
			return ControlFlow::Continue(());
		});
	}) {
		ControlFlow::Continue(()) => Err(anyhow!("No winner")),
		ControlFlow::Break(db) => Ok(db),
	}?;

	println!(
		"answer: {}",
		draw * board.iter().map(|n| n.unwrap_or(0)).sum::<u32>()
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
