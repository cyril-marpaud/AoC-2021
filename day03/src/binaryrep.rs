use anyhow::{anyhow, Result};
use std::{ops::Add, str::FromStr};

pub struct BinaryRep(pub [u32; 12]);

impl Add for BinaryRep {
    type Output = Self;
    fn add(self, o: Self) -> Self {
        let mut my_arr = [0u32; 12];
        for i in 0..12 {
            my_arr[i] = self.0[i] + o.0[i];
        }
        BinaryRep(my_arr)
    }
}

impl FromStr for BinaryRep {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        if s.len() != 12 {
            return Err(anyhow!("Wrong string length"));
        }

        if let Err(_) = u32::from_str_radix(s, 2) {
            return Err(anyhow!("Not a binary repr"));
        }

        let mut my_array = [0u32; 12];
        let mut s = s.chars().enumerate();
        while let Some(c) = s.next() {
            my_array[c.0] = c.1.to_digit(10).unwrap();
        }

        Ok(BinaryRep(my_array))
    }
}
