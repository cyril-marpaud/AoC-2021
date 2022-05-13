use anyhow::{Context, Error, Result};
use std::{
	self,
	fs::File,
	io::{prelude::*, BufReader},
	path::Path,
	str::FromStr,
};

#[derive(Debug)]
struct Display {
	digits: Vec<String>,
	output: Vec<String>,
}

impl FromStr for Display {
	type Err = Error;
	fn from_str(s: &str) -> Result<Self> {
		let mut dbl_vec = s
			.split('|')
			.map(|half_line| {
				half_line
					.split_whitespace()
					.map(|digit| digit.to_string())
					.collect()
			})
			.rev() // rev so that we can pop properly
			.collect::<Vec<Vec<String>>>();
		Ok(Display {
			digits: dbl_vec.pop().unwrap(),
			output: dbl_vec.pop().unwrap(),
		})
	}
}

impl Display {
	// the gap is the number of segments that differ between two digits
	fn compute_gap(first: &str, second: &str) -> usize {
		first
			.chars()
			.fold(0, |acc, el| if second.contains(el) { acc } else { acc + 1 })
			+ second
				.chars()
				.fold(0, |acc, el| if first.contains(el) { acc } else { acc + 1 })

		/* Gap Table

		  0 1 2 3 4 5 6 7 8 9
		0 0 4 3 3 4 3 2 3 1 2
		1 4 0 5 3 2 5 6 1 5 4
		2 3 5 0 2 5 4 3 4 2 3
		3 3 3 2 0 3 2 3 2 2 1
		4 4 2 5 3 0 3 4 3 3 2
		5 3 5 4 2 3 0 1 4 2 1
		6 2 6 3 3 4 1 0 5 1 2
		7 3 1 4 2 3 4 5 0 4 3
		8 1 5 2 2 3 2 1 4 0 1
		9 2 4 3 1 2 1 2 3 1 0

		*/
	}

	fn find_digit_position(&self, comparator: &str, gap: usize) -> usize {
		self
			.digits
			.iter()
			.position(|d| Display::compute_gap(d, comparator) == gap)
			.unwrap()
	}

	fn compute_output(&mut self) -> u32 {
		let mut numbers: [String; 10] = Default::default();

		// 1 is the only 2-segments digit
		numbers[1] = self
			.digits
			.swap_remove(self.digits.iter().position(|d| d.len() == 2).unwrap());
		// 4 is the only 4 segments digit
		numbers[4] = self
			.digits
			.swap_remove(self.digits.iter().position(|d| d.len() == 4).unwrap());
		// 7 is the only 3 segments digit
		numbers[7] = self
			.digits
			.swap_remove(self.digits.iter().position(|d| d.len() == 3).unwrap());
		// 8 is the only 7 segments digit
		numbers[8] = self
			.digits
			.swap_remove(self.digits.iter().position(|d| d.len() == 7).unwrap());

		// 3 is the only remaining digit with a gap of 3 when compared to 1
		numbers[3] = self
			.digits
			.swap_remove(self.find_digit_position(&numbers[1], 3));
		// 6 is the only remaining digit with a gap of 6 when compared to 1
		numbers[6] = self
			.digits
			.swap_remove(self.find_digit_position(&numbers[1], 6));

		// 0 is the only remaining digit with a gap of 4 when compared to 4
		numbers[0] = self
			.digits
			.swap_remove(self.find_digit_position(&numbers[4], 4));
		// 2 is the only remaining digit with a gap of 5 when compared to 4
		numbers[2] = self
			.digits
			.swap_remove(self.find_digit_position(&numbers[4], 5));
		// 5 is the only remaining digit with a gap of 3 when compared to 4
		numbers[5] = self
			.digits
			.swap_remove(self.find_digit_position(&numbers[4], 3));
		// 9 is the only remaining digit with a gap of 2 when compared to 4
		numbers[9] = self
			.digits
			.swap_remove(self.find_digit_position(&numbers[4], 2));

		self
			.output
			.iter()
			.map(|o| {
				format!(
					"{}",
					numbers
						.iter()
						.position(|d| Display::compute_gap(d, o) == 0)
						.unwrap()
				)
			})
			.collect::<String>()
			.parse()
			.unwrap()
	}
}

fn main() -> Result<()> {
	let mut displays = get_input("input.txt")?;

	println!(
		"answer: {}",
		displays.iter_mut().map(|d| d.compute_output()).sum::<u32>()
	);

	Ok(())
}

fn get_input(filename: impl AsRef<Path>) -> Result<Vec<Display>> {
	let file = File::open(filename).with_context(|| "Can't open file")?;
	let lines = BufReader::new(file).lines().map(Result::unwrap);

	Ok(lines.map(|l| l.parse().unwrap()).collect())
}

#[test]
fn test_compute_gap() {
	assert_eq!(Display::compute_gap("a", "b"), 2);
	assert_eq!(Display::compute_gap("ab", "b"), 1);
	assert_eq!(Display::compute_gap("ba", "b"), 1);
	assert_eq!(Display::compute_gap("febca", "feg"), 4);
	assert_eq!(Display::compute_gap("cdfbg", "baecgfd"), 2);
	assert_eq!(Display::compute_gap("cdfbg", "fgcbd"), 0);
}
