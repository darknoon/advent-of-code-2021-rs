use std::io::{self, BufRead, Result};

fn read_input() -> Result<Vec<i32>> {
    let mut measurements = Vec::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        if let Ok(line) = line {
            if let Ok(int) = line.parse::<i32>() {
                measurements.push(int);
            }
        }
    }

    return Ok(measurements);
}

fn main() {
    println!("Reading input");
    let depths = read_input().unwrap();
    println!("Read input");

    let c = depths.len();
    let d0: &[i32] = &depths[0..c - 1];
    let d1: &[i32] = &depths[1..c];
    let mut increase_count = 0;
    for i in 0..c - 1 {
        if d0[i] < d1[i] {
            increase_count += 1;
        }
        println!("{}", d0[i] - d1[i]);
    }
    println!("increase_count {}", increase_count);
}
