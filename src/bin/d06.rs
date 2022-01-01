use advent::utils::Lines;
use std::env;
use std::str::FromStr;
use std::error::Error;

/// Loop over all arguments and process each one as a file.
fn main() -> Result<(), Box<dyn Error>> {
    for input in env::args().skip(1) {
        part1(&input)?;
        //part2(&input)?;
    }
    Ok(())
}

fn part1(name: &str) -> Result<(), Box<dyn Error>> {
    let mut l = Lines::new(name)?;
    l.more();
    let mut fish: Vec<u8> = l.get().split(',').map(|s| u8::from_str(s).unwrap()).collect();
    println!("day 0 {:?}", fish);
    for _days in 0..80 {
        let new:usize = fish.iter().filter(|&f| *f == 0).count();
        for f in fish.iter_mut() {
            if *f == 0 { *f = 6; } else { *f -= 1; }
        }
        let mut new: Vec<u8> = vec![8; new];
        fish.append(&mut new);
        //println!("day {} {:?}", days, fish);
    }
    println!("{} fish", fish.len());
    Ok(())
}
