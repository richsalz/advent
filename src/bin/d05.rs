//use advent::utils::Lines;
use std::env;
use std::error::Error;
use advent::utils::Lines;

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
                if x1 < x2 { x1 += 1; } else if x1 > x2 { x1 -= 1; };
                if y1 < y2 { y1 += 1; } else if y1 > y2 { y1 -= 1; };
                grid[y1][x1] += 1;
            }
            continue;
        }
        if x1 == x2 {
            if y1 == y2 {
                grid[y1][x1] += 1;
                continue;
            }
            if y1 < y2 {
                for y in y1..=y2 {
                    grid[y][x1] += 1;
                }
            } else {
                for y in y2..=y1 {
                    grid[y][x1] += 1;
                }
            }
        } else {
            if x1 < x2 {
                for x in x1..=x2 {
                    grid[y1][x] += 1;
                }
            } else {
                for x in x2..=x1 {
                    grid[y1][x] += 1;
                }
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
