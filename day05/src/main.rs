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

impl Line {
	fn get_range(coords1: (usize, usize), coords2: (usize, usize)) -> Vec<usize> {
		if coords1.0 == coords2.0 {
			iter::repeat(coords1.0)
				.take((i32::abs(coords2.1 as i32 - coords1.1 as i32) + 1) as usize)
				.collect()
		} else if coords1.0 < coords2.0 {
			(coords1.0..=coords2.0).collect()
		} else {
			(coords2.0..=coords1.0).rev().collect()
		}
	}
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
		let (xrange, yrange) = (
			Line::get_range((l.x1, l.y1), (l.x2, l.y2)),
			Line::get_range((l.y1, l.x1), (l.y2, l.x2)),
		);

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
