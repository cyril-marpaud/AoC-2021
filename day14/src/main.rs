#![feature(linked_list_cursors)]
#![feature(test)]
extern crate test;

use std::{
	self,
	collections::{BTreeSet, HashMap, HashSet, LinkedList},
	fs::File,
	io::{prelude::*, BufReader},
	path::Path,
};

use anyhow::{Context, Result};

fn main() -> Result<()> {
	let (mut template, mut insertion_rules, chars) = get_input("input.txt")?;

	const STEP_NUMBER: usize = 10;
	let mut sorted_counts = BTreeSet::<usize>::new();

	(1..=STEP_NUMBER).for_each(|s| polymerize(&mut template, &mut insertion_rules));

	for c in chars {
		sorted_counts.insert(template.iter().filter(|el| **el == c).count());
	}

	println!(
		"answer: {}",
		sorted_counts.iter().last().unwrap() - sorted_counts.iter().next().unwrap()
	);

	Ok(())
}

fn polymerize(template: &mut LinkedList<char>, insertion_rules: &mut HashMap<Vec<char>, char>) {
	let mut cursor = template.cursor_front_mut();

	while let Some(n) = cursor.as_cursor().peek_next() {
		if let Some(c) = insertion_rules.get(&vec![*cursor.as_cursor().current().unwrap(), *n]) {
			cursor.insert_after(*c);
			cursor.move_next();
		}
		cursor.move_next();
	}
}

fn get_input(
	filename: impl AsRef<Path>,
) -> Result<(LinkedList<char>, HashMap<Vec<char>, char>, HashSet<char>)> {
	let file = File::open(filename).with_context(|| "Can't open file")?;
	let mut lines = BufReader::new(file).lines().map(Result::unwrap);

	let template = lines.next().unwrap().chars().collect::<LinkedList<char>>();

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

#[cfg(test)]
mod tests {
	use test::Bencher;

	use super::main;

	#[bench]
	fn bench_main(b: &mut Bencher) {
		b.iter(|| main());
	}
}
