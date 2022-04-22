mod binaryrep;

use anyhow::{Context, Result};
use std::{
    self,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn main() -> Result<()> {
    let input = get_input("input.txt")?;
    let input = input.iter();

    let binary_rep = "0".repeat(12).parse::<binaryrep::BinaryRep>()?;

    let ones = input.fold(binary_rep, |b_r, curr| {
        b_r + curr.parse::<binaryrep::BinaryRep>().unwrap()
    });

    let gamma_rate_string = ones
        .0
        .into_iter()
        .map(|n| if n > 500 { '1' } else { '0' })
        .collect::<String>();

    let gamma_rate = u32::from_str_radix(&gamma_rate_string, 2)?;
    let epsilon_rate = !gamma_rate & 0b111111111111;

    println!("answer: {}", gamma_rate * epsilon_rate);
    Ok(())
}

fn get_input(filename: impl AsRef<Path>) -> Result<Vec<String>> {
    let file = File::open(filename).with_context(|| "Can't open file")?;
    BufReader::new(file)
        .lines()
        .map(|l| l.with_context(|| "Can't parse line"))
        .collect()
}
