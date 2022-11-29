use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::VecDeque;

type Input = u64;

#[aoc_generator(day9)]
pub fn generator(input: &str) -> Vec<Input> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[aoc(day9, part1)]
pub fn p1_solver(input: &[u64]) -> u64 {
    let mut previous = VecDeque::with_capacity(25);
    for num in input {
        if previous.len() >= 25 {
            let mut valid = false;
            'outer: for i in &previous {
                for j in &previous {
                    if i + j == *num {
                        valid = true;
                        break 'outer;
                    }
                }
            }
            if !valid {
                return *num;
            }
            previous.pop_front();
        }
        previous.push_back(*num);
    }
    panic!("Could not find result!");
}

#[aoc(day9, part2)]
pub fn p2_solver(input: &[u64]) -> u64 {
    let from_before = p1_solver(input);
    for start in 0..input.len() {
        let mut i = start;
        let mut smallest = *input.get(i).unwrap();
        let mut largest = smallest;
        let mut sum = smallest;
        if sum == from_before {
            continue;
        }
        while sum < from_before {
            i += 1;
            let current = match input.get(i) {
                Some(n) => *n,
                None => break,
            };
            if current < smallest {
                smallest = current;
            } else if current > largest {
                largest = current;
            }
            sum += current;
        }
        if sum == from_before {
            return smallest + largest;
        }
    }
    panic!("Could not find result!");
}
