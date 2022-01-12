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
        let mut b = Board {
            values: vec![0; 25],
            called: vec![false; 25],
            winner: false,
        };
        for row in 0..5 {
            b.values.splice(
                (row * 5)..(5 + row * 5),
                l.next()
                    .split_ascii_whitespace()
                    .map(|n| u8::from_str(n).unwrap()),
            );
        }
        Ok(b)
    }

    /// Is this board a winner?
    pub fn won(&self) -> bool {
        self.winner
    }

    /// Have all of the squares specified by the five squares
    /// `i0` through `i4` been called?
    fn five(&self, i0: usize, i1: usize, i2: usize, i3: usize, i4: usize) -> bool {
        self.called[i0] && self.called[i1] && self.called[i2] && self.called[i3] && self.called[i4]
    }

    pub fn score(&self) -> i32 {
        (0usize..25usize)
            .filter(|i| !self.called[*i])
            .map(|i| self.values[i] as i32)
            .sum()
    }

    /// See if we won. Check all rows then all columns, then the diagonals.
    /// Yes, I wrote a 5x5 square on paper to figure out what the diagonal
    /// values should be.  
    pub fn check(&mut self) -> bool {
        if self.winner {
            return true;
        }
        for row in [0, 5, 10, 15, 20] {
            if self.five(row, row + 1, row + 2, row + 3, row + 4) {
                self.winner = true;
                return true;
            }
        }
        for col in 0..5 {
            if self.five(col, col + 5, col + 10, col + 15, col + 20) {
                self.winner = true;
                return true;
            }
        }
        // I lost about a day (elapsed time) because I missed the line that said
        // diagonals don't count!!
        //    if self.five(0, 6, 12, 18, 24) || self.five(4, 8, 12, 16, 20) {
        //          self.winner = true;
        //          return true;
        //     }
        false
    }

    /// Set that the number `number` was called.  The `hits` variable
    /// becomes a vector of all indices where the number matches.
    /// Then use that to check for only one hit, and mark it as called.
    /// Return `true` if this was a winning move, else `false`.
    pub fn call(&mut self, number: u8) -> bool {
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
                return self.check();
            }
            _ => {
                eprintln!("multi-match {:?}", hits);
            }
        }
        false
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    for input in env::args().skip(1) {
        part1(&input)?;
        part2(&input)?;
    }
    Ok(())
}

/// Read the file `name` and return a vector of Boards and the vector of squares.
fn setup(name: &str) -> Result<(Vec<Board>, Vec<u8>), Box<dyn Error>> {
    let mut l = Lines::new(name)?;
    l.more();
    let moves: Vec<u8> = l
        .get()
        .split(',')
        .map(|n| u8::from_str(n).ok().unwrap())
        .collect();
    let mut boards: Vec<Board> = Vec::new();
    while l.more() {
        boards.push(Board::new(&mut l)?);
    }
    Ok((boards, moves))
}

/// Part1: Play moves until we have a winner.
fn part1(name: &str) -> Result<(), Box<dyn Error>> {
    let (mut boards, moves) = setup(name)?;
    for m in moves {
        for b in &mut boards {
            if b.call(m) {
                let score = b.score();
                println!("{} * score {} = {}", m, score, score * (m as i32));
                return Ok(());
            }
        }
    }
    Ok(())
}

/// Part2: keep playing, record who the *last* board that wins is so that we can pick
/// that and lose.
fn part2(name: &str) -> Result<(), Box<dyn Error>> {
    let (mut boards, moves) = setup(name)?;
    let mut last = (0, 0);
    for m in moves {
        for (i, b) in boards.iter_mut().enumerate() {
            if !b.won() && b.call(m) {
                last = (i, m);
            }
        }
    }

    let score = boards[last.0].score();
    println!(
        "Board {}: {} * score {} = {}",
        last.0,
        last.1,
        score,
        score * (last.1 as i32)
    );
    Ok(())
}
