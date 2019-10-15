use std::collections::HashSet;
use std::fs;
use std::num::ParseIntError;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
type ParseResult = std::result::Result<Vec<i32>, ParseIntError>;

fn main() -> Result<()> {
    process_input_part_1("inputs/day1.txt")?;
    process_input_part_2("inputs/day1.txt")?;

    Ok(())
}

fn process_input_part_1(filename: &str) -> Result<()> {
    let contents = fs::read_to_string(filename)?;

    match parse_integers_from_contents(contents) {
        Ok(parsed_ints) => {
            println!("Final frequency: {:?}", parsed_ints.iter().sum::<i32>());
            Ok(())
        }
        Err(e) => Err(From::from(format!("Could not parse input file: {}", e))),
    }
}

fn parse_integers_from_contents(contents: String) -> ParseResult {
    contents
        .lines()
        .map(|line| line.parse::<i32>())
        .collect::<ParseResult>()
}

fn process_input_part_2(filename: &str) -> Result<()> {
    let contents = fs::read_to_string(filename)?;

    match parse_integers_from_contents(contents) {
        Ok(parsed_ints) => {
            let mut current_freq = 0;
            let mut frequencies = HashSet::new();
            frequencies.insert(current_freq);

            loop {
                for freq in parsed_ints.iter() {
                    current_freq += *freq;

                    if frequencies.contains(&current_freq) {
                        println!("First doubled frequency: {:?}", current_freq);
                        return Ok(());
                    } else {
                        frequencies.insert(current_freq);
                    }
                }
            }
        }
        Err(e) => Err(From::from(format!("Could not parse input file: {}", e))),
    }
}
