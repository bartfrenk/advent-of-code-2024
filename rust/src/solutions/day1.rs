use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use std::vec::Vec;

fn read_input(path: impl AsRef<Path>) -> io::Result<(Vec<i64>, Vec<i64>)> {
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

fn compute_result(xs: &mut Vec<i64>, ys: &mut Vec<i64>) -> i64 {
    xs.sort();
    ys.sort();
    xs.iter().zip(ys).map(|(x, y)| (*x - *y).abs()).sum()
}

pub fn result() -> i64 {
    let (mut xs, mut ys) = read_input("inputs/day1.txt").unwrap();
    compute_result(&mut xs, &mut ys)
}

#[cfg(test)]
mod tests {
    use crate::solutions::day1::*;

    #[test]
    fn test_example() {
        let mut xs = vec![3, 4, 2, 1, 3, 3];
        let mut ys = vec![4, 3, 5, 3, 9, 3];
        assert!(compute_result(&mut xs, &mut ys) == 11);
    }
}
