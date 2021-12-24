use std::io;

struct Fish(u32);

impl Fish {
    // Update state and optionally create a new fish
    fn next(&self, x: u32) -> (Self, Option<Self>) {
        let BABY_FISH = Fish(8);
        let ADULT_FISH = Fish(6);
        match self {
            Fish(0) => (ADULT_FISH, Some(BABY_FISH)),
            Fish(i) => (Fish(i - 1), None),
        }
    }
}

fn read_input() -> io::Result<Vec<Fish>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
        .trim()
        .split(',')
        .map(|x| {
            x.parse()
                .map(|x| Fish(x))
                .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Couldn't parse fish"))
        })
        .collect()
}

fn main() -> io::Result<()> {
    let mut fish = read_input()?;
    for day in 0..80 {
        // Only need to run update for old fishes, not new
        for i in 0..fish.len() {
            let (fish_updated, new_fish) = fish[i].next(day);
            if let Some(new_fish) = new_fish {
                fish.push(new_fish);
            }
            fish[i] = fish_updated;
        }
        println!("Fish count after {}: {}", day + 1, fish.len());
    }
    return Ok(());
}
