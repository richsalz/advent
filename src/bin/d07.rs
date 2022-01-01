use advent::utils::Lines;
use std::env;
use std::error::Error;
use std::str::FromStr;

/// Loop over all arguments and process each one as a file.
fn main() -> Result<(), Box<dyn Error>> {
    for input in env::args().skip(1) {
        part1(&input)?;
        part2(&input)?;
    }
    Ok(())
}
fn part1(input: &str) -> Result<(), Box<dyn Error>> {
    let cost = |distance: i32| distance;
    common(input, &cost)
}

fn part2(input: &str) -> Result<(), Box<dyn Error>> {
    let cost = |distance: i32| (distance * ( distance + 1)) / 2;
    common(input, &cost)
}

fn common(input: &str, cost: &dyn Fn(i32) -> i32) -> Result<(), Box<dyn Error>> {
    let mut l = Lines::new(input)?;
    l.more();
    let mut crabs: Vec<i32> = l
        .get()
        .split(',')
        .map(|s| i32::from_str(s).unwrap())
        .collect();
    crabs.sort();
    let lowest = crabs[0];
    let highest = crabs[crabs.len() - 1];
    let mut pos = lowest;
    let mut fuel: i32 = crabs.iter().map(|c| cost((c - pos).abs())).sum();
    for newpos in lowest+1..=highest {
        let newfuel: i32 = crabs.iter().map(|c| cost((c - newpos).abs())).sum();
        if newfuel < fuel {
            pos = newpos;
            fuel = newfuel;
        }
    }
    println!("pos = {} fuel = {}", pos, fuel);
    Ok(())
}
