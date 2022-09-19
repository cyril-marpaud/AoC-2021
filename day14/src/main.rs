#![feature(test)]
extern crate test;

use std::{
	self,
	collections::HashMap,
	fs::File,
	io::{prelude::*, BufReader},
	path::Path,
};

use anyhow::{Context, Result};

#[derive(Debug)]
struct Template(HashMap<Vec<char>, usize>);

#[derive(Debug)]
struct Rules(HashMap<Vec<char>, Vec<Vec<char>>>);

fn main() -> Result<()> {
	let (mut template, rules) = get_input("input.txt")?;

	const STEPS: usize = 40;

	(1..=STEPS).for_each(|_| polymerize(&mut template, &rules));

	let mut counts = HashMap::new();
	template.0.into_iter().for_each(|(vec, size)| {
		vec.into_iter().for_each(|c| {
			counts
				.entry(c)
				.and_modify(|ref mut e| **e += size)
				.or_insert(size);
		});
	});

	let mut sorted_counts: Vec<_> = counts.values().map(|&s| (s + 1) / 2).collect();
	sorted_counts.sort_unstable();

	println!(
		"answer: {}",
		sorted_counts.iter().last().unwrap() - sorted_counts.get(0).unwrap()
	);

	Ok(())
}

fn polymerize(template: &mut Template, rules: &Rules) {
	let mut temp = Template(HashMap::new());

	template.0.drain().for_each(|(k, v)| {
		rules.0.get(&k).unwrap().iter().for_each(|el| {
			temp.0
				.entry(el.to_vec())
				.and_modify(|e| *e += v)
				.or_insert(v);
		});
	});

	*template = temp;
}

fn get_input(filename: impl AsRef<Path>) -> Result<(Template, Rules)> {
	let file = File::open(filename).with_context(|| "Can't open file")?;
	let mut lines = BufReader::new(file).lines().map(Result::unwrap);

	let mut template = Template(HashMap::new());
	lines
		.next()
		.unwrap()
		.chars()
		.collect::<Vec<char>>()
		.windows(2)
		.for_each(|w| {
			template
				.0
				.entry(w.to_vec())
				.and_modify(|e| *e += 1)
				.or_insert(1);
		});

	let mut rules = Rules(HashMap::new());
	lines.skip(1).for_each(|l| {
		let mut l = l.split(" -> ");
		let initial_pair = l.next().unwrap().chars().collect::<Vec<char>>();

		let mut generated_pairs = Vec::new();
		let mut temp = initial_pair.clone();
		temp.insert(1, l.next().unwrap().chars().next().unwrap());
		temp.windows(2)
			.for_each(|p| generated_pairs.push(p.to_vec()));

		rules.0.insert(initial_pair, generated_pairs);
	});

	Ok((template, rules))
}

#[cfg(test)]
mod tests {
	use test::Bencher;

	use super::main;

	#[bench]
	fn bench_main(b: &mut Bencher) {
		b.iter(|| main());
	}
}
