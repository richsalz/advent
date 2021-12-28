use advent::utils::Lines;
use std::env;
use std::error::Error;

enum Dir {
    Forward(i32),
    Down(i32),
    Up(i32),
}

/// Loop over all arguments and process each one as a file.
fn main() -> Result<(), Box<dyn Error>> {
    for input in env::args().skip(1) {
        part1(&input)?;
        part2(&input)?;
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
    if v[0] == "down" {
        return Dir::Down(amount);
    }
    Dir::Up(amount)
}
fn part1(name: &str) -> Result<(), Box<dyn Error>> {
    let (mut h, mut v) = (0i32, 0i32);
    let mut l = Lines::new(name)?;
    while l.more() {
        match parse(l.get()) {
            Dir::Forward(d) => v += d,
            Dir::Down(d) => h += d,
            Dir::Up(d) => h -= d,
        }
    }
    println!("part1 horiz {} * depth {} = {}", h, v, h * v);
    Ok(())
}

fn part2(name: &str) -> Result<(), Box<dyn Error>> {
    let (mut h, mut v, mut aim) = (0i32, 0i32, 0i32);
    let mut l = Lines::new(name)?;
    while l.more() {
        match parse(l.get()) {
            Dir::Forward(d) => {
                h += d;
                v += d * aim;
            }
            Dir::Down(d) => aim += d,
            Dir::Up(d) => aim -= d,
        }
    }
    println!("part1 horiz {} * depth {} = {}", h, v, h * v);
    Ok(())
}
