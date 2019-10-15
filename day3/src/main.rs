extern crate regex;

use std::collections::HashMap;
use std::fs;
use std::ops::Range;
use std::str::FromStr;

use regex::Regex;

#[derive(Debug)]
struct Claim {
    id: usize,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

impl Claim {
    fn get_fabric_section_iterator(&self) -> FabricSectionIterator {
        FabricSectionIterator {
            x_range: self.x..(self.x + self.width),
            y_range: self.y..(self.y + self.height),
            x: self.x,
            y: self.y,
        }
    }
}

struct FabricSectionIterator {
    x_range: Range<u32>,
    y_range: Range<u32>,
    x: u32,
    y: u32,
}

impl Iterator for FabricSectionIterator {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.x >= self.x_range.end {
            self.y += 1;
            self.x = self.x_range.start;
        }
        if self.y >= self.y_range.end {
            return None;
        }

        let ret: (u32, u32) = (self.x, self.y);
        self.x += 1;
        Some(ret)
    }
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

    let mut contained_points: HashMap<(u32, u32), u32> = HashMap::new();

    if let Ok(claims) = contents
        .lines()
        .map(|line| Claim::from_str(line))
        .collect::<Result<Vec<Claim>>>()
    {
        for claim in claims.into_iter() {
            for section in claim.get_fabric_section_iterator() {
                *contained_points.entry((section.0, section.1)).or_default() += 1;
            }
        }
    } else {
        return Err(From::from("Problem parsing and iterating all claims"));
    }

    let total_overlap_area = contained_points
        .values()
        .filter(|&&occurrences| occurrences > 1)
        .count();

    println!("Total overlap area: {}", total_overlap_area);

    Ok(())
}
