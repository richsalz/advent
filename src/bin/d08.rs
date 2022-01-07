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

type Wordlist<'a> = Vec<&'a str>;

fn parse(line: &str) -> Result<(Wordlist, Wordlist), Box<dyn Error>> {
    let mut s = line.split('|');
    let inp: Vec<&str> = s.next().unwrap().trim().split(' ').collect();
    let out: Vec<&str> = s.next().unwrap().trim().split(' ').collect();
    assert_eq!(inp.len(), 10);
    assert_eq!(out.len(), 4);
    Ok((inp, out))
}

fn part1(input: &str) -> Result<(), Box<dyn Error>> {
    let segments = [6, 2, 5, 5, 4, 5, 6, 3, 7, 6];
    let uniques = [segments[1], segments[4], segments[7], segments[8]];

    let mut l = Lines::new(input)?;
    let mut count = 0;
    while l.more() {
        let (_inp, out) = parse(l.get())?;
        //println!("{:?} -> {:?}", inp, out);
        count += out.iter().filter(|s| uniques.contains(&s.len())).count();
    }
    println!("uniques {}", count);
    Ok(())
}

fn part2(input: &str) -> Result<(), Box<dyn Error>> {
    let mut l = Lines::new(input)?;
    let mut total = 0;
    while l.more() {
        let (inp, out) = parse(l.get())?;
        let one = inp.iter().find(|d| d.len() == 2).unwrap();
        let four = inp.iter().find(|d| d.len() == 4).unwrap();
        let mut sum: isize = 0;
        for word in &out {
            sum = sum * 10
                + match word.len() {
                    // Simple unique matches of number of segments to number.
                    2 => 1,
                    3 => 7,
                    4 => 4,
                    7 => 8,
                    // Have to do a harder comparison. Match this triplet:
                    //     word_length, #segments from "1", #segements from "4"
                    // From that we can uniquely determine which digit is lit.
                    len => match (
                        len,
                        word.chars().filter(|&b| one.contains(b)).count(),
                        word.chars().filter(|&b| four.contains(b)).count(),
                    ) {
                        (5, 1, 3) => 5,
                        (5, 2, 3) => 3,
                        (5, _, 2) => 2,
                        (6, 1, _) => 6,
                        (6, _, 3) => 0,
                        (6, _, 4) => 9,
                        _ => unreachable!(),
                    },
                }
        }
        total += sum;
        println!("{:?} -> {}", out, sum);
    }
    println!("{}", total);
    Ok(())
}
