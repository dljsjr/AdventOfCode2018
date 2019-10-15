use std::fs;

type Result<T> = std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let contents = fs::read_to_string("day2_input.txt")?;

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
