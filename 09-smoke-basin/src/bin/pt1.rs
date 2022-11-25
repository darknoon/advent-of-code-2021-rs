use std::cmp::min;
use std::io::{self, BufRead};

// Read a 2d grid of numbers
fn read_input() -> io::Result<Vec<Vec<u8>>> {
    let stdin = io::stdin();

    stdin
        .lock()
        .lines()
        .map(|l| {
            // If read line ok
            l.map(|l| {
                // Transform each char into a number
                l.chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect::<Vec<u8>>()
            })
        })
        .collect()
}

fn make_low_point_iter(input: &Vec<Vec<u8>>) -> impl Iterator<Item = (usize, usize, u8)> + '_ {
    // For each point in the grid, check if it's the lowest among its neighbours
    let rows = input.len();
    let cols = input[0].len();
    (0..rows)
        .flat_map(move |r| (0..cols).map(move |c| (r, c, input[r][c])))
        .filter(move |(r, c, v)| {
            // Check if it's the lowest among its up/down neighbours
            for y in r.saturating_sub(1)..=min(r + 1, rows - 1) {
                if y != *r && input[y][*c] <= *v {
                    return false;
                }
            }
            // Check if it's the lowest among its left/right neighbours
            for x in c.saturating_sub(1)..=min(c + 1, cols - 1) {
                if x != *c && input[*r][x] <= *v {
                    return false;
                }
            }
            true
        })
}

fn check_input(input: &Vec<Vec<u8>>) -> bool {
    // Make sure input has same number of columns in each row
    let cols = input[0].len();
    input.iter().all(|row| row.len() == cols)
}

fn main() {
    let input = read_input().unwrap();
    if !check_input(&input) {
        panic!("Input is not a valid grid");
    }
    // Calculate risk level
    let low_points = make_low_point_iter(&input);
    println!(
        "Low points: {:?}",
        make_low_point_iter(&input)
            .filter(|(r, c, _)| *c == 99)
            .collect::<Vec<_>>()
    );
    // The risk level of a low point is 1 plus its height
    // Return the sum of total risk level
    let risk: u32 = low_points.map(|(_, _, v)| v as u32 + 1).sum();
    println!("{}", risk);
}
