use advent::utils::Lines;
use std::cmp::Ordering;
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    for input in env::args().skip(1) {
        part1(&input)?;
        part2(&input)?;
    }
    Ok(())
}

fn common(input: &str, diagonals: bool) -> Result<(), Box<dyn Error>> {
    let mut l = Lines::new(input)?;
    let mut grid = vec![vec![0u8; 1000]; 1000];

    while l.more() {
        let s = l.get();
        let parsed = sscanf::scanf!(s, "{},{} -> {},{}", usize, usize, usize, usize);
        let (mut x1, mut y1, x2, y2) = parsed.unwrap();
        if x1 != x2 && y1 != y2 {
            // Skip if not doing diagonals.
            if !diagonals {
                continue;
            }
            let xdiff: isize = (x1 as isize) - (x2 as isize);
            let ydiff: isize = (y1 as isize) - (y2 as isize);
            if xdiff.abs() != ydiff.abs() {
                // Not a proper 45 degree diagnonal, skip.
                continue;
            }
            grid[y1][x1] += 1;
            while x1 != x2 && y1 != y2 {
                match x1.cmp(&x2) {
                    Ordering::Less => x1 += 1,
                    Ordering::Greater => x1 -= 1,
                    _ => (),
                };
                match y1.cmp(&y2) {
                    Ordering::Less => y1 += 1,
                    Ordering::Greater => y1 -= 1,
                    _ => (),
                };
                grid[y1][x1] += 1;
            }
        }
        if x1 == x2 {
            match y1.cmp(&y2) {
                Ordering::Equal => grid[y1][x1] += 1,
                Ordering::Less => {
                    for row in grid.iter_mut().take(y2 + 1).skip(y1) {
                        row[x1] += 1;
                    }
                }
                Ordering::Greater => {
                    for row in grid.iter_mut().take(y1 + 1).skip(y2) {
                        row[x1] += 1;
                    }
                }
            };
        } else {
            match x1.cmp(&x2) {
                Ordering::Less => {
                    for x in x1..=x2 {
                        grid[y1][x] += 1;
                    }
                }
                Ordering::Greater => {
                    for x in x2..=x1 {
                        grid[y1][x] += 1;
                    }
                }
                _ => (),
            }
        }
    }

    //for row in &grid {
    //    println!("{:?}", *row);
    //}
    let mut sum = 0;
    for row in grid {
        sum += row.iter().filter(|&&y| y > 1).count();
    }
    println!("sum {}", sum);
    Ok(())
}

fn part1(input: &str) -> Result<(), Box<dyn Error>> {
    common(input, false)
}

fn part2(input: &str) -> Result<(), Box<dyn Error>> {
    common(input, true)
}
