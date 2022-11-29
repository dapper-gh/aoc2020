use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;
use std::iter::FromIterator;

type Input = Vec<Vec<char>>;

#[aoc_generator(day6)]
pub fn generator(input: &str) -> Vec<Input> {
    input
        .split("\n\n")
        .map(|group| {
            group
                .split('\n')
                .map(|person| person.chars().collect())
                .collect()
        })
        .collect()
}

#[aoc(day6, part1)]
pub fn p1_solver(input: &[Input]) -> usize {
    input
        .iter()
        .map(|group| group.iter().flatten().collect::<HashSet<_>>().len())
        .sum()
}

#[aoc(day6, part2)]
pub fn p2_solver(input: &[Input]) -> usize {
    input
        .iter()
        .map(|group| {
            let mut iter = group
                .iter()
                .map(|person| -> HashSet<&char> { HashSet::from_iter(person.iter()) });
            let mut begin = iter.next().unwrap();
            for set in iter {
                begin.retain(|c| set.contains(c))
            }
            begin.len()
        })
        .sum()
}
