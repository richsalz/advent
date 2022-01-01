use advent::utils::Lines;
use std::env;
use std::str::FromStr;
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
    common(name, 80)
}

fn part2(name: &str) -> Result<(), Box<dyn Error>> {
    common(name, 256)
}

fn common(name: &str, days: isize) -> Result<(), Box<dyn Error>> {
    let mut l = Lines::new(name)?;
    l.more();
    let mut fish: Vec<usize> = vec![0; 9];
    for num in l.get().split(',').map(|s| usize::from_str(s).unwrap()) {
        fish[num] += 1;
    }
    println!("\nday {} {:?}({}) = {}", 0, fish, fish.len(), fish.iter().sum::<usize>());
    for _d in 0..days {
        let new = fish.remove(0);
        fish.push(new);
        fish[6] += new;
        //println!("day {} {:?}({}) = {}", d+1, fish, fish.len(), fish.iter().sum::<usize>());
    }
    println!("\nday {} {:?} = {}", days, fish, fish.iter().sum::<usize>());
    Ok(())
}
