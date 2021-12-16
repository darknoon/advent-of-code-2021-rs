use std::io::{self, BufRead};

enum Move {
    Forward(i32),
    Down(i32),
    Up(i32),
}

fn parse_move(s: &str) -> Option<Move> {
    // "forward 3"
    let mut components = s.split(" ");
    // ie "forward"
    let action: Option<&str> = components.next();
    // ie "3"
    let amount: Option<i32> = components
        .next()
        .map(|nstr| nstr.parse::<i32>().ok())
        .flatten();

    return both((action, amount))
        .map(|tup| match tup {
            ("forward", n) => Some(Move::Forward(n)),
            ("down", n) => Some(Move::Down(n)),
            ("up", n) => Some(Move::Up(n)),
            (_, _) => None,
        })
        .flatten();
}

// In a (Option<A>, Option<B>), ar both tuple arguments Some? if so, return Some<(A,B)>, else None
fn both<A, B>(ab: (Option<A>, Option<B>)) -> Option<(A, B)> {
    return match ab {
        (Some(aa), Some(bb)) => Some((aa, bb)),
        _ => None,
    };
}

fn main() {
    println!("Reading input");

    let mut pos: i32 = 0;
    let mut depth: i32 = 0;

    io::stdin()
        .lock()
        .lines()
        .filter_map(|line| line.ok()) // need to filter out errors
        .filter_map(|line| parse_move(line.as_str()))
        .for_each(|m| match m {
            Move::Forward(f) => pos += f,
            Move::Down(f) => depth += f,
            Move::Up(f) => depth -= f,
        });

    println!("pos: {} depth: {} product: {}", pos, depth, pos * depth);
}
