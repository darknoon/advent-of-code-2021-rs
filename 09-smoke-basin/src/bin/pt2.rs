use std::cmp::min;
use std::fmt::Debug;
use std::io::{self, BufRead};

type Heightmap = Vec<Vec<u8>>;

// Read a 2d grid of numbers
fn read_input() -> io::Result<Heightmap> {
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

fn make_low_point_iter(input: &Heightmap) -> impl Iterator<Item = (usize, usize, u8)> + '_ {
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

// Basins, number of points unassigned
struct BasinAssign {
    // 2d array of assigned basin indices
    basins: Vec<Vec<Option<usize>>>,
    // Number of points assigned to the given basin
    basin_counts: Vec<usize>,
    // Number of points unassigned
    unassigned: usize,
}
// Implement format for BasinAssign
impl std::fmt::Display for BasinAssign {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in &self.basins {
            for basin in row {
                match basin {
                    Some(b) => write!(f, "{:1}", b)?,
                    None => write!(f, " ")?,
                }
            }
            writeln!(f)?;
        }
        for (i, count) in self.basin_counts.iter().enumerate() {
            writeln!(f, "Basin {}: {}", i, count)?;
        }
        Ok(())
    }
}

fn init_basins_assign(input: &Heightmap) -> BasinAssign {
    let rows = input.len();
    let cols = input[0].len();
    let mut basins = vec![vec![None; cols]; rows];
    let mut unassigned = rows * cols;
    let mut basin_count = 0;
    for (i, (r, c, _)) in make_low_point_iter(input).enumerate() {
        basins[r][c] = Some(i);
        unassigned -= 1;
        basin_count += 1;
    }
    let basin_counts = (0..basin_count).map(|_| 1).collect();
    // Items with value 9 are never assigned
    for r in 0..rows {
        for c in 0..cols {
            if input[r][c] == 9 {
                unassigned -= 1;
            }
        }
    }
    BasinAssign {
        basins,
        basin_counts,
        unassigned,
    }
}

fn assign_points_to_basins(b: &BasinAssign, input: &Heightmap) -> BasinAssign {
    let mut new_basin_assign = b.basins.clone();
    let mut new_unassigned = b.unassigned;
    let mut new_basin_counts = b.basin_counts.clone();
    let rows = b.basins.len();
    let cols = b.basins[0].len();
    for (r, row) in new_basin_assign.iter_mut().enumerate() {
        'point: for (c, pt) in row.iter_mut().enumerate() {
            // Current point is (r,c) height is input[r][c], assigned basin is pt
            if pt.is_none() && input[r][c] != 9 {
                // Check neighbours up/down
                // If there is a neighbour with a basin that is lower than us, assign to that basin
                for y in r.saturating_sub(1)..=min(r + 1, rows - 1) {
                    if let Some(bi) = b.basins[y][c] {
                        if y != r && input[y][c] < input[r][c] {
                            *pt = Some(bi);
                            new_basin_counts[bi] += 1;
                            new_unassigned -= 1;
                            break 'point;
                        }
                    }
                }
                for x in c.saturating_sub(1)..=min(c + 1, cols - 1) {
                    if let Some(bi) = b.basins[r][x] {
                        if x != c && input[r][x] < input[r][c] {
                            *pt = Some(bi);
                            new_basin_counts[bi] += 1;
                            new_unassigned -= 1;
                            break 'point;
                        }
                    }
                }
            }
        }
    }
    BasinAssign {
        basins: new_basin_assign,
        basin_counts: new_basin_counts,
        unassigned: new_unassigned,
    }
}

fn check_input(input: &Heightmap) -> bool {
    // Make sure input has same number of columns in each row
    let cols = input[0].len();
    input.iter().all(|row| row.len() == cols)
}

fn main() {
    let input = read_input().unwrap();
    if !check_input(&input) {
        panic!("Input is not a valid grid");
    }
    // Initialize basin assignments
    let mut b = init_basins_assign(&input);
    // Assign points to basins
    while b.unassigned > 0 {
        println!("Unassigned: {}", b.unassigned);
        println!("{}", b);
        b = assign_points_to_basins(&b, &input);
    }

    for (r, row) in input.iter().enumerate() {
        for (c, height) in row.iter().enumerate() {
            print!(
                "{:1}",
                if b.basins[r][c].is_none() {
                    ' ' as char
                } else {
                    (input[r][c] + 48) as char
                }
            );
        }
        println!();
    }
    // Print top 3 basins
    let mut basin_counts = b.basin_counts.clone();
    basin_counts.sort();
    basin_counts.reverse();
    println!("Top 3 basins:");
    for i in 0..3 {
        println!("Basin {}: {}", i, basin_counts[i]);
    }
    println!(
        "Product: {}",
        basin_counts[0] * basin_counts[1] * basin_counts[2]
    );
}
