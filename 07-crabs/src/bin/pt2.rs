use std::io::{self, BufRead};

fn read_input() -> io::Result<Vec<i32>> {
    let stdin = io::stdin();
    let mut stdin_iter = stdin.lock().lines().into_iter();
    let line = stdin_iter.next().unwrap()?;

    line.split(',')
        .map(|num| num.parse::<i32>())
        .map(|r| r.map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Invalid input")))
        .collect::<io::Result<Vec<_>>>()
}

fn fuel_cost(crab_locations: &[i32], location: i32) -> i32 {
    let mut cost = 0;
    for crab in crab_locations {
        let n = (crab - location).abs();
        cost += (n * (n + 1)) / 2;
    }
    cost
}

// Returns (x, x_cost)
// Could do a fancier algorithm, but this is good enough to solve this size of problem
fn minimize_fuel(crabs: &[i32]) -> (i32, i32) {
    let (min_x, max_x) = crabs.iter().fold((i32::MAX, i32::MIN), |(min, max), crab| {
        (min.min(*crab), max.max(*crab))
    });
    println!("Checking {} to {}", min_x, max_x);
    let mut best = (0, i32::MAX);
    for x in min_x..max_x {
        let cost = fuel_cost(&crabs, x);
        if cost < best.1 {
            best = (x, cost);
        }
        // println!("x={} => {}", x, cost);
    }
    best
}

fn main() -> io::Result<()> {
    let crabs = read_input()?;

    let best = minimize_fuel(&crabs);
    println!("Best: cost {} @ {}", best.1, best.0);

    Ok(())
}
