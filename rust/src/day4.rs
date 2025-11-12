use std::env::args;
use std::fs::File;
use std::io;
use std::io::{BufRead, Error};

use itertools::iproduct;
use std::process::exit;

type Board = Vec<Vec<u8>>;

fn read<S: io::Read>(stream: &mut S) -> Result<Board, Error> {
    let mut board: Board = Vec::new();
    for result in io::BufReader::new(stream).lines() {
        match result {
            Err(error) => return Err(error),
            Ok(line) => board.push(Vec::from(line.as_bytes())),
        }
    }
    return Ok(board);
}

fn has_word(board: &Board, pos: (usize, usize), dir: (i8, i8), word: &[u8]) -> bool {
    let mut x: i64 = pos.0 as i64;
    let mut y: i64 = pos.1 as i64;
    for k in 1..word.len() {
        x += dir.0 as i64;
        y += dir.1 as i64;
        if x < 0 || y < 0 || x >= (board.len() as i64) || y >= (board[0].len() as i64) {
            return false;
        } else if word[k] != board[x as usize][y as usize] {
            return false;
        }
    }
    return true;
}

fn directions() -> impl Iterator<Item = (i8, i8)> {
    iproduct!([-1, 0, 1], [-1, 0, 1]).filter(|(i, j)| *i != 0 || *j != 0)
}

fn part1(board: &Board, word: &[u8]) -> usize {
    let mut total = 0;
    for r in 0..board.len() {
        for c in 0..board[r].len() {
            if board[r][c] == word[0] {
                for (i, j) in directions() {
                    if has_word(&board, (r, c), (i, j), word) {
                        total += 1;
                    }
                }
            }
        }
    }
    return total;
}

fn main() {
    match args().skip(1).next() {
        None => exit(1),
        Some(path) => {
            let mut file = File::open(&path).unwrap();
            if let Ok(board) = read(&mut file) {
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
        let board = vec![
            Vec::from(b"MMMSXXMASM"),
            Vec::from(b"MSAMXMSMSA"),
            Vec::from(b"AMXSXMAAMM"),
            Vec::from(b"MSAMASMSMX"),
            Vec::from(b"XMASAMXAMM"),
            Vec::from(b"XXAMMXXAMA"),
            Vec::from(b"SMSMSASXSS"),
            Vec::from(b"SAXAMASAAA"),
            Vec::from(b"MAMMMXMMMM"),
            Vec::from(b"MXMXAXMASX"),
        ];
        assert!(part1(&board, b"XMAS") == 18);
    }

    #[test]
    fn test_part1_on_input_data() {
        let mut file = File::open("inputs/day4.txt").unwrap();
        let board = read(&mut file).unwrap();
        assert_eq!(part1(&board, b"XMAS"), 2335);
    }
}
