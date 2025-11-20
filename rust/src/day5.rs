use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::env::args;
use std::fs::File;
use std::hash::Hash;
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
        // Ensure that minimal elements also appear as keys in the
        // hash set. This is important for Kahn's algorithm to work.
        index.entry(x).or_insert(HashSet::new());
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

fn part1<B: BufRead>(lines: &mut Lines<B>) -> u32 {
    let index: Index = create_index(read_rules(lines));
    let mut total = 0;
    for update in read_updates(lines) {
        if is_correct(&index, &update) {
            total += update[update.len() / 2];
        }
    }
    return total;
}

trait PopExt {
    type Item;
    fn pop(&mut self) -> Option<Self::Item>;
}

impl<T: Eq + Hash + Copy> PopExt for HashSet<T> {
    type Item = T;

    fn pop(&mut self) -> Option<T> {
        let v = *self.iter().next()?;
        self.remove(&v);
        return Some(v);
    }
}

fn reverse(index: &Index) -> Index {
    let mut reversed: Index = HashMap::new();
    for (x, lt) in index {
        for y in lt {
            reversed
                .entry(*y)
                .and_modify(|s| {
                    s.insert(*x);
                })
                .or_insert(HashSet::from([*x]));
        }
        reversed.entry(*x).or_insert(HashSet::new());
    }
    return reversed;
}

fn restrict_index(index: &Index, v: &Vec<u32>) -> Index {
    let universe: HashSet<u32> = v.iter().copied().collect();
    let mut result: Index = HashMap::new();
    for (x, lt) in index {
        if v.contains(x) {
            let lt_: HashSet<u32> = lt.intersection(&universe).cloned().collect();
            result.insert(*x, lt_);
        }
    }
    return result;
}

fn topological_sort(mut index: Index) -> Vec<u32> {
    let mut pending: HashSet<u32> = HashSet::new();
    let reversed = reverse(&index);
    for (x, lt) in index.iter() {
        if lt.is_empty() {
            pending.insert(*x);
        }
    }

    let mut order: Vec<u32> = Vec::new();

    while let Some(x) = pending.pop() {
        order.push(x);
        if let Some(gt) = reversed.get(&x) {
            for y in gt {
                if let Some(lt) = index.get_mut(y) {
                    lt.remove(&x);
                    if lt.is_empty() {
                        pending.insert(*y);
                    }
                }
            }
        }
    }
    return order;
}

fn from_vec(order: Vec<u32>) -> impl Fn(&u32, &u32) -> Ordering {
    let mut iso: HashMap<u32, usize> = HashMap::with_capacity(order.len());
    for (i, x) in order.into_iter().enumerate() {
        iso.insert(x, i);
    }

    return move |x, y| iso.get(x).unwrap().cmp(&iso.get(y).unwrap());
}

fn from_index(index: Index) -> impl Fn(&u32, &u32) -> Ordering {
    return from_vec(topological_sort(index));
}

fn part2<B: BufRead>(lines: &mut Lines<B>) -> u32 {
    let index: Index = create_index(read_rules(lines));
    let mut total = 0;
    for mut update in read_updates(lines) {
        if !is_correct(&index, &update) {
            // The unrestricted relation represented by `index` contains cycles, and hence
            // Kahn's algorithm fails. In particular this means the relation is not a poset.
            //
            // The restricted relations yield usable comparison functions.
            // Hence, topological sort gives a linear refinement of the
            // restricted relation. The fact that a linear refinement exists, means that
            // the restricted relation was a poset.
            let cmp = from_index(restrict_index(&index, &update));
            update.sort_by(&cmp);
            total += update[update.len() / 2];
        }
    }
    return total;
}

fn main() {
    match args().nth(1) {
        None => exit(1),
        Some(path) => {
            let mut lines = io::BufReader::new(File::open(&path).unwrap()).lines();
            println!("Part1: {}", part1(&mut lines));

            let mut lines = io::BufReader::new(File::open(&path).unwrap()).lines();
            println!("Part2: {}", part2(&mut lines));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn from_rules(rules: impl Iterator<Item = Rule>) -> impl Fn(&u32, &u32) -> Ordering {
        return from_vec(topological_sort(create_index(rules)));
    }

    #[test]
    fn linear_refinement_on_small_vector() {
        let rules = vec![(29, 13), (61, 13), (61, 29)];
        let cmp = from_rules(rules.into_iter());

        let mut v = vec![29, 13, 61];
        v.sort_by(cmp);
        assert_eq!(v, vec![61, 29, 13]);
    }

    #[test]
    fn linear_refinement_on_larger_vector() {
        let rules = vec![
            (97, 13),
            (97, 47),
            (47, 29),
            (75, 29),
            (29, 13),
            (73, 13),
            (97, 29),
            (75, 97),
            (75, 47),
            (61, 13),
            (61, 29),
            (29, 13),
        ];
        let cmp = from_rules(rules.into_iter());

        let mut v = vec![97, 13, 75, 29, 47];
        v.sort_by(cmp);
        assert_eq!(v, vec![75, 97, 47, 29, 13]);
    }

    #[test]
    fn part1_on_small_example() {
        let path = "inputs/day5-small.txt";
        let mut lines = io::BufReader::new(File::open(&path).unwrap()).lines();
        assert_eq!(part1(&mut lines), 143);
    }

    #[test]
    fn part2_on_small_example() {
        let path = "inputs/day5-small.txt";
        let mut lines = io::BufReader::new(File::open(&path).unwrap()).lines();
        assert_eq!(part2(&mut lines), 123);
    }
}
