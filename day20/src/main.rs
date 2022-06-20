use std::{
	self,
	collections::VecDeque,
	fs::File,
	io::{prelude::*, BufReader},
	iter::repeat,
	path::Path,
};

use anyhow::{Context, Result};

#[derive(Debug)]
struct EnhancingAlgo(Vec<char>);

struct Picture {
	pic: VecDeque<VecDeque<char>>,
	bg: char,
	current_side: usize,
}

impl Picture {
	const PIC_SIDE: usize = 100;

	fn enhance(&mut self, e_algo: &EnhancingAlgo) {
		self.enlarge();
		self.recompute(&e_algo);
		self.update_bg(e_algo);
	}

	fn enlarge(&mut self) {
		let vd = VecDeque::from_iter(repeat(self.bg).take(self.current_side));
		self.pic.push_front(vd.clone());
		self.pic.push_back(vd);

		self.current_side += 2;

		for vd in &mut self.pic {
			vd.push_front(self.bg);
			vd.push_back(self.bg);
		}
	}

	fn get_pixels_count(&self) -> usize {
		self.pic.iter().flatten().filter(|&&el| el == '#').count()
	}

	fn recompute(&mut self, e_algo: &EnhancingAlgo) {
		let mut new_pic = VecDeque::new();

		for (i, outer_vd) in self.pic.iter().enumerate() {
			let mut line = VecDeque::<char>::new();

			for (j, _) in outer_vd.iter().enumerate() {
				let mut value = String::new();

				for i_c in i as isize - 1..=i as isize + 1 {
					for j_c in j as isize - 1..=j as isize + 1 {
						value.push(
							if i_c < 0
								|| j_c < 0 || i_c > self.current_side as isize - 1
								|| j_c > self.current_side as isize - 1
							{
								if self.bg == '.' {
									'0'
								} else {
									'1'
								}
							} else {
								if self.pic[i_c as usize][j_c as usize] == '.' {
									'0'
								} else {
									'1'
								}
							},
						);
					}
				}
				line.push_back(e_algo.0[usize::from_str_radix(&value, 2).unwrap()]);
			}
			new_pic.push_back(line);
		}
		self.pic = new_pic;
	}

	fn update_bg(&mut self, e_algo: &EnhancingAlgo) {
		self.bg = match (e_algo.0[0], e_algo.0[511]) {
			('.', _) => '.',
			('#', '#') => '#',
			_ => match self.bg {
				'.' => '#',
				_ => '.',
			},
		}
	}
}

fn main() -> Result<()> {
	let (e_algo, mut pic) = get_input("input.txt")?;

	const ENHANCEMENTS: usize = 2;
	(0..ENHANCEMENTS).for_each(|_| pic.enhance(&e_algo));

	println!("answer: {}", pic.get_pixels_count());

	Ok(())
}

fn get_input(filename: impl AsRef<Path>) -> Result<(EnhancingAlgo, Picture)> {
	let file = File::open(filename).with_context(|| "Can't open file")?;
	let mut lines = BufReader::new(file).lines().map(Result::unwrap);

	let e_algo = EnhancingAlgo(Vec::from_iter(lines.next().unwrap().chars()));

	let mut pic = VecDeque::new();
	for line in lines.by_ref().skip(1) {
		pic.push_back(VecDeque::from_iter(line.chars()));
	}

	let pic = Picture {
		pic,
		bg: '.',
		current_side: Picture::PIC_SIDE,
	};

	Ok((e_algo, pic))
}
