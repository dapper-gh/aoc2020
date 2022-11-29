use aoc_runner_derive::{aoc, aoc_generator};

type Input = u32;

#[aoc_generator(day5)]
pub fn generator(input: &str) -> Vec<Input> {
    input
        .lines()
        .map(|line| {
            let mut num = 0;
            for c in line.chars() {
                let digit = match c {
                    'F' | 'L' => 0,
                    _ => 1,
                };
                num *= 2;
                num += digit;
            }
            num
        })
        .collect()
}

#[aoc(day5, part1)]
pub fn p1_solver(input: &[Input]) -> u32 {
    input.iter().fold(0, |a, b| a.max(*b))
}

#[aoc(day5, part2)]
pub fn p2_solver(input: &[Input]) -> u32 {
    let mut seats = input.to_vec();
    seats.sort_unstable();
    let mut last = 0;
    for seat in seats {
        if last != seat && last != 0 {
            return last;
        }
        last = seat + 1;
    }
    panic!("Could not find result!");
}
