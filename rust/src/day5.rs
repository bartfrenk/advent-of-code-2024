use std::collections::{HashMap, HashSet};
use std::env::args;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::Lines;
use std::process::exit;

type Rule = (u32, u32);
type Update = Vec<u32>;
type Index = HashMap<u32, HashSet<u32>>;

fn read_updates<B: BufRead>(lines: &mut Lines<B>) -> impl Iterator<Item = Update> {
    fn parse_update(s: &str) -> Update {
        s.split(",").map(|s| s.parse().unwrap()).collect()
    }

    lines.map(|res| parse_update(&res.unwrap()))
}

fn read_rules<B: BufRead>(lines: &mut Lines<B>) -> impl Iterator<Item = Rule> {
    fn parse_rule(s: &str) -> Option<Rule> {
        let words: Vec<&str> = s.split("|").collect();
        if words.len() == 2 {
            return Some((str::parse(words[0]).unwrap(), str::parse(words[1]).unwrap()));
        } else {
            return None;
        }
    }

    lines.map_while(|res| parse_rule(&res.unwrap()))
}

fn create_index(rules: impl Iterator<Item = Rule>) -> Index {
    let mut index: Index = HashMap::new();
    for (x, y) in rules {
        index
            .entry(y)
            .and_modify(|s| {
                s.insert(x);
            })
            .or_insert(HashSet::from([x]));
    }
    return index;
}

fn is_correct(index: &Index, update: &Update) -> bool {
    let mut forbidden: HashSet<u32> = HashSet::new();
    for i in update {
        if forbidden.contains(i) {
            return false;
        } else if let Some(s) = index.get(i) {
            forbidden.extend(s);
        }
    }
    return true;
}

fn main() {
    match args().nth(1) {
        None => exit(1),
        Some(path) => {
            let mut lines = io::BufReader::new(File::open(&path).unwrap()).lines();
            let index: Index = create_index(read_rules(&mut lines));
            let mut total = 0;
            for update in read_updates(&mut lines) {
                if is_correct(&index, &update) {
                    total += update[update.len() / 2];
                }
            }
            println!("Part1: {}", total);
        }
    }
}
