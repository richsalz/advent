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

/// Given the character `c` which is a closing bracket, return the opening
/// bracket we expect to see, and the cost if it doesn't match.
fn match_closer(c: char) -> (char, usize) {
    match c {
        ')' => return ( '(', 3 ),
        ']' => return ( '[', 57 ),
        '}' => return ( '{', 1197 ),
        '>' => return ( '<', 25137 ),
        _ => panic!("unrecognized input character"),
    }
}
fn part1(input: &str) -> Result<(), Box<dyn Error>> {
    let mut l = Lines::new(input)?;
    let mut stack: Vec<char> = Vec::new();
    let mut total: usize = 0;
    while l.more() {
        stack.clear();
        let line = l.get();
        //println!("{}", line);
        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => { stack.push(c); continue; },
                ')' | ']' | '}' | '>' => {
                    let (opener, value) = match_closer(c);
                    let stacktop = stack.pop();
                    if stacktop.is_none() || stacktop.unwrap() != opener {
                        //println!("Mismatch; got {} wanted {} score {}", c, opener, value);
                        total += value;
                        break;
                    }
                },
                _ => panic!("bad char in input"),
            }
        }
    }
    println!("total = {}", total);
    Ok(())
}


fn part2(_input: &str) -> Result<(), Box<dyn Error>> {
    Ok(())
}
