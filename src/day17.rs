use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;
use std::convert::TryFrom;

type Coordinate = (i64, i64, i64, i64);
type Input = HashMap<Coordinate, bool>;

#[aoc_generator(day17)]
pub fn generator(input: &str) -> Input {
    let mut map = HashMap::new();
    for (y, line) in input.split('\n').enumerate() {
        for (x, c) in line.chars().enumerate() {
            map.insert(
                (i64::try_from(x).unwrap(), i64::try_from(y).unwrap(), 0, 0),
                c == '#',
            );
        }
    }
    map
}

const MOVEMENTS: [(i64, i64, i64, i64); 80] = [
    (-1, -1, -1, -1),
    (-1, -1, -1, 0),
    (-1, -1, -1, 1),
    (-1, -1, 0, -1),
    (-1, -1, 0, 0),
    (-1, -1, 0, 1),
    (-1, -1, 1, -1),
    (-1, -1, 1, 0),
    (-1, -1, 1, 1),
    (-1, 0, -1, -1),
    (-1, 0, -1, 0),
    (-1, 0, -1, 1),
    (-1, 0, 0, -1),
    (-1, 0, 0, 0),
    (-1, 0, 0, 1),
    (-1, 0, 1, -1),
    (-1, 0, 1, 0),
    (-1, 0, 1, 1),
    (-1, 1, -1, -1),
    (-1, 1, -1, 0),
    (-1, 1, -1, 1),
    (-1, 1, 0, -1),
    (-1, 1, 0, 0),
    (-1, 1, 0, 1),
    (-1, 1, 1, -1),
    (-1, 1, 1, 0),
    (-1, 1, 1, 1),
    (0, -1, -1, -1),
    (0, -1, -1, 0),
    (0, -1, -1, 1),
    (0, -1, 0, -1),
    (0, -1, 0, 0),
    (0, -1, 0, 1),
    (0, -1, 1, -1),
    (0, -1, 1, 0),
    (0, -1, 1, 1),
    (0, 0, -1, -1),
    (0, 0, -1, 0),
    (0, 0, -1, 1),
    (0, 0, 0, -1),
    (0, 0, 0, 1),
    (0, 0, 1, -1),
    (0, 0, 1, 0),
    (0, 0, 1, 1),
    (0, 1, -1, -1),
    (0, 1, -1, 0),
    (0, 1, -1, 1),
    (0, 1, 0, -1),
    (0, 1, 0, 0),
    (0, 1, 0, 1),
    (0, 1, 1, -1),
    (0, 1, 1, 0),
    (0, 1, 1, 1),
    (1, -1, -1, -1),
    (1, -1, -1, 0),
    (1, -1, -1, 1),
    (1, -1, 0, -1),
    (1, -1, 0, 0),
    (1, -1, 0, 1),
    (1, -1, 1, -1),
    (1, -1, 1, 0),
    (1, -1, 1, 1),
    (1, 0, -1, -1),
    (1, 0, -1, 0),
    (1, 0, -1, 1),
    (1, 0, 0, -1),
    (1, 0, 0, 0),
    (1, 0, 0, 1),
    (1, 0, 1, -1),
    (1, 0, 1, 0),
    (1, 0, 1, 1),
    (1, 1, -1, -1),
    (1, 1, -1, 0),
    (1, 1, -1, 1),
    (1, 1, 0, -1),
    (1, 1, 0, 0),
    (1, 1, 0, 1),
    (1, 1, 1, -1),
    (1, 1, 1, 0),
    (1, 1, 1, 1),
];

#[aoc(day17, part1)]
pub fn p1_solver(input: &Input) -> usize {
    let mut current_input = input.clone();
    let base_size = current_input
        .keys()
        .map(|(x, y, _, _)| std::cmp::max(x, y).abs())
        .max()
        .unwrap()
        + 1;

    let three_d_movements: Vec<_> = MOVEMENTS.iter().filter(|(_, _, _, w)| *w == 0).collect();

    for size in base_size..(base_size + 6) {
        let mut cloned = current_input.clone();

        for x in (-size)..=size {
            for y in (-size)..=size {
                for z in (-size)..=size {
                    let neighbors = three_d_movements.iter().map(|(mov_x, mov_y, mov_z, _)| {
                        current_input
                            .get(&(x + mov_x, y + mov_y, z + mov_z, 0))
                            .unwrap_or(&false)
                    });
                    let num_active = neighbors.filter(|n| **n).count();
                    match num_active {
                        2 => {}
                        3 => {
                            cloned.insert((x, y, z, 0), true);
                        }
                        _ => {
                            cloned.insert((x, y, z, 0), false);
                        }
                    }
                }
            }
        }

        current_input = cloned;
    }
    current_input.values().filter(|active| **active).count()
}

#[aoc(day17, part2)]
pub fn p2_solver(input: &Input) -> usize {
    let mut current_input = input.clone();
    let base_size = current_input
        .keys()
        .map(|(x, y, _, _)| std::cmp::max(x, y).abs())
        .max()
        .unwrap()
        + 1;

    for size in base_size..(base_size + 6) {
        let mut cloned = current_input.clone();

        for x in (-size)..=size {
            for y in (-size)..=size {
                for z in (-size)..=size {
                    for w in (-size)..=size {
                        let neighbors = MOVEMENTS.iter().map(|(mov_x, mov_y, mov_z, mov_w)| {
                            current_input
                                .get(&(x + mov_x, y + mov_y, z + mov_z, w + mov_w))
                                .unwrap_or(&false)
                        });
                        let num_active = neighbors.filter(|n| **n).count();
                        match num_active {
                            2 => {}
                            3 => {
                                cloned.insert((x, y, z, w), true);
                            }
                            _ => {
                                cloned.insert((x, y, z, w), false);
                            }
                        }
                    }
                }
            }
        }

        current_input = cloned;
    }
    current_input.values().filter(|active| **active).count()
}
