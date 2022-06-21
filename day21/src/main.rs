use std::{
	self,
	fs::File,
	io::{prelude::*, BufReader},
	ops::RangeInclusive,
	path::Path,
};

use anyhow::{Context, Result};

const MAX_SCORE: u8 = 21;
const DICE_RANGE: RangeInclusive<u8> = 3..=9;
const PROBA: [u8; 7] = [1, 3, 6, 7, 6, 3, 1]; // three three-sided dice means 27 outcomes

#[derive(Clone, Debug)]
struct Player {
	pos: u8,
	score: u8,
}

fn main() -> Result<()> {
	let players = get_input("input.txt")?;

	let (won_by_p2, played) = play_all(players, 0);

	println!("answer: {}", won_by_p2.max(played - won_by_p2));

	Ok(())
}

fn play_all(players: [Player; 2], turn: usize) -> (u64, u64) {
	DICE_RANGE.fold((0, 0), |acc, roll| {
		let (won_by_p2, played) = play_turn(players.clone(), turn, roll);
		let factor = PROBA[roll as usize - 3] as u64;
		(acc.0 + won_by_p2 * factor, acc.1 + played * factor)
	})
}

fn play_turn(mut players: [Player; 2], turn: usize, roll: u8) -> (u64, u64) {
	players[turn].pos = (players[turn].pos + roll) % 10;
	players[turn].score += players[turn].pos + 1; // because positions are shifted by 1

	if players[turn].score >= MAX_SCORE {
		(turn as u64, 1) // using 'turn' to count p2 wins means less computations
	} else {
		play_all(players, 0usize.pow(turn as u32))
	}
}

fn get_input(filename: impl AsRef<Path>) -> Result<[Player; 2]> {
	let file = File::open(filename).with_context(|| "Can't open file")?;
	let mut lines = BufReader::new(file)
		.lines()
		.map(|l| l.unwrap().chars().last().unwrap().to_digit(10).unwrap());

	Ok([
		Player {
			pos: lines.next().unwrap() as u8 - 1, // shift positions from 1-10 to 0-9 for ease
			score: 0,
		},
		Player {
			pos: lines.next().unwrap() as u8 - 1,
			score: 0,
		},
	])
}
