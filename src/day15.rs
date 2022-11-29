use aoc_runner_derive::{aoc, aoc_generator};

type Input = usize;

#[aoc_generator(day15)]
pub fn generator(input: &str) -> Vec<Input> {
    input.split(',').map(|s| s.parse().unwrap()).collect()
}

fn calc_until(input: &[Input], until: usize) -> usize {
    let mut index = 0;
    let mut last_time = vec![None; until];
    let mut last = 0;

    for num in input {
        let last_time_num = last_time.get_mut(*num).unwrap();
        *last_time_num = Some(index);
        last = *num;
        index += 1;
    }

    while index < until {
        let last_time_last = last_time.get_mut(last).unwrap();
        let num = match last_time_last {
            Some(x) if *x != index - 1 => index - *x - 1,
            _ => 0,
        };
        *last_time_last = Some(index - 1);
        last = num;
        index += 1;
    }

    last
}

#[aoc(day15, part1)]
pub fn p1_solver(input: &[Input]) -> usize {
    calc_until(input, 2020)
}

#[aoc(day15, part2)]
pub fn p2_solver(input: &[Input]) -> usize {
    calc_until(input, 30000000)
}
