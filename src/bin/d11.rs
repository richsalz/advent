use advent::utils::Lines;
use std::env;
use std::error::Error;

type Map = Vec<Vec<i8>>;

/// Loop over all arguments and process each one as a file.
fn main() -> Result<(), Box<dyn Error>> {
    for input in env::args().skip(1) {
        part1(&input)?;
        part2(&input)?;
    }
    Ok(())
}

fn load(input: &str) -> Result<Map, Box<dyn Error>>
{
    let mut l = Lines::new(input)?;
    let mut squids:Map = Vec::with_capacity(10);
    while l.more() {
        squids.push(l.get().chars().map(|c| c.to_digit(10).unwrap() as i8).collect());
    }
    return Ok(squids);
}

fn part1(input: &str) -> Result<(), Box<dyn Error>> {
    let  squids = load(input)?;
    println!("{:#?}", squids);
    Ok(())
}

fn part2(input: &str) -> Result<(), Box<dyn Error>> {
    load(input)?;
    Ok(())
}
