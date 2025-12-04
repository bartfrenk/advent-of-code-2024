use std::env::args;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::process::exit;

#[derive(Debug)]
struct Equation {
    lhs: u64,
    rhs: Vec<u64>,
}

impl Equation {
    fn parse(s: &str) -> Equation {
        let mut split = s.split(':');
        let lhs = str::parse(split.next().unwrap()).unwrap();
        let rhs = split
            .next()
            .unwrap()
            .split(' ')
            .filter(|s| !s.trim().is_empty())
            .map(|s| str::parse(s).unwrap())
            .rev()
            .collect();
        return Equation { lhs: lhs, rhs: rhs };
    }

    fn read<S: io::Read>(stream: &mut S) -> impl Iterator<Item = Equation> {
        let reader = io::BufReader::new(stream);
        return reader.lines().map(|res| Equation::parse(&res.unwrap()));
    }

    fn possible(&self) -> bool {
        fn pred(lhs: u64, rhs: &[u64]) -> bool {
            if rhs.len() == 1 {
                return lhs == rhs[0];
            };
            if lhs > rhs[0] && pred(lhs - rhs[0], &rhs[1..]) {
                return true;
            }
            if lhs % rhs[0] == 0 && pred(lhs / rhs[0], &rhs[1..]) {
                return true;
            }
            return false;
        }

        return pred(self.lhs, &self.rhs);
    }
}

fn part1(path: &str) -> u64 {
    let mut file = File::open(&path).unwrap();
    return Equation::read(&mut file)
        .filter(|eqn| eqn.possible())
        .map(|eqn| eqn.lhs)
        .sum();
}

fn main() {
    match args().nth(1) {
        None => {
            println!("Specify an input file");
            exit(1);
        }
        Some(path) => {
            println!("Part 1: {}", part1(&path));
        }
    }
}
