use advent::utils::Lines;
use std::env;
use std::error::Error;

type Map = Vec<Vec<i8>>;
type Point = (usize, usize);

/// Loop over all arguments and process each one as a file.
fn main() -> Result<(), Box<dyn Error>> {
    for input in env::args().skip(1) {
        part1(&input, 100)?;
        part2(&input)?;
    }
    Ok(())
}

// Yes I know they are octopi, but squids is easier to type.
fn load(input: &str) -> Result<Map, Box<dyn Error>> {
    let mut l = Lines::new(input)?;
    let mut squids: Map = Vec::with_capacity(10);
    while l.more() {
        squids.push(
            l.get()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as i8)
                .collect(),
        );
    }
    Ok(squids)
}

/* fn print(m: &Map) {
    for (nr, r) in m.iter().enumerate() {
        println!("{} {:?}", nr, r);
    }
}*/

const OFFSETS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn part1(input: &str, days: isize) -> Result<(), Box<dyn Error>> {
    let mut squids = load(input)?;
    let mut flashers: Vec<Point> = Vec::new();
    let mut total = 0usize;
    for _d in 0..days {
        // print(&squids);
        // Increment all octopi, record location in flashers if they hit.
        for (nr, row) in squids.iter_mut().enumerate() {
            for (nc, cell) in row.iter_mut().enumerate() {
                *cell += 1;
                if *cell >= 9 {
                    flashers.push((nr, nc));
                }
            }
        }
        loop {
            let mut newbies: Vec<Point> = Vec::new();
            for (fr, fc) in &flashers {
                let mut newest: Vec<Point> = OFFSETS
                    .iter()
                    .map(|(dr, dc)| ((*fr as isize + dr) as usize, (*fc as isize + dc) as usize))
                    .filter(|(nr, nc)| squids.get(*nr).is_some() && squids[*nr].get(*nc).is_some())
                    .filter(|(nr, nc)| squids[*nr][*nc] >= 8 && !flashers.contains(&(*nr, *nc)))
                    .collect();
                newest.iter().for_each(|(nr, nc)| squids[*nr][*nc] += 1);
                newbies.append(&mut newest);
            }
            println!("day {} flashers was {} adding {}", _d, flashers.len(), newbies.len());
            if newbies.len() == 0 {
                break;
            }
            flashers.append(&mut newbies);
        }
        total += flashers.len();
        for (nr, nc) in &flashers {
            squids[*nr][*nc] = 0;
        }
        flashers.clear();
    }
    println!("{}", total);
    Ok(())
}

fn part2(input: &str) -> Result<(), Box<dyn Error>> {
    load(input)?;
    Ok(())
}
