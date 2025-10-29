use itertools::Itertools;
use std::fs::File;
use std::io;
use std::io::BufRead;

type Report = Vec<i64>;

fn parse(s: &str) -> Report {
    s.split(" ")
        .filter(|x| !x.is_empty())
        .map(|s| str::parse::<i64>(s).unwrap())
        .collect()
}

fn read(stream: impl io::Read) -> impl Iterator<Item = Report> {
    let reader = io::BufReader::new(stream);
    reader.lines().map(|res| parse(&res.unwrap()))
}

fn has_strict_bounded_incs<'a>(it: impl Iterator<Item = &'a i64>) -> bool {
    for (x, y) in it.tuple_windows() {
        if !(*x < *y && *y <= *x + 3) {
            return false;
        }
    }
    true
}

fn is_safe(report: &Report) -> bool {
    has_strict_bounded_incs(report.iter()) || has_strict_bounded_incs(report.iter().rev())
}

fn skip_over(i: usize, report: &Report) -> Report {
    report
        .iter()
        .take(i)
        .chain(report.iter().skip(i + 1))
        .copied()
        .collect()
}

fn is_nearly_safe(report: &Report) -> bool {
    if is_safe(report) {
        return true;
    }
    for i in 0..report.len() {
        let skipped = skip_over(i, report);
        if is_safe(&skipped) {
            return true;
        }
    }
    false
}

fn part1(reports: impl Iterator<Item = Report>) -> usize {
    reports.filter(is_safe).count()
}

fn part2(reports: impl Iterator<Item = Report>) -> usize {
    reports.filter(is_nearly_safe).count()
}

pub fn run() {
    let path = "inputs/day2.txt";
    println!("Day 2");

    let reports: Vec<Report> = read(File::open(path).unwrap()).collect();
    println!("\tPart 1: {}", part1(reports.into_iter()));

    let reports: Vec<Report> = read(File::open(path).unwrap()).collect();
    println!("\tPart 2: {}", part2(reports.into_iter()));
}

#[cfg(test)]
mod tests {
    use crate::solutions::day2::*;

    #[test]
    fn test_small_example_part1() {
        let reports = [
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ];
        assert!(part1(reports.into_iter()) == 2);
    }

    #[test]
    fn test_part1_on_input_data() {
        let reports = read(File::open("inputs/day2.txt").unwrap());
        assert!(part1(reports) == 390);
    }
}
