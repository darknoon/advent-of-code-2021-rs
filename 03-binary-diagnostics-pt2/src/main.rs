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

fn digit_stats(i: usize, rows: &Vec<Vec<bool>>) -> usize {
    let mut count = 0;
    for number in rows {
        count += number[i] as usize;
    }
    return count;
}

// Find the best match for the digits of number according to criteria
fn find_match(numbers: Vec<Vec<bool>>, most_likely: bool) -> Option<Vec<bool>> {
    use std::cmp::Ordering::*;
    let digit_len = numbers[0].len();
    let mut filtered = numbers;
    for i in 0..digit_len {
        // Remove all numbers that don't match the criteria
        let count = digit_stats(i, &filtered);
        // Is it over half true?
        let remaining_count = filtered.len();
        if most_likely {
            filtered.retain(|num| -> bool {
                match (count * 2).partial_cmp(&remaining_count).unwrap() {
                    Equal => num[i] == true,
                    Greater => num[i],
                    Less => !num[i],
                }
            });
        } else {
            filtered.retain(
                |num| match (count * 2).partial_cmp(&remaining_count).unwrap() {
                    Equal => num[i] == false,
                    Greater => !num[i],
                    Less => num[i],
                },
            );
        }

        println!("digit {}, {:?} remain", i, filtered.len());
        if filtered.len() == 1 {
            return Some(filtered[0].clone());
        }
    }
    return None;
}

fn main() {
    println!("Reading input");

    let stdin = io::stdin();
    let rows: Vec<Vec<bool>> = stdin
        .lock()
        .lines()
        .filter_map(|line| line.ok()) // need to filter out errors
        .map(|line| parse_bin(line.as_str()))
        .collect();

    println!("Calculating oxy");
    let oxy_digits = find_match(rows.clone(), true).unwrap();
    println!("Calculating co2");
    let co2_digits = find_match(rows.clone(), false).unwrap();

    let oxy = to_number(&oxy_digits);
    let co2 = to_number(&co2_digits);

    println!("oxy: {} co2: {} product: {}", oxy, co2, oxy * co2);
}
