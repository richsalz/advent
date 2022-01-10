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

fn corrupted(line: &str) -> Option<usize> {
    let mut stack: Vec<char> = Vec::new();
    for c in line.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            ')' | ']' | '}' | '>' => {
                let (opener, value) = match c {
                    ')' => ('(', 3),
                    ']' => ('[', 57),
                    '}' => ('{', 1197),
                    '>' => ('<', 25137),
                    _ => panic!("unrecognized input character"),
                };
                let stacktop = stack.pop();
                if stacktop.is_none() || stacktop.unwrap() != opener {
                    return Some(value);
                }
            }
            _ => panic!("bad char in input"),
        }
    }
    None
}

fn part1(input: &str) -> Result<(), Box<dyn Error>> {
    let mut l = Lines::new(input)?;
    let mut total: usize = 0;
    while l.more() {
        if let Some(value) = corrupted(l.get()) {
            total += value;
        }
    }
    println!("total = {}", total);
    Ok(())
}

fn part2(input: &str) -> Result<(), Box<dyn Error>> {
    let mut l = Lines::new(input)?;
    let mut stack: Vec<char> = Vec::new();
    let mut scores: Vec<usize> = Vec::new();
    while l.more() {
        stack.clear();
        let line = l.get();
        if corrupted(line).is_some() {
            continue;
        }
        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                _ => {
                    stack.pop();
                }
            };
        }
        stack.reverse();
        let mut sum: usize = 0;
        for c in stack.iter() {
            sum = sum * 5
                + match c {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => {
                        panic!("YABC {} in {}", c, line);
                    }
                };
        }
        scores.push(sum);
    }
    scores.sort_unstable();
    println!("score = {}", scores[scores.len() / 2]);
    Ok(())
}
