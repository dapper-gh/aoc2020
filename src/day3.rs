use aoc_runner_derive::aoc;

const WIDTH: usize = 31;

fn with_slope(right: usize, down: usize, input: &str) -> u64 {
    let mut x = 0;
    let mut y = 0;
    let mut index = 0;
    let mut trees = 0;
    while index < input.len() {
        if input.get(index..(index + 1)).unwrap() == "#" {
            trees += 1;
        }
        x += right;
        y += down;
        index = y * WIDTH + (x % WIDTH) + y;
    }
    trees
}

#[aoc(day3, part1)]
pub fn p1_solver(input: &str) -> u64 {
    with_slope(3, 1, input)
}

#[aoc(day3, part2)]
pub fn p2_solver(input: &str) -> u64 {
    with_slope(1, 1, input)
        * with_slope(3, 1, input)
        * with_slope(5, 1, input)
        * with_slope(7, 1, input)
        * with_slope(1, 2, input)
}
