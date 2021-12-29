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

// Read the file `name` and return a vector of numbers and the line length.
fn readit(name: &str) -> Result<(Vec<usize>, usize), Box<dyn Error>> {
    let mut values: Vec<usize> = Vec::new();
    let mut length: usize = 0;
    let mut l = Lines::new(name)?;
    while l.more() {
        let text = l.get();
        length = text.len();
        values.push(usize::from_str_radix(text, 2)?);
    }
    Ok((values, length))
}

/// Count the numbers of 1 and 0 at bit position `bit` in vector `values`
fn count(bit: usize, values: &Vec<usize>) -> (usize, usize) {
    let (mut ones, mut zeros) = (0, 0);
    for v in values {
        if *v & bit == bit {
            ones += 1;
        } else {
            zeros += 1;
        }
    }
    (ones, zeros)
}

fn part1(name: &str) -> Result<(), Box<dyn Error>> {
    let (values, length) = readit(name)?;

    // Now scan down the columns.
    let (mut gamma, mut epsilon) = (0i32, 0i32);
    for b in (0..length).rev() {
        let (ones, zeros) = count(1 << b, &values);
        gamma *= 2;
        epsilon *= 2;
        if ones > zeros {
            gamma += 1;
        } else {
            epsilon += 1;
        }
    }

    println!(
        "part1 gamma {} * epsilon {} = {}",
        gamma,
        epsilon,
        gamma * epsilon
    );
    Ok(())
}

fn part2(name: &str) -> Result<(), Box<dyn Error>> {
    let (values, length) = readit(name)?;
    let mut oxy = values.clone();
    let mut scrub = values.clone();
    for b in (0..length).rev() {
        if oxy.len() == 1 && scrub.len() == 1 {
            break;
        }
        let bit = 1 << b;
        if oxy.len() > 1 {
            let (ones, zeros) = count(bit, &oxy);
            let wanted = if ones >= zeros { bit } else { 0 };
            oxy.retain(|&v| v & bit == wanted);
        }
        if scrub.len() > 1 {
            let (ones, zeros) = count(bit, &scrub);
            let wanted = if zeros <= ones { 0 } else { bit };
            scrub.retain(|&v| v & bit == wanted);
        }
    }
    println!("{} {}", oxy[0], scrub[0]);
    Ok(())
}
