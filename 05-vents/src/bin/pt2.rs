use std::{
    fmt::Debug,
    io::{self, BufRead, Stdin},
    ops::Add,
    str::FromStr,
};

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct Line {
    start: Point,
    end: Point,
}

impl FromStr for Line {
    type Err = io::Error;
    // "x1,y1 -> x2,y2"
    fn from_str(line_str: &str) -> io::Result<Self> {
        // (x1,y1), (x2,y2)
        let mut ab = line_str.split(" -> ").map(|pt| {
            let mut start_end = pt.split(',').map(|num| num.parse::<i32>()).into_iter();
            return match (start_end.next(), start_end.next()) {
                (Some(Ok(x)), Some(Ok(y))) => Ok(Point { x, y }),
                _ => Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid input")),
            };
        });
        // start, end
        match (ab.next(), ab.next()) {
            (Some(Ok(start)), Some(Ok(end))) => Ok(Line { start, end }),
            _ => Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid input")),
        }
    }
}

impl Add for Line {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Line {
            start: self.start + rhs.start,
            end: self.end + rhs.end,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Cell {
    Empty,
    Marked,
    Overlap,
}
struct Board {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

impl Board {
    fn new(width: usize, height: usize) -> Self {
        Board {
            width,
            height,
            cells: vec![Cell::Empty; width * height],
        }
    }

    fn mark(&mut self, x: i32, y: i32) {
        let idx = (x as usize) + self.width * (y as usize);
        self.cells[idx] = match self.cells[idx] {
            Cell::Empty => Cell::Marked,
            Cell::Marked => Cell::Overlap,
            Cell::Overlap => Cell::Overlap,
        }
    }

    fn write(&mut self, line: &Line) {
        // Assume line is either vertical or horizontal
        let dx = clamp(line.end.x - line.start.x, -1, 1);
        let dy = clamp(line.end.y - line.start.y, -1, 1);
        let d = Point { x: dx, y: dy };
        let mut cur = line.start;
        println!(
            "line: start: {} {} => {} {}",
            line.start.x, line.start.y, dx, dy
        );
        loop {
            self.mark(cur.x, cur.y);
            if cur == line.end {
                break;
            }
            cur = cur + d;
        }
    }

    // TODO: in nightly rust, can use a generator to return incrementally
    fn count_intersections(&self) -> i32 {
        let mut results = 0;
        for x in 0..self.width {
            for y in 0..self.height {
                if self.cells[x + self.width * y] == Cell::Overlap {
                    results += 1;
                }
            }
        }
        return results;
    }
}

impl Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.cells.chunks(self.width).try_for_each(|row| {
            write!(f, "\n")?;
            row.iter().try_for_each(|cell| {
                write!(
                    f,
                    "{}",
                    match cell {
                        Cell::Empty => " ",
                        Cell::Marked => ".",
                        Cell::Overlap => "X",
                    }
                )
            })
        })
    }
}

fn clamp<T: PartialOrd>(x: T, min: T, max: T) -> T {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    return x;
}

fn main() -> io::Result<()> {
    let mut stdin = io::stdin();
    let lines = get_input(&mut stdin)?;

    let (width, height) = lines.iter().fold((0, 0), |(w, h), line| {
        (
            w.max(line.start.x + 1).max(line.end.x + 1),
            h.max(line.start.y + 1).max(line.end.y + 1),
        )
    });
    println!("Decided board is {}x{}", width, height);
    for line in &lines {
        println!("line: {:?}", line);
    }

    let mut board = Board::new(width as usize, height as usize);
    for line in lines {
        board.write(&line);
    }
    println!("Board {:?} ", &board);
    let intersections = board.count_intersections();
    println!("Found {} intersections", intersections);

    Ok(())
}

fn get_input(input: &mut Stdin) -> io::Result<Vec<Line>> {
    input
        .lock()
        .lines()
        .map(|line| line.and_then(|line| line.parse::<Line>()))
        .collect()
}
