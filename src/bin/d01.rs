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

/// Reads the file named `name` which is taken as a set of numbers,
/// one per line. Read the first line and store the number. Read
/// the rest of the file, and when the new line is greater than the
/// current, increment `count`. Replace the newline as the current
/// value and continue.  At EOF, print the filename and total number
/// of increases found.
fn part1(name: &str) -> Result<(), Box<dyn Error>> {
    let mut l = Lines::new(name)?;
    l.more();
    let mut curr: u32 = l.get().parse()?;
    let mut count = 0u32;
    while l.more() {
        let next: u32 = l.get().parse()?;
        if next > curr {
            count += 1;
        }
        curr = next;
    }
    println!("part1 {} {}", name, count);
    Ok(())
}

fn part2(name: &str) -> Result<(), Box<dyn Error>> {
    let mut l = Lines::new(name)?;
    let mut numbers: Vec<u32> = Vec::new();
    // Create a vector of numbers based on the file contents.
    while l.more() {
        let curr = l.get().parse()?;
        numbers.push(curr);
    }
    let mut triples: Vec<u32> = Vec::with_capacity(numbers.len() / 3 + 1);
    for n in numbers.as_slice().windows(3) {
        triples.push(n[0] + n[1] + n[2]);
    }
    let mut count = 0u32;
    let mut curr = triples[0];
    for n in triples.as_slice().iter().skip(1) {
        if *n > curr {
            count += 1;
        }
        curr = *n;
    }
    println!("part 2 {} {}", name, count);
    Ok(())
}
