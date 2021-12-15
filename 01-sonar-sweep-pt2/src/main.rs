use std::{
    io::{self, BufRead},
    str::FromStr,
};

fn read_parsed_lines<T: FromStr>() -> Vec<T> {
    io::stdin()
        .lock()
        .lines()
        .filter_map(|line| line.ok()) // need to filter out errors
        .filter_map(|s| s.parse::<T>().ok()) // need to filter out non-numbers
        .collect()
}

fn main() {
    println!("Reading input");
    let depths: Vec<i32> = read_parsed_lines();
    println!("Read input");

    let c = depths.len();
    // Offset views
    let d0: &[i32] = &depths[0..c - 2];
    let d1: &[i32] = &depths[1..c - 1];
    let d2: &[i32] = &depths[2..c - 0];

    let filtered: Vec<i32> = d0
        .iter()
        .zip(d1.iter())
        .zip(d2.iter())
        .map(|((a, b), c)| a + b + c)
        .collect();

    let fc = filtered.len();

    // println!("ff {:?}", filtered);

    // Offset filtered
    let f0 = &filtered[0..fc - 1];
    let f1 = &filtered[1..fc - 0];

    let increases = f0
        .iter()
        .zip(f1.iter())
        .map(|(a, b)| (b - a) > 0)
        .collect::<Vec<bool>>();
    // println!("ffi {:?}", increases);

    let increase_count = increases.iter().filter(|&&a| a).count();

    println!("increase_count {}", increase_count);
}
