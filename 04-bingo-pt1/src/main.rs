use itertools::Itertools;
use std::{
    collections::HashSet,
    fmt::Debug,
    io::{self, BufRead},
    str::FromStr,
};

type BingoNumber = i32;

#[derive(Debug, Eq, PartialEq)]
struct Board {
    cells: [[i32; 5]; 5],
}

struct BoardRow {
    cells: [i32; 5],
}

impl FromStr for BoardRow {
    type Err = io::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        parse_row2(s)
    }
}

impl Board {
    fn is_winner(&self, marked: &HashSet<&BingoNumber>) -> bool {
        // Check for a winner starting at a position moving in given direction
        let check_win = |start: (i32, i32), dir: (i32, i32)| -> bool {
            for i in 0..5 {
                let (x, y) = (start.0 + i * dir.0, start.1 + i * dir.1);
                if !marked.contains(&self.cells[x as usize][y as usize]) {
                    return false;
                }
            }
            return true;
        };

        // Check rows
        for row in 0..5 {
            if check_win((row, 0), (0, 1)) {
                return true;
            }
        }
        for col in 0..5 {
            if check_win((0, col), (1, 0)) {
                return true;
            }
        }
        // Check diagonals (oops, turns out this isn't valid in this version of bingo)
        // if check_win((0, 0), (1, 1)) {
        //     return true;
        // }

        // if check_win((0, 4), (1, -1)) {
        //     return true;
        // }
        return false;
    }

    fn sum_unmarked(&self, marked: &HashSet<&BingoNumber>) -> BingoNumber {
        let mut sum = 0;
        for row in 0..5 {
            for col in 0..5 {
                let n = &self.cells[row][col];
                if !marked.contains(n) {
                    sum += n;
                }
            }
        }
        sum
    }
}

// Not particularly elegant, but handles invalid input
fn parse_row(row: &str) -> std::io::Result<BoardRow> {
    let mut numbers = [0; 5];
    let pairs: Vec<(&mut BingoNumber, &str)> =
        numbers.iter_mut().zip(row.split_whitespace()).collect();

    if pairs.len() != 5 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Invalid row length",
        ));
    }
    for (result, nstr) in pairs {
        match nstr.parse::<BingoNumber>() {
            Ok(n) => *result = n,
            Err(_) => return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid number")),
        }
    }
    Ok(BoardRow { cells: numbers })
}

fn parse_row2(row: &str) -> std::io::Result<BoardRow> {
    let row_nums = row
        .split_whitespace()
        .map(|nstr| nstr.parse::<BingoNumber>())
        .map(|result| {
            result.map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid number"))
        })
        .into_iter()
        .collect::<io::Result<Vec<_>>>()?;

    row_nums
        .into_iter()
        .to_array5()
        .map(|numbers| BoardRow { cells: numbers })
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid row length"))
}

enum VecToArrayError {
    InvalidLength,
}

trait ToArray<T> {
    fn to_array5(self) -> Result<[T; 5], VecToArrayError>;
}

impl<T, I> ToArray<T> for I
where
    I: Iterator<Item = T>,
    T: Default,
    T: Debug,
    T: Copy, // Allows [T::default(); 5] vs [T::default(), T::default(), …]
{
    fn to_array5(self) -> Result<[T; 5], VecToArrayError> {
        let mut arr = [T::default(); 5];
        let vec = self.collect::<Vec<T>>();
        if vec.len() != 5 {
            return Err(VecToArrayError::InvalidLength);
        }
        for i in 0..5 {
            arr[i] = vec[i];
        }
        return Ok(arr);
    }
}

fn read_input() -> std::io::Result<(Vec<BingoNumber>, Vec<Board>)> {
    let stdin = io::stdin();
    let mut stdin_iter = stdin.lock().lines().into_iter();

    let called_numbers_str = match stdin_iter.next() {
        Some(Ok(line)) => line,
        Some(Err(e)) => return Err(e),
        None => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Couldn't read first line",
            ))
        }
    };

    // 0,5,8,9... etc
    let called_numbers = called_numbers_str
        .split(',')
        .map(|s| {
            s.parse::<BingoNumber>()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
        })
        .collect::<io::Result<Vec<BingoNumber>>>()?;

    // Don't care about this line. Skip
    stdin_iter.next();

    if called_numbers.len() < 5 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Not enough numbers for a winner",
        ));
    }

    let boards = stdin_iter
        .chunks(6)
        .into_iter()
        .map(|row_iter| {
            // Make an empty board to write into
            // TODO: how can I spell the type of this to separate it out into a function?
            // let mut board = Board { cells: [[0; 5]; 5] };
            let board_rows: io::Result<Vec<_>> = row_iter
                .take(5)
                .map(|line| match line {
                    Ok(line) => line.parse::<BoardRow>(),
                    Err(e) => Err(e),
                })
                .collect();

            let board: io::Result<Board> = match board_rows {
                Ok(rows) => {
                    let cells = rows.iter().map(|row| row.cells).to_array5();
                    cells.map(|cells| Board { cells }).map_err(|_| {
                        io::Error::new(io::ErrorKind::InvalidData, "Not enough rows in board")
                    })
                }
                Err(e) => return Err(e),
            };
            return board;
        })
        .collect::<io::Result<Vec<Board>>>();

    return Ok((called_numbers, boards?));
}

fn main() {
    let (called_numbers, boards) = read_input().unwrap();

    for (turn_number, called_number) in called_numbers.iter().enumerate() {
        // Set of numbers that have been called
        let called_set = HashSet::from_iter(called_numbers[0..(turn_number + 1)].iter());
        // Look for a winner
        if let Some(winner) = boards.iter().find(|board| board.is_winner(&called_set)) {
            let unmarked = winner.sum_unmarked(&called_set);
            println!(
                "Called {}! We have a winner! \nIt's {:?} \n unmarked sum: {} product: {}",
                called_number,
                winner,
                unmarked,
                unmarked * called_number
            );
            return;
        } else {
            println!("Called {}! No winner yet.", called_number);
        }
    }
    println!("No winners!");
}
