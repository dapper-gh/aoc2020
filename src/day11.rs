use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Cell {
    Occupied,
    Empty,
    Floor,
}

#[aoc_generator(day11)]
pub fn generator(input: &str) -> Vec<Vec<Cell>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'L' => Cell::Empty,
                    _ => Cell::Floor,
                })
                .collect()
        })
        .collect()
}

#[aoc(day11, part1)]
pub fn p1_solver(input_slice: &[Vec<Cell>]) -> usize {
    let mut input = input_slice.to_vec();
    let default_vec = vec![];
    let mut did_op = true;
    while did_op {
        did_op = false;
        let old_input = input.clone();
        for (y, line) in old_input.iter().enumerate() {
            for (x, cell) in line.iter().enumerate() {
                let above = old_input.get(y - 1).unwrap_or(&default_vec);
                let below = old_input.get(y + 1).unwrap_or(&default_vec);
                let adjacent = [
                    above.get(x - 1).unwrap_or(&Cell::Floor),
                    above.get(x).unwrap_or(&Cell::Floor),
                    above.get(x + 1).unwrap_or(&Cell::Floor),
                    line.get(x - 1).unwrap_or(&Cell::Floor),
                    line.get(x + 1).unwrap_or(&Cell::Floor),
                    below.get(x - 1).unwrap_or(&Cell::Floor),
                    below.get(x).unwrap_or(&Cell::Floor),
                    below.get(x + 1).unwrap_or(&Cell::Floor),
                ];
                let occupied = adjacent.iter().filter(|c| ***c == Cell::Occupied).count();
                if *cell == Cell::Empty && occupied == 0 {
                    did_op = true;
                    *input.get_mut(y).unwrap().get_mut(x).unwrap() = Cell::Occupied;
                } else if *cell == Cell::Occupied && occupied >= 4 {
                    did_op = true;
                    *input.get_mut(y).unwrap().get_mut(x).unwrap() = Cell::Empty;
                }
            }
        }
    }
    input
        .into_iter()
        .flatten()
        .filter(|c| *c == Cell::Occupied)
        .count()
}

enum Step {
    Subtract,
    Nothing,
    Add,
}

const DIRECTIONS: [(Step, Step); 8] = [
    (Step::Add, Step::Add),
    (Step::Add, Step::Nothing),
    (Step::Add, Step::Subtract),
    (Step::Nothing, Step::Add),
    (Step::Nothing, Step::Subtract),
    (Step::Subtract, Step::Add),
    (Step::Subtract, Step::Nothing),
    (Step::Subtract, Step::Subtract),
];

#[aoc(day11, part2)]
pub fn p2_solver(input_slice: &[Vec<Cell>]) -> usize {
    let mut input = input_slice.to_vec();
    let default_vec = vec![];
    let mut did_op = true;
    while did_op {
        did_op = false;
        let old_input = input.clone();
        for (y, line) in old_input.iter().enumerate() {
            for (x, cell) in line.iter().enumerate() {
                if *cell == Cell::Floor {
                    continue;
                }
                let mut seats = DIRECTIONS.iter().map(|(dir_y, dir_x)| {
                    let mut cur_y = y;
                    let mut cur_x = x;
                    match dir_y {
                        Step::Subtract => cur_y -= 1,
                        Step::Nothing => {}
                        Step::Add => cur_y += 1,
                    };
                    match dir_x {
                        Step::Subtract => cur_x -= 1,
                        Step::Nothing => {}
                        Step::Add => cur_x += 1,
                    };
                    let mut cur_cell = old_input.get(cur_y).unwrap_or(&default_vec).get(cur_x);
                    while Some(&Cell::Floor) == cur_cell {
                        match dir_y {
                            Step::Subtract => cur_y -= 1,
                            Step::Nothing => {}
                            Step::Add => cur_y += 1,
                        };
                        match dir_x {
                            Step::Subtract => cur_x -= 1,
                            Step::Nothing => {}
                            Step::Add => cur_x += 1,
                        };
                        cur_cell = old_input.get(cur_y).unwrap_or(&default_vec).get(cur_x);
                    }
                    cur_cell.unwrap_or(&Cell::Floor)
                });
                if *cell == Cell::Empty && !seats.by_ref().any(|c| *c == Cell::Occupied) {
                    did_op = true;
                    *input.get_mut(y).unwrap().get_mut(x).unwrap() = Cell::Occupied;
                } else if *cell == Cell::Occupied
                    && seats.filter(|c| **c == Cell::Occupied).count() >= 5
                {
                    did_op = true;
                    *input.get_mut(y).unwrap().get_mut(x).unwrap() = Cell::Empty;
                }
            }
        }
    }
    input
        .into_iter()
        .flatten()
        .filter(|c| *c == Cell::Occupied)
        .count()
}
