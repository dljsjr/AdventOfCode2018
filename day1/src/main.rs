use std::fs;
use std::num::ParseIntError;

type Result<T> = std::result::Result<T, Box<dyn ::std::error::Error>>;
type ParseResult = std::result::Result<Vec<i32>, ParseIntError>;

fn main() -> Result<()> {
    process_input("day1_input.txt")
}

fn process_input(filename: &str) -> Result<()> {
    match fs::read_to_string(filename)?
        .lines()
        .map(|line| line.parse::<i32>())
        .collect::<ParseResult>()
    {
        Ok(val) => {
            println!("Final frequency: {:?}", val.iter().sum::<i32>());
            Ok(())
        }
        Err(e) => Err(From::from(format!("Could not parse input file: {}", e))),
    }
}
