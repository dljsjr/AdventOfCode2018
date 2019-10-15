use std::fs;
use std::num::ParseIntError;

type Result<T> = std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let contents = fs::read_to_string("day1_input.txt")?;
    match process_input(contents) {
        Ok(changes) => {
            let final_freq: i32 = changes.iter().sum();
            println!("Final frequency: {}", final_freq)
        },
        Err(e) => {
            eprintln!("Error processing input: {:?}", e);
            std::process::exit(1)
        },
    }

    Ok(())
}

fn process_input(contents: String) -> std::result::Result<Vec<i32>, ParseIntError> {

    contents.lines().map(|line| line.parse::<i32>()).collect()
}