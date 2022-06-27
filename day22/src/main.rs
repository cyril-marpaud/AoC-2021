use std::{self, collections::HashSet, fs::File, io::prelude::*, ops::RangeInclusive, path::Path};

use anyhow::{Context, Result};
use regex::Regex;

const MAX_COORD: i32 = 50;
const CUBE_RANGE: RangeInclusive<i32> = -MAX_COORD..=MAX_COORD;

#[derive(Debug)]
struct Rules {
	list: Vec<Rule>,
}

#[derive(Clone, Debug)]
struct Rule {
	is_on: bool,
	x_start: i32,
	x_end: i32,
	y_start: i32,
	y_end: i32,
	z_start: i32,
	z_end: i32,
}

fn main() -> Result<()> {
	let rules = get_input("input.txt")?;

	let mut lit_cubes = HashSet::new();

	rules.list.iter().enumerate().for_each(|(_, r)| {
		(r.x_start..=r.x_end).for_each(|x| {
			if CUBE_RANGE.contains(&x) {
				(r.y_start..=r.y_end).for_each(|y| {
					if CUBE_RANGE.contains(&y) {
						(r.z_start..=r.z_end).for_each(|z| {
							if CUBE_RANGE.contains(&z) {
								match r.is_on {
									true => lit_cubes.insert((x, y, z)),
									false => lit_cubes.remove(&(x, y, z)),
								};
							}
						});
					}
				});
			}
		});
	});

	println!("answer: {}", lit_cubes.len());

	Ok(())
}

fn get_input(filename: impl AsRef<Path>) -> Result<Rules> {
	let mut file = File::open(filename).with_context(|| "Can't open file")?;
	let mut buffer = String::new();
	file.read_to_string(&mut buffer)?;

	let mut rules = Vec::new();
	let mut max_coord = 0;

	let re = Regex::new(r"(on|off) x=(.*)\.\.(.*),y=(.*)\.\.(.*),z=(.*)\.\.(.*)").unwrap();
	for cap in re.captures_iter(&buffer) {
		let rule = Rule {
			is_on: &cap[1] == "on",
			x_start: cap[2].parse().unwrap(),
			x_end: cap[3].parse().unwrap(),
			y_start: cap[4].parse().unwrap(),
			y_end: cap[5].parse().unwrap(),
			z_start: cap[6].parse().unwrap(),
			z_end: cap[7].parse().unwrap(),
		};

		max_coord = max_coord.max(rule.x_start.abs());
		max_coord = max_coord.max(rule.x_end.abs());
		max_coord = max_coord.max(rule.y_start.abs());
		max_coord = max_coord.max(rule.y_end.abs());
		max_coord = max_coord.max(rule.z_start.abs());
		max_coord = max_coord.max(rule.z_end.abs());

		rules.push(rule);
	}

	Ok(Rules { list: rules })
}
