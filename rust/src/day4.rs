use std::env::args;
use std::fs::File;
use std::io;
use std::io::{BufRead, Error};

use itertools::iproduct;
use std::process::exit;

type Index = i32;

#[derive(Copy, Clone)]
struct Point(Index, Index);

impl Point {
    fn add(&mut self, dir: &Point) {
        self.0 = self.0 + dir.0;
        self.1 = self.1 + dir.1;
    }

    fn origin(&self) -> bool {
        return self.0 == 0 && self.1 == 0;
    }
}

struct Cursor<'a> {
    board: &'a Board,
    current: Option<Point>,
}

impl<'a> Iterator for Cursor<'a> {
    type Item = Point;

    fn next(&mut self) -> Option<Point> {
        if let Some(p) = self.current.take() {
            if (p.1 as usize) < self.board.0[0].len() - 1 {
                self.current = Some(Point(p.0, p.1 + 1));
            } else if (p.0 as usize) < self.board.0.len() - 1 {
                self.current = Some(Point(p.0 + 1, 0));
            } else {
                self.current = None;
            }
            return Some(p);
        } else {
            return None;
        }
    }
}

struct Board(Vec<Vec<u8>>);

impl Board {
    fn read<S: io::Read>(stream: &mut S) -> Result<Board, Error> {
        let mut contents: Vec<Vec<u8>> = Vec::new();
        for result in io::BufReader::new(stream).lines() {
            match result {
                Err(error) => return Err(error),
                Ok(line) => contents.push(Vec::from(line.as_bytes())),
            }
        }
        return Ok(Board(contents));
    }

    #[allow(dead_code)]
    fn from(xs: &[&[u8]]) -> Board {
        Board(xs.iter().map(|row| row.to_vec()).collect())
    }

    fn get(&self, p: &Point) -> Option<u8> {
        if p.0 < 0 || p.1 < 0 {
            return None;
        }
        if self.0.len() <= (p.0 as usize) || self.0[0].len() <= (p.1 as usize) {
            return None;
        }
        return Some(self.0[p.0 as usize][p.1 as usize]);
    }

    fn positions(&self) -> impl Iterator<Item = Point> {
        return Cursor {
            board: &self,
            current: Some(Point(0, 0)),
        };
    }
}

fn has_word(board: &Board, init: &Point, dir: &Point, word: &[u8]) -> bool {
    let mut cursor: Point = *init;
    for &c in word {
        match board.get(&cursor) {
            None => return false,
            Some(b) => {
                if c != b {
                    return false;
                }
            }
        }
        cursor.add(dir);
    }
    return true;
}

fn part1(board: &Board, word: &[u8]) -> usize {
    let mut total = 0;
    let dirs: Vec<Point> = iproduct!([-1, 0, 1], [-1, 0, 1])
        .map(|(i, j)| Point(i, j))
        .filter(|p| !p.origin())
        .collect();
    for p in board.positions() {
        for dir in dirs.iter() {
            if has_word(&board, &p, dir, word) {
                total += 1;
            }
        }
    }

    return total;
}

fn main() {
    match args().nth(1) {
        None => exit(1),
        Some(path) => {
            let mut file = File::open(&path).unwrap();
            if let Ok(board) = Board::read(&mut file) {
                println!("Part1: {}", part1(&board, "XMAS".as_bytes()));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_example_part1() {
        let board = Board::from(&[
            b"MMMSXXMASM",
            b"MSAMXMSMSA",
            b"AMXSXMAAMM",
            b"MSAMASMSMX",
            b"XMASAMXAMM",
            b"XXAMMXXAMA",
            b"SMSMSASXSS",
            b"SAXAMASAAA",
            b"MAMMMXMMMM",
            b"MXMXAXMASX",
        ]);
        assert_eq!(part1(&board, b"XMAS"), 18);
    }

    #[test]
    fn test_part1_on_input_data() {
        let mut file = File::open("inputs/day4.txt").unwrap();
        let board = Board::read(&mut file).unwrap();
        assert_eq!(part1(&board, b"XMAS"), 2336);
    }
}
