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

// Read the file `name` and return a vector of numbers and the line length.
fn readit(name: &str) -> Result<(Vec<usize>, usize), Box<dyn Error>> {
    let mut values: Vec<usize> = Vec::new();
    let mut length: usize = 0;
    let mut l = Lines::new(name)?;
    while l.more() {
        let text = l.get();
        length = text.len();
        values.push(usize::from_str_radix(text, 2)?);
    }
    Ok((values, length))
}

/// Count the numbers of 1's and 0's at bit position `bit` in each element
/// of the vector `values`
fn count(bit: usize, values: &Vec<usize>) -> (usize, usize) {
    let (mut ones, mut zeros) = (0, 0);
    for v in values {
        if *v & bit == bit {
            ones += 1;
        } else {
            zeros += 1;
        }
    }
    (ones, zeros)
}

/// Part1
fn part1(name: &str) -> Result<(), Box<dyn Error>> {
    let (values, length) = readit(name)?;

    // Now scan down the columns.
    let (mut gam, mut eps) = (0i32, 0i32);
    for b in (0..length).rev() {
        let (ones, zeros) = count(1 << b, &values);
        gam *= 2;
        eps *= 2;
        if ones > zeros {
            gam += 1;
        } else {
            eps += 1;
        }
    }

    println!("part1 gam {} * eps {} = {}", gam, eps, gam * eps);
    Ok(())
}

/// Filter the `values` (each of `length` bits) looking at each bit in turn.
/// Remove those that don't match the `result` value.
fn filter(
    values: &Vec<usize>,
    length: usize,
    result: &dyn Fn(usize, usize, usize) -> usize,
) -> usize {
    let mut values = values.clone();
    for b in (0..length).rev() {
        if values.len() == 1 {
            break;
        }
        let bit = 1 << b;
        let (ones, zeros) = count(bit, &values);
        let wanted = result(ones, zeros, bit); //if ones >= zeros { bit } else { 0 };
        values.retain(|&v| v & bit == wanted);
    }
    values[0]
}

fn part2(name: &str) -> Result<(), Box<dyn Error>> {
    let (values, length) = readit(name)?;
    let most = |ones: usize, zeros: usize, bit: usize| if ones >= zeros { bit } else { 0 };
    let least = |ones: usize, zeros: usize, bit: usize| if zeros <= ones { 0 } else { bit };
    let oxy = filter(&values, length, &most);
    let co2 = filter(&values, length, &least);

    println!("oxy {} co2 {} product {}", oxy, co2, oxy * co2);
    Ok(())
}
