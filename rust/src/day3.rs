use regex::{Captures, Regex};
use std::env::args;
use std::fs::File;
use std::io;
use std::process::exit;

fn read<S: io::Read>(stream: &mut S) -> String {
    let mut contents = String::new();
    stream.read_to_string(&mut contents).unwrap();
    contents
}

#[derive(Debug)]
enum Instruction {
    Do,
    Dont,
    Mul(i64, i64),
}

impl Instruction {
    fn from_captures(caps: Captures) -> Instruction {
        match &caps[0] {
            "do()" => Instruction::Do,
            "don't()" => Instruction::Dont,
            _ => Instruction::Mul(str::parse(&caps[1]).unwrap(), str::parse(&caps[2]).unwrap()),
        }
    }
}

struct State {
    total: i64,
    enabled: bool,
}

impl State {
    fn init() -> State {
        State {
            total: 0,
            enabled: true,
        }
    }
}

fn part2(s: &str) -> i64 {
    let re = Regex::new(r"mul\((\d{1,3}?),(\d{1,3}?)\)|do\(\)|don't\(\)").unwrap();
    let instructions = re.captures_iter(s).map(Instruction::from_captures);

    let mut state = State::init();
    for i in instructions {
        match i {
            Instruction::Do => state.enabled = true,
            Instruction::Dont => state.enabled = false,
            Instruction::Mul(x, y) => {
                if state.enabled {
                    state.total += x * y;
                }
            }
        }
    }
    state.total
}

fn part1(s: &str) -> i64 {
    let re = Regex::new(r"mul\((\d{1,3}?),(\d{1,3}?)\)").unwrap();
    re.captures_iter(s)
        .map(|caps| {
            let (_, [s, t]) = caps.extract();
            str::parse::<i64>(s).unwrap() * str::parse::<i64>(t).unwrap()
        })
        .sum()
}

pub fn main() {
    match args().skip(1).next() {
        None => exit(1),
        Some(path) => {
            let mut file = File::open(&path).unwrap();
            let contents = read(&mut file);
            println!("Part1: {}", part1(&contents));
            println!("Part2: {}", part2(&contents));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_example_part1() {
        let s = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(part1(s), 161);
    }

    #[test]
    fn test_example_part2() {
        let s = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(part2(s), 48);
    }

    #[test]
    fn run_part1() {
        let mut file = File::open("inputs/day3.txt").unwrap();
        let s = read(&mut file);
        println!("{:?}", part1(&s));
    }

    #[test]
    fn run_part2() {
        let mut file = File::open("inputs/day3.txt").unwrap();
        let s = read(&mut file);
        println!("{:?}", part2(&s));
    }
}
