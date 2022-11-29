use aoc_runner_derive::{aoc, aoc_generator};

pub struct Input {
    first_time: usize,
    buses: Vec<Option<usize>>,
}

#[aoc_generator(day13)]
pub fn generator(input: &str) -> Input {
    let mut lines = input.lines();
    let first_time = lines.next().unwrap().parse().unwrap();
    let buses = lines
        .next()
        .unwrap()
        .split(',')
        .map(|entry| match entry {
            "x" => None,
            _ => Some(entry.parse().unwrap()),
        })
        .collect();
    Input { first_time, buses }
}

#[aoc(day13, part1)]
pub fn p1_solver(input: &Input) -> usize {
    let buses: Vec<_> = input.buses.iter().filter_map(|bus| *bus).collect();
    for time in input.first_time.. {
        if let Some(bus) = buses.iter().find(|bus| time % **bus == 0) {
            return bus * (time - input.first_time);
        }
    }
    panic!("Could not find result!");
}

#[aoc(day13, part2)]
pub fn p2_solver(input: &Input) -> usize {
    let mut buses: Vec<_> = input
        .buses
        .iter()
        .enumerate()
        .filter_map(|(index, opt)| opt.map(|bus| (index, bus)))
        .collect();
    let smallest_index = buses.get(0).unwrap().0;
    buses.sort_unstable_by_key(|(_, opt)| *opt);
    buses.reverse();
    let mut buses_iter = buses.into_iter();

    let first = buses_iter.next().unwrap();
    let second = buses_iter.next().unwrap();

    let mut effective_base = first.1;
    let mut searched_base = second.1;
    let mut searched_remainder = searched_base - second.0;
    let mut current_solution = effective_base - first.0;

    loop {
        current_solution = (0..)
            .map(|n| current_solution + effective_base * n)
            .find(|n| n % searched_base == searched_remainder)
            .unwrap();

        let next = match buses_iter.next() {
            Some(x) => x,
            None => break,
        };
        effective_base *= searched_base;
        searched_base = next.1;
        searched_remainder = (searched_base - (next.0 % searched_base)) % searched_base;
    }
    current_solution + smallest_index
}
