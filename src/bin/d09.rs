use advent::utils::Lines;
use std::env;
use std::error::Error;

type Matrix = Vec<Vec<u8>>;
type Point = (usize, usize);

/// Loop over all arguments and process each one as a file.
fn main() -> Result<(), Box<dyn Error>> {
    for input in env::args().skip(1) {
        part1(&input)?;
        part2(&input)?;
    }
    Ok(())
}

fn load(input: &str) -> Result<Matrix, Box<dyn Error>> {
    let mut l = Lines::new(input)?;
    let mut m: Matrix = Vec::new();
    while l.more() {
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
        m.push(row);
    }
    Ok(m)
}

fn findlows(m: &Matrix) -> Vec<Point> {
    let mut points: Vec<(usize, usize)> = Vec::new();
    let maxrows = m.len() - 1;
    let maxcols = m[0].len() - 1;
    for row in 0..=maxrows {
        let top: bool = row == 0;
        let bot: bool = row == maxrows;
        for col in 0..=maxcols {
            let cell = m[row][col];
            if (cell == 9
                || !top && cell >= m[row - 1][col])
                || (!bot && cell >= m[row + 1][col])
                || (col != 0 && cell >= m[row][col - 1])
                || (col != maxcols && cell >= m[row][col + 1])
            {
                continue;
            }
            points.push((row, col));
        }
    }
    points
}

fn part1(input: &str) -> Result<(), Box<dyn Error>> {
    let m = load(input)?;
    let points = findlows(&m);
    let risks: isize = points.iter().map(|(r, c)| 1 + m[*r][*c] as isize).sum();
    println!("numrisks {} risks = {}", points.len(), risks);
    Ok(())
}

//const OFFSETS: [(isize, isize); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

// Find the size of the basin at point `p`  As we find neighbors in the basin,
// set those cells to 9; they're ignored in subsequent scans.
fn basinsize(m: &mut Matrix, p: Point) -> isize {
    m[p.0][p.1] = 9; // Don't let this one be re-counted.
    let mut count = 1isize;
    for drow in [-1isize, 1isize] {
        let nrow = (p.0 as isize + drow) as usize;
        if let Some(_) = m.get(nrow) {
            if m[nrow][p.1] != 9 {
                count += basinsize(m, (nrow, p.1));
            }
        }
    }
    for dcol in [-1isize, 1isize] {
        let ncol = (p.1 as isize + dcol) as usize;
        if let Some(_) = m[p.0].get(ncol) {
            if m[p.0][ncol] != 9 {
                count += basinsize(m, (p.0, ncol));
            }
        }
    }
    count
}

fn part2(input: &str) -> Result<(), Box<dyn Error>> {
    let mut m = load(input)?;
    let mut sizes: Vec<isize> = findlows(&m).iter().map(|(r,c)| basinsize(&mut m, (*r, *c))).collect();
    sizes.sort();
    sizes.reverse();
    let product: isize = sizes.iter().take(3).product();
    println!("product = {}", product);

    Ok(())
}
