use std::io;

// Just an array of the count of fish of each age [0 ... 8]  inclusive
struct FishState([u64; 9]);

impl FishState {
    fn next(&mut self) {
        let bebes = self.0[0];
        // Decrement fish of each age
        for i in 1..self.0.len() {
            self.0[i - 1] = self.0[i];
        }
        // Spawn bebes
        self.0[8] = bebes;
        // Previous ready to spawn now adults
        self.0[6] += bebes;
    }

    fn count(&self) -> u64 {
        self.0.iter().sum()
    }
}

fn read_input() -> io::Result<FishState> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let fish = input
        .trim()
        .split(',')
        .map(|x| {
            x.parse()
                .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Couldn't parse fish"))
        })
        .collect::<io::Result<Vec<usize>>>()?;

    let fish_state = fish.iter().fold(FishState([0; 9]), |mut state, &fish| {
        state.0[fish] += 1;
        state
    });
    Ok(fish_state)
}

fn main() -> io::Result<()> {
    let mut fish_state = read_input()?;
    println!("Fish initial state: {:?}", &fish_state.0);
    for day in 0..256 {
        // Only need to run update for old fishes, not new
        fish_state.next();
        println!(
            "Day {} Count: {} State: {:?}",
            day + 1,
            fish_state.count(),
            &fish_state.0
        );
    }
    return Ok(());
}
