use rayon::prelude::*;
use std::collections::HashSet;
use std::env::args;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::process::exit;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct Point(usize, usize);

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

use Dir::*;

impl Dir {
    const VALUES: [Dir; 4] = [Up, Right, Down, Left];

    fn parse(c: char) -> Option<Dir> {
        for dir in Dir::VALUES {
            if c == char::from(&dir) {
                return Some(dir);
            }
        }
        return None;
    }

    fn step(&self, p: &Point) -> Option<Point> {
        match self {
            Up => {
                if p.0 > 0 {
                    return Some(Point(p.0 - 1, p.1));
                } else {
                    return None;
                }
            }
            Right => return Some(Point(p.0, p.1 + 1)),
            Down => return Some(Point(p.0 + 1, p.1)),
            Left => {
                if p.1 > 0 {
                    return Some(Point(p.0, p.1 - 1));
                } else {
                    return None;
                }
            }
        }
    }

    fn turn(&self) -> Dir {
        match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }
}

impl From<&Dir> for char {
    fn from(dir: &Dir) -> Self {
        match dir {
            Up => return '^',
            Right => return '>',
            Left => return '<',
            Down => return 'V',
        }
    }
}

impl ToString for Dir {
    fn to_string(&self) -> String {
        return char::from(self).into();
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct Guard {
    pos: Point,
    dir: Dir,
}

impl Guard {
    fn turn(&mut self) {
        self.dir = self.dir.turn();
    }
}

#[derive(Clone)]
struct Board {
    height: usize,
    width: usize,
    obstacles: HashSet<Point>,
}

struct IterState<'a> {
    board: &'a Board,
    guard: Option<Guard>,
}

impl Iterator for IterState<'_> {
    type Item = Guard;

    fn next(&mut self) -> Option<Self::Item> {
        let guard = self.guard;
        self.board.step(&mut self.guard);
        return guard;
    }
}

impl Board {
    fn step(&self, guard: &mut Option<Guard>) {
        if let Some(g) = guard {
            if let Some(p) = g.dir.step(&g.pos) {
                if !self.contains(&p) {
                    guard.take();
                } else {
                    if self.obstacles.contains(&p) {
                        g.turn();
                        self.step(guard);
                    } else {
                        g.pos = p;
                    }
                }
            } else {
                guard.take();
            }
        }
    }

    fn contains(&self, p: &Point) -> bool {
        return p.0 < self.height && p.1 < self.width;
    }

    fn walk(&self, guard: Option<Guard>) -> impl Iterator<Item = Guard> {
        return IterState {
            board: self,
            guard: guard,
        };
    }
}

struct Instance {
    board: Board,
    guard: Option<Guard>,
}

impl Instance {
    fn read<S: io::Read>(stream: &mut S) -> Result<Self, io::Error> {
        let mut obstacles = HashSet::new();
        let mut height = 0;
        let mut width = 0;
        let mut guard = None;
        for (i, res) in io::BufReader::new(stream).lines().enumerate() {
            let line = res?;
            for (j, c) in line.chars().enumerate() {
                match c {
                    '.' => continue,
                    '#' => {
                        obstacles.insert(Point(i, j));
                    }
                    _ => {
                        if let Some(dir) = Dir::parse(c) {
                            guard.replace(Guard {
                                pos: Point(i, j),
                                dir: dir,
                            });
                        };
                    }
                }
            }
            height += 1;
            width = line.len();
        }
        return Ok(Instance {
            board: Board {
                height: height,
                width: width,
                obstacles: obstacles,
            },
            guard: guard,
        });
    }
}

fn part1(path: &str) -> usize {
    let mut file = File::open(&path).unwrap();
    let instance = Instance::read(&mut file).unwrap();
    let mut visited = HashSet::new();
    for guard in instance.board.walk(instance.guard) {
        visited.insert(guard.pos);
    }
    return visited.len();
}

fn pred(board: &mut Board, point: Point, guard: Option<Guard>) -> bool {
    board.obstacles.insert(point);
    let mut history = HashSet::new();
    for guard in board.walk(guard) {
        if history.contains(&guard) {
            return true;
        }
        history.insert(guard);
    }
    return false;
}

fn part2(path: &str) -> usize {
    let mut file = File::open(&path).unwrap();
    let instance = Instance::read(&mut file).unwrap();

    let mut visited = HashSet::new();
    for guard in instance.board.walk(instance.guard) {
        visited.insert(guard.pos);
    }

    return visited
        .par_iter()
        .filter(|p| pred(&mut instance.board.clone(), **p, instance.guard))
        .count();
}

fn main() {
    match args().nth(1) {
        None => {
            println!("Specify an input file");
            exit(1);
        }
        Some(path) => {
            println!("Part 1: {}", part1(&path));
            println!("Part 2: {}", part2(&path))
        }
    }
}
