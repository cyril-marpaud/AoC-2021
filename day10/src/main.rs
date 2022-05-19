use anyhow::{Context, Result};
use std::{
	self,
	collections::HashMap,
	fs::File,
	io::{prelude::*, BufReader},
	path::Path,
};

#[derive(Debug)]
struct Line(String);

impl Line {
	fn is_corrupt(&self) -> Option<char> {
		let mut chunk = Vec::new();

		for char in self.0.chars() {
			match char {
				'(' => chunk.push(')'),
				'[' => chunk.push(']'),
				'{' => chunk.push('}'),
				'<' => chunk.push('>'),
				character @ _ => {
					match chunk.pop() {
						Some(expected) if character == expected => (),
						_ => return Some(character),
					};
				}
			}
		}

		None
	}
}

fn main() -> Result<()> {
	let lines = get_input("input.txt")?;

	let chars_value = HashMap::from([
		(None, 0),
		(Some(')'), 3),
		(Some(']'), 57),
		(Some('}'), 1197),
		(Some('>'), 25137),
	]);

	println!(
		"answer: {}",
		lines
			.iter()
			.map(|l| chars_value.get(&l.is_corrupt()).expect("unknown character"))
			.sum::<u32>()
	);

	Ok(())
}

fn get_input(filename: impl AsRef<Path>) -> Result<Vec<Line>> {
	let file = File::open(filename).with_context(|| "Can't open file")?;
	let lines = BufReader::new(file).lines().map(Result::unwrap);

	Ok(lines.map(|l| Line(l)).collect())
}
