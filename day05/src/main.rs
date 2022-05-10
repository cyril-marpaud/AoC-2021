use anyhow::{Context, Result};
use std::{
	self,
	fs::File,
	io::{prelude::*, BufReader},
	iter,
	path::Path,
	str::FromStr,
};

#[derive(Debug)]
struct Line {
	x1: usize,
	y1: usize,
	x2: usize,
	y2: usize,
}

impl FromStr for Line {
	type Err = anyhow::Error;

	fn from_str(s: &str) -> Result<Self> {
		let mut line = s
			.replace(" -> ", ",")
			.split(',')
			.map(|n| n.parse().unwrap())
			.collect::<Vec<usize>>();

		let (mut y2, mut x2, mut y1, mut x1) = (
			line.pop().unwrap(),
			line.pop().unwrap(),
			line.pop().unwrap(),
			line.pop().unwrap(),
		);

		if x1 == x2 && y1 > y2 {
			let temp = y1;
			y1 = y2;
			y2 = temp;
		} else if y1 == y2 && x1 > x2 {
			let temp = x1;
			x1 = x2;
			x2 = temp;
		}

		Ok(Line { x1, y1, x2, y2 })
	}
}

#[derive(Debug)]
struct Floor {
	data: [u32; Floor::FLOOR_SIZE],
}

impl Floor {
	const FLOOR_WIDTH: usize = 1000;
	const FLOOR_HEIGHT: usize = 1000;
	const FLOOR_SIZE: usize = Floor::FLOOR_WIDTH * Floor::FLOOR_HEIGHT;

	fn new() -> Self {
		Floor {
			data: [0; Floor::FLOOR_SIZE],
		}
	}

	fn map_straight_line(&mut self, l: &Line) {
		let range: Vec<_> = if l.x1 == l.x2 {
			iter::repeat(l.x1)
				.take(l.y2 - l.y1 + 1)
				.zip(l.y1..=l.y2)
				.collect()
		} else if l.y1 == l.y2 {
			(l.x1..=l.x2)
				.zip(iter::repeat(l.y1).take(l.x2 - l.x1 + 1))
				.collect()
		} else {
			panic!("Not a straight line");
		};

		range.iter().for_each(|(x, y)| {
			self.data[x * Floor::FLOOR_WIDTH + y] += 1;
		});
	}

	fn count_dangerous_points(&self) -> usize {
		self.data.iter().filter(|&n| n > &1).count()
	}

	fn display(&self) {
		(0..Floor::FLOOR_SIZE)
			.collect::<Vec<_>>()
			.chunks(Floor::FLOOR_WIDTH)
			.for_each(|c| {
				c.iter().for_each(|&l| print!("{}", self.data[l]));
				println!();
			});
	}
}

fn main() -> Result<()> {
	let lines = get_input("input.txt")?;
	let mut floor = Floor::new();

	// map each straight line on the floor
	lines
		.iter()
		.filter(|l| l.x1 == l.x2 || l.y1 == l.y2) // straight lines only
		.for_each(|l| floor.map_straight_line(l));

	//floor.display();

	println!("answer: {}", floor.count_dangerous_points());

	Ok(())
}

fn get_input(filename: impl AsRef<Path>) -> Result<Vec<Line>> {
	let file = File::open(filename).with_context(|| "Can't open file")?;
	let lines = BufReader::new(file).lines().map(Result::unwrap);

	Ok(lines.map(|l| l.parse().unwrap()).collect())
}
