//use advent::utils::Lines;
use std::env;
use std::error::Error;
use advent::utils::Lines;

fn main() -> Result<(), Box<dyn Error>> {

    for input in env::args().skip(1) {
        part1(&input)?;
        //part2(&input)?;
    }
    Ok(())
}

fn part1(input: &str) -> Result<(), Box<dyn Error>> {
    let mut l = Lines::new(input)?;
    let mut grid = [[0usize; 10]; 10];

    while l.more() {
        let s = l.get();
        let parsed = sscanf::scanf!(s, "{},{} -> {},{}", usize, usize, usize, usize);
        let (x1, y1, x2, y2) = parsed.unwrap();
        if x1 != x2 && y1 != y2 {
            continue;
        }
        println!("parsed {:?}", parsed);
        if x1 == x2 {
            if y1 > y2 {
                for y in y2..=y1 {
                    grid[x1][y] += 1;
                }
            } else if y2 > x1 {
                for y in y1..=y2 {
                    grid[x1][y] += 1;
                }
            } else {
                grid[x1][y1] += 1
            }
        } else if y1 == y2 {
            if x1 > x2 {
                for x in x2..=x1 {
                    grid[y1][x] += 1;
                }
            } else if x2 > x1 {
                for x in x1..=x2 {
                    grid[x1][x] += 1;
                }
            } else {
                grid[x1][y1] += 1
            }
        }
            
    }
    for i in 0..10 {
        println!("{}: {:?}", i, grid[i]);
    }
    let mut sum = 0;
    for i in 0..10 {
        sum += grid[i].iter().filter(|&&y| grid[i][y] > 1).count();
    }
    println!("sum {}", sum);
    Ok(())
}