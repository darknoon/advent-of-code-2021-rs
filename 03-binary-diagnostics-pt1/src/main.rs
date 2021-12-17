use std::io::{self, BufRead};

fn parse_bin(s: &str) -> Vec<bool> {
    let bin_digits: Vec<bool> = s
        .chars()
        .filter_map(|c| match c {
            '0' => Some(false),
            '1' => Some(true),
            _ => None,
        })
        .collect();
    return bin_digits;
}

// Converts a variable-length binary number to a number
fn to_number(b: &[bool]) -> i32 {
    let mut n = 0;
    for &digit in b {
        n <<= 1;
        n += i32::from(digit)
    }
    return n;
}

fn main() {
    println!("Reading input");

    let stdin = io::stdin();
    let mut rows = stdin
        .lock()
        .lines()
        .filter_map(|line| line.ok()) // need to filter out errors
        .map(|line| parse_bin(line.as_str()))
        .peekable();

    // Length of output number
    let digit_count = rows.peek().unwrap().len();

    // zero-fill result
    let mut true_count: Vec<u32> = vec![0; digit_count];

    let mut row_count = 0;
    for row in rows {
        row_count += 1;
        for (i, &digit) in row.iter().enumerate() {
            if digit {
                true_count[i] += 1;
            }
        }
    }

    // TODO: the .collect() seems unnecessary, maybe to_number could be generic and take an iterator?
    let gamma_digits: Vec<bool> = true_count
        .iter()
        .map(|&count| count > row_count / 2)
        .collect();
    let epsilon_digits: Vec<bool> = true_count
        .iter()
        .map(|&count| count < row_count / 2)
        .collect();

    let gamma = to_number(gamma_digits.as_slice());
    let epsilon = to_number(epsilon_digits.as_slice());

    println!(
        "gamma: {} epsilon: {} product: {}",
        gamma,
        epsilon,
        gamma * epsilon
    );
}
