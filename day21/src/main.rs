use std::{
	self,
	fs::File,
	io::{prelude::*, BufReader},
	path::Path,
};

use anyhow::{Context, Result};

const MAX_SCORE: u32 = 1000;

pub struct Die {
	iter: Box<dyn Iterator<Item = u32>>,
	rolls: u32,
}

impl Die {
	fn new() -> Self {
		Die {
			iter: Box::new(1..),
			rolls: 0,
		}
	}
}

impl Iterator for Die {
	type Item = u32;

	fn next(&mut self) -> Option<Self::Item> {
		self.rolls += 1;
		Some(self.iter.next().unwrap() % 100)
	}
}

#[derive(Clone, Debug)]
struct Player {
	pos: u32,
	score: u32,
}

impl Player {
	fn new(pos: u32) -> Self {
		Self {
			pos: pos - 1,
			score: 0,
		}
	}
}

fn main() -> Result<()> {
	let (pos1, pos2) = get_input("input.txt")?;
	let mut players = [Player::new(pos1), Player::new(pos2)];
	let mut die = Die::new();

	let mut player_iter = (0..=1).cycle();

	loop {
		if play(&mut players[player_iter.next().unwrap()], &mut die) {
			break;
		}
	}

	println!(
		"answer: {}",
		players[player_iter.next().unwrap()].score * die.rolls
	);

	Ok(())
}

fn play(player: &mut Player, die: &mut Die) -> bool {
	player.pos =
		(player.pos + die.next().unwrap() + die.next().unwrap() + die.next().unwrap()) % 10;
	player.score += player.pos as u32 + 1;

	player.score >= MAX_SCORE // return true if win
}

fn get_input(filename: impl AsRef<Path>) -> Result<(u32, u32)> {
	let file = File::open(filename).with_context(|| "Can't open file")?;
	let mut lines = BufReader::new(file)
		.lines()
		.map(|l| l.unwrap().chars().last().unwrap().to_digit(10).unwrap());

	Ok((lines.next().unwrap(), lines.next().unwrap()))
}
