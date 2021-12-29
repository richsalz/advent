use advent::utils::Lines;
use std::env;
use std::error::Error;

/// Loop over all arguments and process each one as a file.
fn main() -> Result<(), Box<dyn Error>> {
    for input in env::args().skip(1) {
        part1(&input)?;
        part2(&input)?;
    }
    Ok(())
}

fn part1(name: &str) -> Result<(), Box<dyn Error>> {
    // Read each line, store its value; also the line length.
    let mut values: Vec<isize> = Vec::new();
    let mut length = 0;
    let mut l = Lines::new(name)?;
    while l.more() {
        let text = l.get();
        length = text.len();
        values.push(isize::from_str_radix(text, 2)?);
    }

    // Now scan down the columns.
    let (mut gamma, mut epsilon) = (0i32, 0i32);
    for i in 0..length {
        let b = length - i - 1;
        let (mut ones, mut zeros) = (0, 0);
        for v in &values {
            if *v & (1 << b) != 0 {
                ones += 1;
            } else {
                zeros += 1;
            }
        }
        gamma *= 2;
        epsilon *= 2;
        if ones > zeros {
            gamma += 1;
        } else {
            epsilon += 1;
        }
    }

    println!("part1 gamma {} * epsilon {} = {}", gamma, epsilon, gamma * epsilon);
    Ok(())
}

fn part2(name: &str) -> Result<(), Box<dyn Error>> {
    let mut l = Lines::new(name)?;
    while l.more() {
    }
    Ok(())
}
