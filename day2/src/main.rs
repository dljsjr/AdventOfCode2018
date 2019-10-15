use std::fs;

type Result<T> = std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    process_input_part_1("inputs/day2.txt")?;
    process_input_part_2("inputs/day2.txt")?;

    Ok(())
}

fn process_input_part_1(filename: &str) -> Result<()> {
    let contents = fs::read_to_string(filename)?;
    let mut assoc_array = [0usize; 256];
    let (mut twos, mut threes) = (0usize, 0usize);
    for line in contents.lines() {
        for byte in assoc_array.iter_mut() {
            *byte = 0;
        }

        for byte in line.as_bytes() {
            let idx = *byte as usize;
            assoc_array[idx] = assoc_array[idx] + 1;
        }

        if assoc_array.iter().any(|f| *f == 2) {
            twos += 1;
        }

        if assoc_array.iter().any(|f| *f == 3) {
            threes += 1;
        }
    }
    println!("Checksum: {}", twos * threes);
    Ok(())
}

fn process_input_part_2(filename: &str) -> Result<()> {
    let contents = fs::read_to_string(filename)?;
    let contents: Vec<&str> = contents.lines().collect();

    for i in 0..contents.len() {
        for j in i + 1..contents.len() {
            let s1 = contents[i];
            let s2 = contents[j];

            if let Some(common_letters) = compare_ids(s1, s2) {
                println!("Found common letters in IDs: {}", common_letters);
                return Ok(());
            }
        }
    }

    Err(From::from(
        "Could not find two IDs with one letter difference",
    ))
}

fn compare_ids(id1: &str, id2: &str) -> Option<String> {
    if id1.len() != id2.len() {
        return None;
    }

    let mut diffs = 0;
    for (c1, c2) in id1.chars().zip(id2.chars()) {
        if c1 != c2 {
            diffs += 1;
        }
    }

    if diffs != 1 {
        return None;
    }

    Some(
        id1.chars()
            .zip(id2.chars())
            .filter(|(c1, c2)| c1 == c2)
            .map(|(c1, _c2)| c1)
            .collect(),
    )
}
