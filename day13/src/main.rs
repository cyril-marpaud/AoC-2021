use std::{
	self,
	cmp::{Ord, Ordering, PartialOrd},
	fmt::Display,
	fs::File,
	io::{prelude::*, BufReader},
	path::Path,
};

use anyhow::{Context, Result};

#[derive(Debug)]
struct Fold(char, u32);

struct Grid([char; Grid::GRID_SIZE]);

impl Grid {
	const GRID_HEIGHT: usize = 6;
	const GRID_WIDTH: usize = 39;
	const GRID_SIZE: usize = Grid::GRID_WIDTH * Grid::GRID_HEIGHT;

	fn new() -> Self {
		Grid(['_'; Grid::GRID_SIZE])
	}
}

impl Display for Grid {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.0
			.chunks(Grid::GRID_WIDTH)
			.try_for_each(|el| writeln!(f, "{:?}", el))?;
		Ok(())
	}
}

#[derive(Debug, Eq)]
struct Point(u32, u32);

impl Point {
	fn fold(&mut self, fold: &Fold) {
		match fold.0 {
			'x' => {
				if let Ordering::Less = fold.1.cmp(&self.0) {
					self.0 = 2 * fold.1 - self.0;
				}
			}
			'y' => {
				if let Ordering::Less = fold.1.cmp(&self.1) {
					self.1 = 2 * fold.1 - self.1;
				}
			}
			_ => panic!(),
		};
	}
}

impl PartialEq for Point {
	fn eq(&self, other: &Self) -> bool {
		self.0 == other.0 && self.1 == other.1
	}
}

impl PartialOrd for Point {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		match self.0.cmp(&other.0) {
			Ordering::Greater => Some(Ordering::Greater),
			Ordering::Less => Some(Ordering::Less),
			Ordering::Equal => Some(self.1.cmp(&other.1)),
		}
	}
}

impl Ord for Point {
	fn cmp(&self, other: &Self) -> Ordering {
		self.partial_cmp(other).unwrap()
	}
}

fn main() -> Result<()> {
	let (mut points, folds) = get_input("input.txt")?;

	folds.iter().for_each(|fold| {
		points.iter_mut().for_each(|point| point.fold(fold));
		points.sort();
		points.dedup();
	});

	let mut grid = Grid::new();
	points
		.into_iter()
		.for_each(|Point(x, y)| grid.0[y as usize * Grid::GRID_WIDTH + x as usize] = '#');

	println!("answer:\n{}", grid);

	Ok(())
}

fn get_input(filename: impl AsRef<Path>) -> Result<(Vec<Point>, Vec<Fold>)> {
	let file = File::open(filename).with_context(|| "Can't open file")?;
	let mut lines = BufReader::new(file).lines().map(Result::unwrap);

	let mut points = Vec::<Point>::new();
	let mut folds = Vec::<Fold>::new();

	lines.by_ref().take_while(|l| !l.is_empty()).for_each(|l| {
		let mut l = l.split(',');
		points.push(Point(
			l.next().unwrap().parse().unwrap(),
			l.next().unwrap().parse().unwrap(),
		));
	});

	lines.for_each(|l| {
		let mut l = l.split('=');
		folds.push(Fold(
			l.next().unwrap().chars().last().unwrap(),
			l.next().unwrap().parse().unwrap(),
		));
	});

	Ok((points, folds))
}
