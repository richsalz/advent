use advent::utils::Lines;
use std::env;
use std::error::Error;

/// Loop over all arguments and process each one as a file.
fn main() -> Result<(), Box<dyn Error>> {
    for input in env::args().skip(1) {
        part1(&input)?;
        //part2(&input)?;
    }
    Ok(())
}

fn parse(line: &str) -> Result<(Vec<&str>, Vec<&str>), Box<dyn Error>> {
    let mut s = line.split('|');
    let inp = s.next().unwrap().trim().split(' ').collect();
    let out = s.next().unwrap().trim().split(' ').collect();
    Ok((inp, out))
}

fn part1(input: &str) -> Result<(), Box<dyn Error>> {
    let segments = [ 6, 2, 5, 5, 4, 5, 6, 3, 7, 6];
    let uniques = [ segments[1], segments[4], segments[7], segments[8]];

    let mut l = Lines::new(&input)?;
    let mut count = 0;
    while l.more() {
        let (inp, out) = parse(l.get())?;
        println!("{:?} -> {:?}", inp, out);
        count += out.iter().filter(|s| uniques.contains(&s.len())).count();
    }
    println!("uniques {}", count);
    Ok(())
}