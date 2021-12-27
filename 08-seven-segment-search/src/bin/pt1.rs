use std::io::{self, BufRead};

fn digit_lut(segment_count: u8) -> Option<u8> {
    match segment_count {
        2 => Some(1),
        3 => Some(7),
        4 => Some(4),
        7 => Some(8),
        _ => None,
    }
}

fn read_digits(second_part: &str) -> Vec<u8> {
    second_part
        .split_whitespace()
        .map(|run| run.len() as u8)
        .collect()
}

fn read_input() -> io::Result<Vec<Vec<u8>>> {
    let stdin = io::stdin();

    // Just read the length of each digit list
    stdin
        .lock()
        .lines()
        .map(|l| {
            // If read line ok
            l.map(|l| {
                let mut sides = l.split('|');
                let (_first_part, second_part) = (sides.next().unwrap(), sides.next().unwrap());
                // Split each line into digits, then just return the segment count in each one
                read_digits(second_part)
            })
        })
        .collect()
}

fn main() -> io::Result<()> {
    // read lines
    let segment_counts = read_input()?;
    let count_1_4_7_8 = segment_counts.iter().flatten().fold(0, |count, &segment| {
        count
            + match digit_lut(segment) {
                Some(_) => 1,
                None => 0,
            }
    });
    println!("Count of 1 4 7 8: {}", count_1_4_7_8);
    Ok(())
}
