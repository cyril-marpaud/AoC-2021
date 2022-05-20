use anyhow::{Context, Result};
use std::{
	self,
	fs::File,
	io::{prelude::*, BufReader},
	path::Path,
};

#[derive(Debug)]
struct Line(String);

impl Line {
	fn build_vec(&self) -> Option<Vec<char>> {
		let mut chunk = Vec::new();

		for char in self.0.chars() {
			match char {
				'(' => chunk.push(')'),
				'[' => chunk.push(']'),
				'{' => chunk.push('}'),
				'<' => chunk.push('>'),
				character => {
					match chunk.pop() {
						Some(expected) if character == expected => (),
						_ => return None,
					};
				}
			}
		}

		chunk.reverse();
		Some(chunk)
	}

	fn compute_score(&self) -> Option<u64> {
		self.build_vec().map(|vec| {
			vec.iter().fold(0, |acc, el| {
				5 * acc
					+ match el {
						')' => 1,
						']' => 2,
						'}' => 3,
						'>' => 4,
						_ => panic!(),
					}
			})
		})
	}
}

fn main() -> Result<()> {
	let lines = get_input("input.txt")?;

	let mut scores = lines
		.iter()
		.filter_map(|l| l.compute_score())
		.collect::<Vec<_>>();

	scores.sort_unstable();

	println!("answer: {}", scores[scores.len() / 2]);

	Ok(())
}

fn get_input(filename: impl AsRef<Path>) -> Result<Vec<Line>> {
	let file = File::open(filename).with_context(|| "Can't open file")?;
	let lines = BufReader::new(file).lines().map(Result::unwrap);

	Ok(lines.map(Line).collect())
}
