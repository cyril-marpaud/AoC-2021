use std::{
	self,
	collections::{HashMap, HashSet},
	fs::File,
	io::{prelude::*, BufReader},
	path::Path,
};

use anyhow::{Context, Result};

#[derive(Debug)]
struct Cave(HashMap<String, HashSet<String>>);

impl Cave {
	fn extend_paths(&self, vec: Vec<Vec<String>>) -> (bool, Vec<Vec<String>>) {
		let mut has_grown = false;
		let mut extensions = Vec::<Vec<String>>::new();

		let (mut complete, incomplete): (Vec<_>, Vec<_>) =
			vec.into_iter().partition(|el| el.last().unwrap() == "end");

		incomplete.into_iter().for_each(|el| {
			self.0
				.get(el.last().unwrap())
				.unwrap()
				.iter()
				.filter(|cave| {
					Cave::is_big_cave(cave)
						|| !el.contains(cave) || Cave::has_unique_small_caves(el.clone())
				})
				.for_each(|step| {
					let mut temp_el = el.clone();
					temp_el.push(step.clone());
					extensions.push(temp_el);
					has_grown = true;
				});
		});

		complete.extend_from_slice(&extensions);
		(has_grown, complete)
	}

	fn is_big_cave(cave: &str) -> bool {
		cave.chars().nth(0).unwrap().is_uppercase()
	}

	fn has_unique_small_caves(vec: Vec<String>) -> bool {
		let mut uniq = HashSet::new();
		vec.iter()
			.filter(|cave| !Cave::is_big_cave(cave))
			.all(move |x| uniq.insert(x))
	}

	fn map_paths(&self) -> Vec<Vec<String>> {
		let mut paths = vec![vec!["start".to_owned()]];

		let mut extended = true;
		while extended == true {
			(extended, paths) = self.extend_paths(paths);
		}

		paths
	}

	fn new() -> Self {
		Cave(HashMap::new())
	}
}

fn main() -> Result<()> {
	let cave = get_input("input.txt")?;
	let paths = cave.map_paths();

	println!("answer: {}", paths.len());

	Ok(())
}

fn get_input(filename: impl AsRef<Path>) -> Result<Cave> {
	let file = File::open(filename).with_context(|| "Can't open file")?;
	let lines = BufReader::new(file).lines().map(Result::unwrap);

	let mut cave = Cave::new();

	lines.for_each(|l| {
		let mut l = l.split('-');
		let (a, b) = (l.next().unwrap().to_owned(), l.next().unwrap().to_owned());

		if b != "start" {
			cave.0
				.entry(a.clone())
				.or_insert(HashSet::new())
				.insert(b.clone());
		}
		if a != "start" {
			cave.0.entry(b).or_insert(HashSet::new()).insert(a);
		}
	});

	Ok(cave)
}
