use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use std::rc::Rc;

pub struct BagAndNum {
    bag: Rc<str>,
    num: u32,
}
impl BagAndNum {
    fn from_str(s: &str) -> Self {
        let mut iter = s.split(' ');
        let num_str = iter.next().unwrap();
        let mut name_words: Vec<_> = iter.collect();
        name_words.pop();
        Self {
            bag: Rc::from(name_words.join(" ")),
            num: num_str.parse().unwrap(),
        }
    }
}

type BagContains = HashMap<Rc<str>, Vec<BagAndNum>>;
type BagContained = HashMap<Rc<str>, Vec<Rc<str>>>;

#[aoc_generator(day7)]
pub fn generator(input: &str) -> (BagContains, BagContained) {
    let mut contains = HashMap::new();
    let mut contained = HashMap::new();
    for line in input.split('\n') {
        let mut contain_iter = line.split(" contain ");
        let name = contain_iter.next().unwrap();
        let contained_str = contain_iter.next().unwrap();

        let mut name_words: Vec<_> = name.split(' ').collect();
        name_words.pop();
        let name_without_bags: Rc<str> = Rc::from(name_words.join(" "));

        let contained_bags = if contained_str.starts_with("no other") {
            Vec::new()
        } else {
            contained_str.split(", ").map(BagAndNum::from_str).collect()
        };

        for ban in &contained_bags {
            contained
                .entry(ban.bag.clone())
                .or_insert_with(Vec::new)
                .push(name_without_bags.clone());
        }

        contains.insert(name_without_bags, contained_bags);
    }
    (contains, contained)
}

fn find_containing<'a>(query: &str, rules: &'a BagContained) -> Vec<&'a str> {
    rules
        .get(query)
        .map(|contained| {
            let mut list = vec![];
            for s in contained.iter() {
                list.extend(find_containing(&s, rules));
                list.push(s);
            }
            list
        })
        .unwrap_or_else(Vec::new)
}

#[aoc(day7, part1)]
pub fn p1_solver((_, input): &(BagContains, BagContained)) -> usize {
    let set: HashSet<_> = HashSet::from_iter(find_containing(&"shiny gold".to_owned(), input));
    set.len()
}

fn find_contained(query: &str, rules: &BagContains) -> u32 {
    rules
        .get(query)
        .unwrap()
        .iter()
        .map(|ban| (find_contained(&ban.bag, rules) + 1) * ban.num)
        .sum()
}

#[aoc(day7, part2)]
pub fn p2_solver((input, _): &(BagContains, BagContained)) -> u32 {
    find_contained(&"shiny gold".to_owned(), input)
}
