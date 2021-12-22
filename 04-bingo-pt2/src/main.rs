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

    #[allow(dead_code)]
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

    // Is this more elegant? Not sure really.
    fn sum_unmarked2(&self, marked: &HashSet<&BingoNumber>) -> BingoNumber {
        let coords = (0..5).flat_map(|row| (0..5).map(move |col| (row, col)));
        coords.fold(0, |sum, (x, y)| {
            let n = self.cells[x][y];
            sum + if marked.contains(&n) { 0 } else { n }
        })
    }
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

// Takes an iterator of 6 strings, consumes 5 and builds a Board, or returns an error
fn read_board<T: Iterator<Item = std::io::Result<String>>>(row_iter: T) -> std::io::Result<Board> {
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
            cells
                .map(|cells| Board { cells })
                .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Not enough rows in board"))
        }
        Err(e) => return Err(e),
    };
    return board;
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
        .map(read_board)
        .collect::<io::Result<Vec<Board>>>();

    return Ok((called_numbers, boards?));
}

fn main() {
    let (called_numbers, boards) = match read_input() {
        Ok(v) => v,
        Err(e) => {
            println!("Input Error: {}", e);
            return;
        }
    };

    let mut remaining_boards = boards;

    for (turn_number, called_number) in called_numbers.iter().enumerate() {
        // Set of numbers that have been called
        let called_set = HashSet::from_iter(called_numbers[0..(turn_number + 1)].iter());

        // Ok, we're down to just one board left, this is the one for the squid
        if remaining_boards.len() == 1 {
            let eventual_winner = remaining_boards.first().unwrap();
            if eventual_winner.is_winner(&called_set) {
                let winner = eventual_winner;
                let unmarked = winner.sum_unmarked2(&called_set);
                println!(
                    "Called {}! We have our last winner! \nIt's {:?} \n unmarked sum: {} product: {}",
                    called_number,
                    winner,
                    unmarked,
                    unmarked * called_number
                );
                return;
            }
        } else {
            println!(
                "Called {}! Boards that haven't won yet: {}.",
                called_number,
                remaining_boards.len()
            );
        }
        // Remove boards that already won
        remaining_boards.retain(|board| !board.is_winner(&called_set));
    }
    println!("No winners! boards");
}
