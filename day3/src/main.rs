extern crate regex;

use regex::Regex;
use std::fs;
use std::str::FromStr;

#[derive(Debug)]
struct Claim {
    id: usize,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

impl FromStr for Claim {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let pattern = Regex::new(
            r"#(?P<id>[0-9]+)\s@\s(?P<x>[0-9]+),(?P<y>[0-9]+):\s(?P<width>[0-9]+)x(?P<height>[0-9]+)",
        )?;

        if let Some(captures) = pattern.captures(s) {
            Ok(Claim {
                id: captures["id"].parse()?,
                x: captures["x"].parse()?,
                y: captures["y"].parse()?,
                width: captures["width"].parse()?,
                height: captures["height"].parse()?,
            })
        } else {
            Err(From::from(format!("Could not parse claim {}", s)))
        }
    }
}

type Result<T> = std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let contents = fs::read_to_string("inputs/day3.txt")?;

    if let Some(first_entry) = contents.lines().next() {
        match Claim::from_str(first_entry) {
            Ok(claim) => {
                println!("Claim: {:#?}", claim);
                Ok(())
            }
            Err(e) => {
                eprintln!("Error: {:?}", e);
                std::process::exit(1);
            }
        }
    } else {
        Ok(())
    }
}
