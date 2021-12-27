use std::env;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use advent::utils::fgets;

enum Direction {
    Forward(u32),
    Backward(u32),
    Up(u32),
    Down(u32),
}

/// Loop over all arguments and process each one as a file.
fn main() -> Result<(), Box<dyn Error>> {
    for input in env::args().skip(1) {
        part1(&input)?;
    }
    Ok(())
}

/// Parse a line of text into a Direction and return it.
fn parse(line: &str) -> Direction {
    if line.starts
}
fn part1(name: &str) -> Result<(), Box<dyn Error>> {
    let mut reader = BufReader::new(File::open(name)?);
    let mut line = String::new();
    while fgets(&mut reader, &mut line) {
        let next: u32 = line.parse()?;
        if next > curr {
            count += 1;
        }
        curr = next;
    }
    println!("part1 {} {}", name, count);
    Ok(())
}
