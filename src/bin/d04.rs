use advent::utils::Lines;
use std::env;
use std::error::Error;
use std::str::FromStr;

/// The board.  Since it is fixed (5x5) size, just use flattened vectors.
#[derive(Debug)]
struct Board {
    /// The values of the squares on the board.
    values: Vec<u8>,
    /// True/false if this number was called.
    called: Vec<bool>,
    /// Did we win?
    winner: bool,
}

impl Board {
    /// Constructor, read board from file `l`  Boards start with a blank line.
    pub fn new(l: &mut Lines) -> Result<Self, Box<dyn Error>> {
        l.more(); // skip blank line before board
        let mut b = Board {
            values: vec![0; 25],
            called: vec![false; 25],
            winner: false,
        };
        let mut i: usize = 0;
        for _row in 0..5 {
            l.more();
            for n in l.get().split_ascii_whitespace() {
                b.values[i] = u8::from_str(n)?;
                i += 1;
            }
        }
        Ok(b)
    }

    /// Is this board a winner?
    pub fn won(self: &Self) -> bool {
        self.winner
    }

    /// Have all of the squares specified by the five-element tuple
    /// `i` been called?
    fn five(self: &Self, i: (usize, usize, usize, usize, usize)) -> bool {
        self.called[i.0]
            && self.called[i.1]
            && self.called[i.2]
            && self.called[i.3]
            && self.called[i.4]
    }

    /// See if we won. Check all rows then all columns, then the diagonals.
    /// Yes, I wrote a 5x5 square on paper to figure out what the diagonal
    /// values should be.  
    pub fn check(self: &mut Self) -> bool {
        if self.winner {
            return true;
        }
        for row in [0, 5, 10, 15, 20] {
            if self.five((row, row + 1, row + 2, row + 3, row + 4)) {
                self.winner = true;
                return true;
            }
        }
        for col in 0..5 {
            if self.five((col, col + 5, col + 10, col + 15, col + 20)) {
                self.winner = true;
                return true;
            }
        }
    // I lost about a day (elapsed time) because I missed the line that said
    // diagonals don't count!!
    //    if self.five((0, 6, 12, 18, 24)) || self.five((4, 8, 12, 16, 20)) {
    //          self.winner = true;
    //          return true;
    //     }
        false
    }

    /// Set that the number `number` was called.  The `hits` variable
    /// becomes a vector of all indices where the number matches.
    /// Then use that to check for only one hit, and mark it as called.
    pub fn call(self: &mut Self, number: u8) {
        let hits: Vec<usize> = self
            .values
            .iter()
            .enumerate()
            .filter(|(_i, num)| **num == number)
            .map(|(i, _)| i)
            .collect();
        match hits.len() {
            0 => {}
            1 => {
                self.called[hits[0]] = true;
                self.check();
            }
            _ => {
                eprintln!("multi-match {:?}", hits);
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    for input in env::args().skip(1) {
        part1(&input)?;
        //        part2(&input)?;
    }
    Ok(())
}

fn part1(name: &str) -> Result<(), Box<dyn Error>> {
    let mut l = Lines::new(name)?;
    l.more();
    let mut moves: Vec<u8> = Vec::new();
    for n in l.get().split(',') {
        moves.push(u8::from_str(n)?);
    }
    let mut boards: Vec<Board> = Vec::with_capacity(3);
    for _ in [0, 1, 2] {
        boards.push(Board::new(&mut l)?);
    }

    for m in moves {
        println!("calling {}", m);
        for i in [2] {
            boards[i].call(m);
            if boards[i].won() {
                println!("Board {} wins!", i);
                return Ok(());
            }
        }
    }
    Ok(())
}
