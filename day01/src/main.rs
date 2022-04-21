use anyhow::{Context, Result};
use std::{
    self,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn main() -> Result<()> {
    let input = get_input("input.txt")?;
    let mut input = input.iter();

    let (first, second, third) = (
        input.next().unwrap(),
        input.next().unwrap(),
        input.next().unwrap(),
    );

    let count = input
        .fold(
            (0, (first, second, third)),
            |(count, (f, s, t)), curr| match (f + s + t) < (s + t + curr) {
                true => (count + 1, (s, t, curr)),
                false => (count, (s, t, curr)),
            },
        )
        .0;

    println!("answer: {}", count);

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
