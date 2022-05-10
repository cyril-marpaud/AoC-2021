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
			.rev()
			.collect::<Vec<usize>>();

		Ok(Line {
			x1: line.pop().unwrap(),
			y1: line.pop().unwrap(),
			x2: line.pop().unwrap(),
			y2: line.pop().unwrap(),
		})
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

	fn map_line(&mut self, l: &Line) {
		let xrange: Vec<_> = if l.x1 == l.x2 {
			iter::repeat(l.x1)
				.take((i32::abs(l.y2 as i32 - l.y1 as i32) + 1) as usize)
				.collect()
		} else if l.x1 < l.x2 {
			(l.x1..=l.x2).collect()
		} else {
			(l.x2..=l.x1).rev().collect()
		};

		let yrange: Vec<_> = if l.y1 == l.y2 {
			iter::repeat(l.y1)
				.take((i32::abs(l.x2 as i32 - l.x1 as i32) + 1) as usize)
				.collect()
		} else if l.y1 < l.y2 {
			(l.y1..=l.y2).collect()
		} else {
			(l.y2..=l.y1).rev().collect()
		};

		xrange.iter().zip(yrange).for_each(|(x, y)| {
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

	// map each line on the floor
	lines.iter().for_each(|l| floor.map_line(l));

	//floor.display();

	println!("answer: {}", floor.count_dangerous_points());

	Ok(())
}

fn get_input(filename: impl AsRef<Path>) -> Result<Vec<Line>> {
	let file = File::open(filename).with_context(|| "Can't open file")?;
	let lines = BufReader::new(file).lines().map(Result::unwrap);

	Ok(lines.map(|l| l.parse().unwrap()).collect())
}
