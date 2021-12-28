use advent::utils::fgets;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

enum Dir {
    Forward(i32),
    Backward(i32),
    Up(i32),
    Down(i32),
}

/// Loop over all arguments and process each one as a file.
fn main() -> Result<(), Box<dyn Error>> {
    for input in env::args().skip(1) {
        part1(&input)?;
    }
    Ok(())
}

/// Parse a line of text into a Dir and return it.
fn parse(line: &str) -> Dir {
    let v: Vec<&str> = line.split(' ').collect();
    let amount: i32 = v[1].parse().unwrap();
    if v[0] == "forward" {
        return Dir::Forward(amount);
    }
    if v[0] == "backword" {
        return Dir::Backward(amount);
    }
    if v[0] == "up" {
        return Dir::Up(amount);
    }
    Dir::Down(amount)
}
fn part1(name: &str) -> Result<(), Box<dyn Error>> {
    let mut reader = BufReader::new(File::open(name)?);
    let mut line = String::new();
    let (mut h, mut v) = (0i32, 0i32);
    while fgets(&mut reader, &mut line) {
        match parse(&line) {
            Dir::Up(d) => h -= d,
            Dir::Down(d) => h += d,
            Dir::Forward(d) => v += d,
            Dir::Backward(d) => v -= d,
        }
    }
    println!("part1 horis {} depth {}", h, v);
    Ok(())
}
