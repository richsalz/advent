use advent::utils::Lines;
use std::env;
use std::error::Error;

type Matrix = Vec<Vec<u8>>;

/// Loop over all arguments and process each one as a file.
fn main() -> Result<(), Box<dyn Error>> {
    for input in env::args().skip(1) {
        part1(&input)?;
        //part2(&input)?;
    }
    Ok(())
}

fn part1(input: &str) -> Result<(), Box<dyn Error>> {
    let mut l = Lines::new(input)?;
    let mut m: Matrix = Vec::new();
    let mut maxcols = 0;
    while l.more() {
        maxcols = l.get().len() - 1;
        let row = l
            .get()
            .chars()
            .map(|c| match c {
                '1' => 1,
                '2' => 2,
                '3' => 3,
                '4' => 4,
                '5' => 5,
                '6' => 6,
                '7' => 7,
                '8' => 8,
                '9' => 9,
                _ => 0,
            })
            .collect();
        println!("{:?}", row);
        m.push(row);
    }
    let maxrows = m.len() - 1;

    let mut risks: isize = 0;
    let mut numrisks: isize = 0;
    for row in 0..=maxrows {
        let top: bool = row == 0;
        let bot: bool = row == maxrows;
        for col in 0..=maxcols {
            let cell = m[row][col];
            if !top && cell >= m[row-1][col] { continue; }
            if !bot && cell >= m[row+1][col] { continue; }
            if col != 0 && cell >= m[row][col-1] { continue; }
            if col != maxcols && cell >= m[row][col+1] { continue; }
            risks += cell as isize + 1;
            numrisks += 1;
        }
    }
    println!("numrisks {} risks = {}", numrisks, risks);
    Ok(())
}
