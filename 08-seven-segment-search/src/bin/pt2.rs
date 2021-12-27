use self::Segment::*;
use std::{
    collections::HashSet,
    fmt::Debug,
    io::{self, BufRead},
    str::FromStr,
};
enum SegmentParseError {
    InvalidSegment,
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Segment {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl FromStr for Segment {
    type Err = SegmentParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "a" => Ok(Self::A),
            "b" => Ok(Self::B),
            "c" => Ok(Self::C),
            "d" => Ok(Self::D),
            "e" => Ok(Self::E),
            "f" => Ok(Self::F),
            "g" => Ok(Self::G),
            _ => Err(SegmentParseError::InvalidSegment),
        }
    }
}

// Translate a list of digits into a packed representation
fn segment_to_bits(segments: &Vec<Segment>) -> DigitBits {
    let mut b = DigitBits([false; 7]);
    for s in segments {
        b.0[*s as usize] = true;
    }
    b
}

// Permute the bits
fn permute_bits(bits: &DigitBits, permutation: &Permutation) -> DigitBits {
    let mut permuted_bits = DigitBits::default();
    for i in 0..(bits.0.len()) {
        permuted_bits.0[i] = bits.0[permutation[i] as usize];
    }
    permuted_bits
}

// A particular combination of wires being on/ off
type Digit = Vec<Segment>;

// Representation of said, packed into a single u8
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
struct DigitBits([bool; 7]);

// Must be a set of exactly 7 unique segments, representing the rewiring of the corresponding Segment to that index
// ie a list of 7 indices
type Permutation = Vec<Segment>;
// type Permutation = [Segment; 7];

// https://en.wikipedia.org/wiki/Heap%27s_algorithm
// Ideally, we could use generators to just yield each possibility instead of storing them in dynamic memory
fn recursive_heaps_algorithm<T: Clone + Debug>(items: Vec<T>) -> Vec<Vec<T>> {
    struct Env<T: Clone + Debug> {
        current: Vec<T>,
        generated: Vec<Vec<T>>,
    }
    fn recur<T: Clone + Debug>(e: &mut Env<T>, len: usize) {
        match len {
            1 => e.generated.push(e.current.clone()),
            _ => {
                recur(e, len - 1);
                for i in 0..(len - 1) {
                    match len % 2 {
                        0 => e.current.swap(i, len - 1),
                        1 => e.current.swap(0, len - 1),
                        _ => unreachable!(),
                    }
                    recur(e, len - 1);
                }
            }
        }
    }

    let len = items.len();
    let mut e = Env {
        current: items,
        generated: vec![],
    };
    recur(&mut e, len);

    e.generated
}

// eg ac -> [true, false, true, false, false, …]
fn parse_segments_to_digit(input: &str) -> io::Result<DigitBits> {
    let vec: io::Result<Digit> = input
        .chars()
        .map(|c| {
            c.to_string()
                .parse()
                .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid segment"))
        })
        .collect();
    vec.map(|segments| segment_to_bits(&segments))
}

fn read_input() -> io::Result<Vec<(Vec<DigitBits>, Vec<DigitBits>)>> {
    let stdin = io::stdin();

    // Just read the length of each digit list
    stdin
        .lock()
        .lines()
        .map(|l| {
            // If read line ok
            l.and_then(|l| -> io::Result<(Vec<DigitBits>, Vec<DigitBits>)> {
                let mut sides = l.split('|');
                let (first_part, second_part) = (sides.next().unwrap(), sides.next().unwrap());

                // Split each line into digits, then just return the segment count in each one
                let unique_segments = first_part
                    .split_whitespace()
                    .map(parse_segments_to_digit)
                    .collect::<io::Result<Vec<DigitBits>>>()?;

                let data_segments = second_part
                    .split_whitespace()
                    .map(parse_segments_to_digit)
                    .collect::<io::Result<Vec<DigitBits>>>()?;

                Ok((unique_segments, data_segments))
            })
        })
        .collect()
}

// Should be const but can't figure that out
fn get_true_segments() -> Vec<DigitBits> {
    [
        vec![A, B, C, E, F, G],    // 0
        vec![C, F],                // 1
        vec![A, C, D, E, G],       // 2
        vec![A, C, D, F, G],       // 3
        vec![B, C, D, F],          // 4
        vec![A, B, D, F, G],       // 5
        vec![A, B, D, E, F, G],    // 6
        vec![A, C, F],             // 7
        vec![A, B, C, D, E, F, G], // 8
        vec![A, B, C, D, F, G],    // 9
    ]
    .iter()
    .map(|segments| segment_to_bits(&segments))
    .collect::<Vec<_>>()
}

fn main() -> io::Result<()> {
    let input = read_input()?;

    let true_segments = get_true_segments();
    let permutations = recursive_heaps_algorithm(vec![A, B, C, D, E, F, G] as Permutation);

    let mut sum = 0;
    for (unique_digits, data_segments) in input {
        // Try each permutation
        if let Some(solution) = permutations.iter().find(|permutation| {
            // True segments, permuted by this permutation
            let true_segments_permuted: HashSet<DigitBits> = true_segments
                .iter()
                .map(|bits| permute_bits(&bits, permutation))
                .collect();

            // Substitute wires based on ordering
            // Is every unique digit valid?
            unique_digits
                .iter()
                .all(|digit| true_segments_permuted.contains(&digit))
        }) {
            let mut result_number = 0;
            for data in data_segments {
                let result_digit = true_segments
                    .iter()
                    .enumerate()
                    .find(|(_, num_bits)| permute_bits(num_bits, solution) == data)
                    .unwrap();
                result_number *= 10;
                result_number += result_digit.0;
            }
            println!("Found solution {:?} -> {}", solution, result_number);
            sum += result_number;
        } else {
            println!("No solution found");
        }
    }
    println!("Total sum: {}", sum);
    Ok(())
}
