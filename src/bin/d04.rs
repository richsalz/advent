use advent::utils::Lines;
use std::env;
use std::error::Error;
use std::str::FromStr;

#[derive(Debug)]
struct Board {
    pub values: Vec<u8>,
    pub picked: Vec<bool>,
    pub winner: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    for input in env::args().skip(1) {
        part1(&input)?;
        //        part2(&input)?;
    }
    Ok(())
}

impl Board {
    pub fn new(l: &mut Lines) -> Result<Self, Box<dyn Error>> {
        l.more(); // skip blank line before board
        let mut b = Board {
            values: vec![0; 25],
            picked: vec![false; 25],
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
        println! {"{:#?}", b.values}
        Ok(b)
    }

    pub fn won(self: &Self) -> bool {
        self.winner
    }

    fn five(self: &Self, spots: &Vec<usize>) -> bool {
        spots.iter().find(|&s| !self.picked[*s]).is_none()
    }

    /// See if we won.
    pub fn check(self: &mut Self) -> bool {
        if self.winner {
            return true;
        }
        for row in 0..5 {
            let start: usize = row * 5;
            let r: Vec<usize> = vec![start, start + 1, start + 2, start + 3, start + 4];
            if self.five(&r) {
                self.winner = true;
                return true;
            }
        }
        for col in 0..5 {
            let items: Vec<usize> = vec![col, col + 5, col + 10, col + 15, col + 20];
            if self.five(&items) {
                self.winner = true;
                return true;
            }
        }
        let diag1: Vec<usize> = vec![0, 6, 12, 18, 24];
        let diag2: Vec<usize> = vec![4, 8, 12, 16, 20];
        if self.five(&diag1) || self.five(&diag2) {
            self.winner = true;
            return true;
        }
        false
    }

    pub fn picked(self: &mut Self, number: u8) {
        let x: Vec<usize> = self
            .values
            .iter()
            .enumerate()
            .filter(|(_i, num)| **num == number)
            .map(|(i, _)| i)
            .collect();
        println! {"picked {} len {} vec = {:#?}", number, x.len(), x};
        match x.len() {
            0 => {}
            1 => {
                self.picked[x[0]] = true;
                self.check();
            }
            _ => {
                println! {"multi-match {:?}", x};
            }
        }
    }
}

fn part1(name: &str) -> Result<(), Box<dyn Error>> {
    let mut l = Lines::new(name)?;
    l.more();
    let _moves = l.get();
    let mut b = Board::new(&mut l)?;
    println! {"{:#?}", b};
    b.picked(17);
    if b.won() {
        println! {"you won!"};
    }
    println! {"{:#?}", b};
    Ok(())
}
