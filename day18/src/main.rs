use std::{
	self,
	fmt::Display,
	fs::File,
	io::{prelude::*, BufReader},
	ops::Add,
	path::Path,
	str::FromStr,
};

use anyhow::{Context, Result};
use regex::Regex;

const DEC: u32 = 10;
const EXPLOSION_NEST_LVL: u32 = 4;

struct Sfn {
	left: SfnOperand,
	right: SfnOperand,
}

enum SfnOperand {
	Literal(u32),
	Sfn(Box<Sfn>),
}

impl Display for SfnOperand {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		return write!(
			f,
			"{}",
			match &self {
				SfnOperand::Literal(l) => l.to_string(),
				SfnOperand::Sfn(sfn) => sfn.to_string(),
			}
		);
	}
}

impl SfnOperand {
	fn get_magnitude(&self) -> u32 {
		match self {
			SfnOperand::Literal(l) => *l,
			SfnOperand::Sfn(sfn) => sfn.get_magnitude(),
		}
	}
}

impl Add for Sfn {
	type Output = Sfn;
	fn add(self, rhs: Self) -> Self::Output {
		Sfn::reduce(format!("[{},{}]", self, rhs).parse().unwrap())
	}
}

impl Display for Sfn {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		return write!(f, "[{},{}]", &self.left, &self.right);
	}
}

impl FromStr for Sfn {
	type Err = anyhow::Error;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(Sfn::parse_sfn(&mut s.chars().skip(1)))
	}
}

impl Sfn {
	fn get_magnitude(&self) -> u32 {
		3 * &self.left.get_magnitude() + 2 * &self.right.get_magnitude()
	}

	fn parse_sfn(iter: &mut impl Iterator<Item = char>) -> Sfn {
		Sfn {
			left: Sfn::parse_sfn_side(iter),
			right: Sfn::parse_sfn_side(iter),
		}
	}

	fn parse_sfn_side(iter: &mut impl Iterator<Item = char>) -> SfnOperand {
		let operand = match iter.next().unwrap() {
			'[' => {
				let sfn = Sfn::parse_sfn(iter);
				SfnOperand::Sfn(Box::new(sfn))
			}
			c if c.is_ascii_digit() => SfnOperand::Literal(c.to_digit(DEC).unwrap()),
			_ => panic!(),
		};
		iter.next(); // skip the ',' / ']' char respectively following left / right side
		operand
	}

	fn reduce(sfn: Sfn) -> Sfn {
		let mut string_self = sfn.to_string();

		loop {
			// detect explosion
			let mut nest_lvl = 0;
			let mut explosion = None;

			for (i, c) in string_self.chars().enumerate().by_ref() {
				match c {
					'[' => nest_lvl += 1,
					']' => nest_lvl -= 1,
					_ => (),
				}
				if nest_lvl > EXPLOSION_NEST_LVL {
					let next_cl_bracket = string_self[i..].find(']').unwrap();
					match string_self[i..].find('[') {
						Some(next_op_bracket) if next_op_bracket < next_cl_bracket => (),
						_ => {
							explosion = Some((i - 1, i + next_cl_bracket));
							break;
						}
					}
				}
			}

			// explode
			if let Some((i, j)) = explosion {
				let mut split = string_self[i + 1..j].split(',');
				let (left, right) = (
					split.next().unwrap().parse::<u32>().unwrap(),
					split.next().unwrap().parse::<u32>().unwrap(),
				);

				// step 1: replace right
				if let Some(mut i_next) = string_self[j..].find(|c: char| c.is_ascii_digit()) {
					i_next += j;
					let j_next = string_self[i_next..]
						.find(|c: char| !c.is_ascii_digit())
						.unwrap() + i_next;

					string_self.replace_range(
						i_next..j_next,
						&(string_self[i_next..j_next].parse::<u32>().unwrap() + right).to_string(),
					);
				}

				// step 2: replace exploding pair
				string_self.replace_range(i..=j, "0");

				// step 3: replace left
				if let Some(j_prev) = string_self[..i].rfind(|c: char| c.is_ascii_digit()) {
					let i_prev = string_self[..j_prev]
						.rfind(|c: char| !c.is_ascii_digit())
						.unwrap();
					string_self.replace_range(
						i_prev + 1..=j_prev,
						&(string_self[i_prev + 1..=j_prev].parse::<u32>().unwrap() + left)
							.to_string(),
					);
				}
				continue;
			}

			// detect split
			let re = Regex::new("[0-9]{2,}").unwrap();
			if let Some(split) = re.find(&string_self.clone()) {
				// split
				let value = string_self[split.start()..split.end()]
					.parse::<u32>()
					.unwrap();
				string_self.replace_range(
					split.start()..split.end(),
					&format!("[{},{}]", value / 2, (value + 1) / 2),
				);
				continue;
			}
			break;
		}
		string_self.parse().unwrap()
	}
}

fn main() -> Result<()> {
	let vec_sfn = get_input("input.txt")?;
	let sfn = vec_sfn.into_iter().reduce(|acc, el| acc + el).unwrap();

	println!("answer: {}", sfn.get_magnitude());

	Ok(())
}

fn get_input(filename: impl AsRef<Path>) -> Result<Vec<Sfn>> {
	let file = File::open(filename).with_context(|| "Can't open file")?;
	BufReader::new(file)
		.lines()
		.map(|l| l.unwrap().parse())
		.collect()
}
