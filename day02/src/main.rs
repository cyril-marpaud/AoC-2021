use anyhow::{anyhow, Context, Result};
use std::{
    self, fmt,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    str::FromStr,
};

enum Command {
    Forward(u32),
    Up(u32),
    Down(u32),
}

impl FromStr for Command {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        match s.find(' ') {
            Some(0) | None => Err(anyhow!("Missing command")),
            Some(p) => match s[p + 1..].parse::<u32>() {
                Err(e) => Err(anyhow!("Can't parse u8: {}", e)),
                Ok(n) => match &s[..p] {
                    "forward" => Ok(Command::Forward(n)),
                    "up" => Ok(Command::Up(n)),
                    "down" => Ok(Command::Down(n)),
                    _ => Err(anyhow!("Unknown command")),
                },
            },
        }
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (c, n) = match self {
            Command::Forward(n) => ("forward", n),
            Command::Up(n) => ("up", n),
            Command::Down(n) => ("down", n),
        };
        write!(f, "{} {}", c, n)
    }
}

fn main() -> Result<()> {
    let input = get_input("input.txt")?;
    let input = input.iter();

    let (pos, dep) = input.fold((0, 0), |(p, d), curr| match curr {
        Command::Forward(n) => (p + n, d),
        Command::Up(n) => (p, d - n),
        Command::Down(n) => (p, d + n),
    });

    println!("answer: {}", pos * dep);

    Ok(())
}

fn get_input(filename: impl AsRef<Path>) -> Result<Vec<Command>> {
    let file = File::open(filename).with_context(|| "Can't open file")?;
    BufReader::new(file)
        .lines()
        .map(|l| {
            l?.parse::<Command>()
                .with_context(|| "Can't parse line into Command")
        })
        .collect()
}
