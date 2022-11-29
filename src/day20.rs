use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;
use std::cmp::Ordering;

type Input = HashMap<u64, Vec<Vec<bool>>>;

#[aoc_generator(day20)]
pub fn generator(input: &str) -> Input {
    let mut current_id = 0;
    let mut current_tile = vec![];
    let mut res = HashMap::new();

    for line in input.lines() {
        if line == "" {
            continue;
        }
        if line.starts_with('T') {
            if current_id != 0 {
                res.insert(current_id, current_tile);
            }
            current_id = line.get(5..(line.len() - 1)).unwrap().parse().unwrap();
            current_tile = vec![];
        } else {
            current_tile.push(line.chars().map(|c| c == '#').collect());
        }
    }

    res.insert(current_id, current_tile);

    res
}

fn reverse_if_needed(v: &mut Vec<bool>) {
    for i in 0..v.len() {
        let start: u8 = if *v.get(i).unwrap() { 1 } else { 0 };
        let end: u8 = if *v.get(v.len() - 1 - i).unwrap() {
            1
        } else {
            0
        };
        match start.cmp(&end) {
            Ordering::Greater => return,
            Ordering::Less => {
                v.reverse();
                return;
            },
            Ordering::Equal => ()
        };
    }
}

#[aoc(day20, part1)]
pub fn p1_solver(input: &Input) -> u64 {
    let mut buckets = HashMap::new();

    for (id, tile) in input.iter() {
        let mut top = tile.first().unwrap().clone();
        let mut bottom = tile.last().unwrap().clone();
        let mut left: Vec<_> = tile.iter().map(|row| *row.first().unwrap()).collect();
        let mut right: Vec<_> = tile.iter().map(|row| *row.last().unwrap()).collect();

        reverse_if_needed(&mut top);
        reverse_if_needed(&mut bottom);
        reverse_if_needed(&mut left);
        reverse_if_needed(&mut right);

        buckets.entry(top).or_insert_with(Vec::new).push(*id);
        buckets.entry(bottom).or_insert_with(Vec::new).push(*id);

        buckets.entry(left).or_insert_with(Vec::new).push(*id);
        buckets.entry(right).or_insert_with(Vec::new).push(*id);
    }

    let mut seen = HashMap::new();
    for ids in buckets.values() {
        if ids.len() == 1 {
            continue;
        }
        for id in ids {
            *seen.entry(*id).or_insert(0u8) += 1;
        }
    }

    let once = seen.iter().filter(|(_, num)| **num == 2).map(|(id, _)| *id);

    once.product()
}

#[aoc(day20, part2)]
pub fn p2_solver(_input: &Input) -> usize {
    panic!("Could not find result!");
}
