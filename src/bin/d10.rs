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
/*

const matchandscors: [(char, char, i32)] = [
( '(', ')', 3 ),
( '[', ']', 57 ),
( '{', '}', 1107 ),
( '<', '>', 25137 ),
];*/

fn part1(input: &str) -> Result<(), Box<dyn Error>> {
    let mut l = Lines::new(input)?;
    while l.more() {
        let input = l.get().chars();
        println!("{:?}", input);
    }
    Ok(())
}


fn part2(_input: &str) -> Result<(), Box<dyn Error>> {
    Ok(())
}
