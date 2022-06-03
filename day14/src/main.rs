use std::{
	self,
	collections::{BTreeSet, HashMap, HashSet},
	fs::File,
	io::{prelude::*, BufReader},
	path::Path,
};

use anyhow::{Context, Result};

fn main() -> Result<()> {
	let (mut template, mut insertion_rules, chars) = get_input("input.txt")?;

	const STEP_NUMBER: usize = 10;
	let mut sorted_counts = BTreeSet::<usize>::new();

	(1..=STEP_NUMBER).for_each(|_| polymerize(&mut template, &mut insertion_rules));

	for c in chars {
		sorted_counts.insert(template.iter().filter(|el| **el == c).count());
	}

	println!(
		"answer: {}",
		sorted_counts.iter().last().unwrap() - sorted_counts.iter().next().unwrap()
	);

	Ok(())
}

fn polymerize(template: &mut Vec<char>, insertion_rules: &mut HashMap<Vec<char>, char>) {
	template.clone().windows(2).enumerate().rev().for_each(|w| {
		if let Some(c) = insertion_rules.get(w.1) {
			template.insert(w.0 + 1, *c);
		}
	});
}

fn get_input(
	filename: impl AsRef<Path>,
) -> Result<(Vec<char>, HashMap<Vec<char>, char>, HashSet<char>)> {
	let file = File::open(filename).with_context(|| "Can't open file")?;
	let mut lines = BufReader::new(file).lines().map(Result::unwrap);

	let template = lines.next().unwrap().chars().collect::<Vec<char>>();

	let mut chars = HashSet::<char>::new();
	chars.extend(template.iter());

	let mut insertion_rules = HashMap::new();
	lines.skip(1).for_each(|l| {
		let mut l = l.split(" -> ");
		let (sequence, polymer) = (
			l.next().unwrap().chars().collect(),
			l.last().unwrap().chars().next().unwrap(),
		);
		insertion_rules.insert(sequence, polymer);
		chars.insert(polymer);
	});

	Ok((template, insertion_rules, chars))
}
