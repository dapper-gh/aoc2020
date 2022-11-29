use aoc_runner_derive::{aoc, aoc_generator};
use std::cmp::Ordering;

#[aoc_generator(day1)]
pub fn generator(input: &str) -> Vec<u64> {
    input.lines().map(|n| n.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn p1_solver(input: &[u64]) -> u64 {
    let mut input: Vec<_> = input.to_vec();
    input.sort_unstable();
    for i in &input {
        for j in &input {
            match (j + i).cmp(&2020) {
                Ordering::Greater => break,
                Ordering::Equal => return j * i,
                _ => {}
            }
        }
    }
    panic!("Could not find result!");
}

#[aoc(day1, part2)]
pub fn p2_solver(input: &[u64]) -> u64 {
    let mut input: Vec<_> = input.to_vec();
    input.sort_unstable();
    for i in &input {
        for j in &input {
            if (i + j) > 2020 {
                break;
            }
            for k in &input {
                match (i + j + k).cmp(&2020) {
                    Ordering::Greater => {
                        break;
                    }
                    Ordering::Equal => {
                        return i * j * k;
                    }
                    _ => {}
                }
            }
        }
    }
    panic!("Could not find result!");
}
