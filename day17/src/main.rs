use std::{self, fs::File, io::prelude::Read, ops::RangeInclusive, path::Path};

use anyhow::{Context, Result};

#[derive(Debug)]
struct Target {
	x: RangeInclusive<usize>,
	y: RangeInclusive<isize>,
}

#[derive(Debug)]
struct Probe<'a> {
	pos_x: usize,
	pos_y: isize,
	speed_x: usize,
	speed_y: isize,
	target: &'a Target,
}

impl<'a> Probe<'a> {
	fn is_on_target(&self) -> bool {
		self.target.x.contains(&self.pos_x) && self.target.y.contains(&self.pos_y)
	}

	fn has_overshot(&self) -> bool {
		&self.pos_x > self.target.x.end() || &self.pos_y < self.target.y.start()
	}

	fn new(initial_speed: (usize, isize), target: &'a Target) -> Self {
		Probe {
			pos_x: 0,
			pos_y: 0,
			speed_x: initial_speed.0,
			speed_y: initial_speed.1,
			target,
		}
	}

	fn step(&mut self) {
		self.pos_x += self.speed_x;
		self.pos_y += self.speed_y;
		self.speed_x = self.speed_x.saturating_sub(1);
		self.speed_y -= 1;
	}
}

fn main() -> Result<()> {
	let target = get_input("input.txt")?;

	let (speed_x_min, speed_x_max) = (
		((((1 + 8 * target.x.start()) as f32).sqrt() - 1.0) / 2.0).ceil() as usize,
		target.x.end(),
	);

	let (speed_y_min, speed_y_max) = (target.y.start(), target.y.start().abs() - 1);

	let mut max_height = 0;

	for speed_x in speed_x_min..=*speed_x_max {
		for speed_y in *speed_y_min..=speed_y_max {
			let mut probe = Probe::new((speed_x, speed_y), &target);

			let mut temp_max_height = 0;

			while !probe.has_overshot() {
				probe.step();

				temp_max_height = probe.pos_y.max(temp_max_height);

				if probe.is_on_target() {
					max_height = max_height.max(temp_max_height);
					break;
				}
			}
		}
	}

	println!("answer: {max_height}");

	Ok(())
}

fn get_input(filename: impl AsRef<Path>) -> Result<Target> {
	let mut file = File::open(filename).with_context(|| "Can't open file")?;
	let mut buffer = String::new();
	file.read_to_string(&mut buffer)?;

	let mut range_iter = buffer.split(|c| c == '=' || c == ',');

	let mut x_range = range_iter.by_ref().skip(1).next().unwrap().split("..");
	let (x_start, x_end) = (
		x_range.next().unwrap().parse().unwrap(),
		x_range.next().unwrap().parse().unwrap(),
	);

	let mut y_range = range_iter.skip(1).next().unwrap().split("..");
	let (y_start, y_end) = (
		y_range.next().unwrap().parse().unwrap(),
		y_range.next().unwrap().parse().unwrap(),
	);

	Ok(Target {
		x: x_start..=x_end,
		y: y_start..=y_end,
	})
}
