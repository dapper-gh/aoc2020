use aoc_runner_derive::{aoc, aoc_generator};

type Input = u64;

#[aoc_generator(day10)]
pub fn generator(input: &str) -> Vec<Input> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[aoc(day10, part1)]
pub fn p1_solver(input_slice: &[u64]) -> usize {
    let mut input = input_slice.to_vec();
    input.push(0);
    input.sort_unstable();
    input.push(input.last().unwrap() + 3);
    let threes = input
        .windows(2)
        .filter(|n| (n.get(1).unwrap() - n.get(0).unwrap()) == 3)
        .count();
    threes * (input.len() - threes)
}

fn arrangements(input: &[u64], start: u64, last: u64) -> u64 {
    if start == last {
        1
    } else {
        input
            .iter()
            .enumerate()
            .filter(|(_, n)| **n > start && (start + 4) > **n && **n <= last)
            .map(|(i, n)| {
                if *n == last {
                    1
                } else {
                    arrangements(input.get(i..).unwrap(), *n, last)
                }
            })
            .sum()
    }
}

#[aoc(day10, part2)]
pub fn p2_solver(input_slice: &[u64]) -> u64 {
    let mut input = input_slice.to_vec();
    input.push(0);
    input.sort_unstable();
    let last = input.last().unwrap() + 3;
    input.push(last);
    let threes: Vec<(usize, (u64, u64))> = input
        .windows(2)
        .map(|n| (*n.get(0).unwrap(), *n.get(1).unwrap()))
        .enumerate()
        .filter(|(_, (start, end))| (end - start) == 3)
        .collect();
    let mut total_arrangements = 1;
    let mut start_at = 0;
    for (index, (start_of_three, end_of_three)) in threes {
        total_arrangements *=
            arrangements(input.get(..(index + 1)).unwrap(), start_at, start_of_three);
        start_at = end_of_three;
    }
    total_arrangements
}
