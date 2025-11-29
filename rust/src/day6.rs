use std::collections::HashSet;
use std::env::args;
use std::fs::File;
use std::io;
use std::io::{BufRead, Error};
use std::process::exit;

#[derive(Clone, Copy)]
enum Pixel {
    Empty,
    Wall,
}

#[derive(Debug)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl Dir {
    fn step(&self, pos: &Point) -> Point {
        match &self {
            Dir::Up => Point(pos.0 - 1, pos.1),
            Dir::Right => Point(pos.0, pos.1 + 1),
            Dir::Down => Point(pos.0 + 1, pos.1),
            Dir::Left => Point(pos.0, pos.1 - 1),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point(i32, i32);

#[derive(Debug)]
struct Guard {
    pos: Point,
    dir: Dir,
}

impl Guard {
    fn new(i: usize, j: usize, ch: char) -> Result<Guard, String> {
        let pos = Point(i as i32, j as i32);
        match ch {
            '^' => {
                return Ok(Guard {
                    pos: pos,
                    dir: Dir::Up,
                })
            }
            '>' => {
                return Ok(Guard {
                    pos: pos,
                    dir: Dir::Right,
                })
            }
            'v' => {
                return Ok(Guard {
                    pos: pos,
                    dir: Dir::Down,
                })
            }
            '<' => {
                return Ok(Guard {
                    pos: pos,
                    dir: Dir::Left,
                })
            }
            _ => return Err(format!("Invalid direction {}", ch)),
        }
    }

    fn turn(&mut self) {
        match self.dir {
            Dir::Up => self.dir = Dir::Right,
            Dir::Right => self.dir = Dir::Down,
            Dir::Down => self.dir = Dir::Left,
            Dir::Left => self.dir = Dir::Up,
        }
    }
}

struct Board(Vec<Vec<Pixel>>);

impl Board {
    fn get(&self, point: &Point) -> Option<Pixel> {
        if point.0 < (self.0.len() as i32)
            && (point.1 < self.0[point.0 as usize].len() as i32)
            && point.0 >= 0
            && point.1 >= 0
        {
            return Some(self.0[point.0 as usize][point.1 as usize]);
        }
        return None;
    }
}

struct State {
    board: Board,
    guard: Guard,
}

impl State {
    fn read<S: io::Read>(stream: &mut S) -> Result<State, Error> {
        fn parse(i: usize, s: &str, guard: &mut Option<Guard>) -> Result<Vec<Pixel>, String> {
            let mut row = Vec::new();
            for (j, ch) in s.chars().enumerate() {
                match ch {
                    '.' => row.push(Pixel::Empty),
                    '#' => row.push(Pixel::Wall),
                    '^' | '>' | 'v' | '<' => {
                        row.push(Pixel::Empty);
                        guard.replace(Guard::new(i, j, ch).unwrap());
                    }
                    _ => return Err(format!("Unknown character {}", ch)),
                }
            }
            return Ok(row);
        }

        let mut contents = Vec::new();
        let mut guard = None;
        for (i, result) in io::BufReader::new(stream).lines().enumerate() {
            match result {
                Err(error) => return Err(error),
                Ok(line) => contents.push(parse(i, &line, &mut guard).unwrap()),
            }
        }
        return Ok(State {
            board: Board(contents),
            guard: guard.unwrap(),
        });
    }

    fn step(&mut self) -> Option<Point> {
        let point = self.guard.dir.step(&self.guard.pos);
        match self.board.get(&point) {
            None => return None,
            Some(pixel) => match pixel {
                Pixel::Empty => {
                    self.guard.pos = point;
                    return Some(point);
                }
                Pixel::Wall => {
                    self.guard.turn();
                    return self.step();
                }
            },
        }
    }
}

impl ToString for Guard {
    fn to_string(&self) -> String {
        match self.dir {
            Dir::Up => return String::from("^"),
            Dir::Right => return String::from(">"),
            Dir::Down => return String::from("v"),
            Dir::Left => return String::from("<"),
        }
    }
}

impl ToString for Pixel {
    fn to_string(&self) -> String {
        match self {
            Pixel::Empty => return String::from("."),
            Pixel::Wall => return String::from("#"),
        }
    }
}

fn unwords<I: Iterator<Item = String>>(it: I) -> String {
    let words: Vec<String> = it.collect();
    return words.join("\n");
}

impl ToString for Board {
    fn to_string(&self) -> String {
        return unwords(
            self.0
                .iter()
                .map(|row| row.iter().map(|p| p.to_string()).collect::<String>()),
        );
    }
}

fn part1(state: &mut State) -> usize {
    let mut visited: HashSet<Point> = HashSet::new();
    visited.insert(state.guard.pos);
    while let Some(point) = state.step() {
        visited.insert(point);
    }
    return visited.len();
}

fn main() {
    match args().nth(1) {
        None => {
            println!("Specify an input file");
            exit(1);
        }
        Some(path) => {
            let mut file = File::open(&path).unwrap();
            let mut state = State::read(&mut file).unwrap();
            println!("Part 1: {}", part1(&mut state))
        }
    }
}
