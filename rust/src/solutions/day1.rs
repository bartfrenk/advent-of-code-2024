use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use std::vec::Vec;

pub fn greeting(s: &str) {
    println!("Hello, {}!", s)
}

fn read_input(path: impl AsRef<Path>) -> io::Result<(Vec<i64>, Vec<i64>)> {
    let file = File::open(path)?;
    let x = vec![];
    let y = vec![];
    let reader = io::BufReader::new(file);
    for line in reader.lines() {
        let x: Vec<&str> = line?.split(" ").collect();
        println!("{:?}", x);
    }
    return Ok((x, y));
}

#[cfg(test)]
mod tests {
    use crate::solutions::day1::*;

    #[test]
    fn test_read_input() {
        read_input("../inputs/day1.rs");
    }
}
