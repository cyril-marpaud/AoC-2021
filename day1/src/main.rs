use anyhow::{Context, Result};
use std::{
    self,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn main() -> Result<()> {
    let input = get_input("input.txt")?;
    let first = input.iter().next().unwrap();

    let count = input
        .iter()
        .fold((0, first), |(count, prec), curr| match prec < curr {
            true => (count + 1, curr),
            false => (count, curr),
        });

    println!("answer: {}", count.0);

    Ok(())
}

fn get_input(filename: impl AsRef<Path>) -> Result<Vec<u32>> {
    let file = File::open(filename).with_context(|| "Can't open file")?;
    BufReader::new(file)
        .lines()
        .map(|l| {
            l?.parse::<u32>()
                .with_context(|| "Can't parse line into integer")
        })
        .collect()
}
