use std::collections::HashMap;
use std::env::args;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use std::process::exit;
use std::vec::Vec;

fn read(path: impl AsRef<Path>) -> io::Result<(Vec<i64>, Vec<i64>)> {
    let file = File::open(path)?;
    let mut xs = Vec::new();
    let mut ys = Vec::new();
    let reader = io::BufReader::new(file);
    for line in reader.lines() {
        let owned = line?;
        let mut words = owned.split(" ").filter(|x| !x.is_empty());
        let x = str::parse::<i64>(words.next().unwrap()).unwrap();
        let y = str::parse::<i64>(words.next().unwrap()).unwrap();
        xs.push(x);
        ys.push(y);
    }
    Ok((xs, ys))
}

fn part1(xs: &mut [i64], ys: &mut [i64]) -> i64 {
    xs.sort();
    ys.sort();
    xs.iter().zip(ys).map(|(x, y)| (*x - *y).abs()).sum()
}

fn part2(xs: &[i64], ys: &[i64]) -> i64 {
    let mut counts = HashMap::new();
    for y in ys {
        let count = counts.entry(*y).or_insert(0);
        *count += 1;
    }
    xs.iter().map(|x| counts.get(x).unwrap_or(&0) * x).sum()
}

pub fn main() {
    match args().skip(1).next() {
        None => exit(1),
        Some(path) => {
            let (mut xs, mut ys) = read(&path).unwrap();
            println!("Part 1: {}", part1(&mut xs, &mut ys));
            println!("Part 2: {}", part2(&xs, &ys));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        let mut xs = vec![3, 4, 2, 1, 3, 3];
        let mut ys = vec![4, 3, 5, 3, 9, 3];
        assert!(part1(&mut xs, &mut ys) == 11);
    }

    #[test]
    fn test_example_part2() {
        let xs = vec![3, 4, 2, 1, 3, 3];
        let ys = vec![4, 3, 5, 3, 9, 3];
        assert!(part2(&xs, &ys) == 31);
    }
}
