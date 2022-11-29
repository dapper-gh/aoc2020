use aoc_runner_derive::{aoc, aoc_generator};

pub enum Instruction {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Left(i32),
    Right(i32),
    Forward(i32),
}
impl Instruction {
    pub fn from_str(s: &str) -> Self {
        let arg = s.get(1..).unwrap().parse().unwrap();
        match s.get(0..1).unwrap() {
            "N" => Instruction::North(arg),
            "S" => Instruction::South(arg),
            "E" => Instruction::East(arg),
            "W" => Instruction::West(arg),
            "L" => Instruction::Left(arg),
            "R" => Instruction::Right(arg),
            "F" => Instruction::Forward(arg),
            _ => panic!(),
        }
    }
}

type Input = Instruction;

#[aoc_generator(day12)]
pub fn generator(input: &str) -> Vec<Input> {
    input.lines().map(Instruction::from_str).collect()
}

#[aoc(day12, part1)]
pub fn p1_solver(input: &[Input]) -> i32 {
    let mut facing = 0;
    let mut x = 0;
    let mut y = 0;
    for instruction in input {
        match instruction {
            Instruction::North(dist) => y += dist,
            Instruction::South(dist) => y -= dist,
            Instruction::East(dist) => x += dist,
            Instruction::West(dist) => x -= dist,
            Instruction::Left(deg) => {
                facing += deg;
                facing %= 360;
            }
            Instruction::Right(deg) => {
                facing -= deg;
                facing %= 360;
                facing += 360;
                facing %= 360;
            }
            Instruction::Forward(dist) => match facing {
                0 => x += dist,
                90 => y += dist,
                180 => x -= dist,
                270 => y -= dist,
                _ => panic!("Facing value not a multiple of 90"),
            },
        }
    }
    x.abs() + y.abs()
}

#[aoc(day12, part2)]
pub fn p2_solver(input: &[Input]) -> i32 {
    let mut way_x = 10;
    let mut way_y = 1;
    let mut ship_x = 0;
    let mut ship_y = 0;
    for instruction in input {
        match instruction {
            Instruction::North(dist) => way_y += dist,
            Instruction::South(dist) => way_y -= dist,
            Instruction::East(dist) => way_x += dist,
            Instruction::West(dist) => way_x -= dist,
            Instruction::Left(dist) => {
                let (flip, transform_x, transform_y) = match dist {
                    0 => (false, 1, 1),
                    90 => (true, -1, 1),
                    180 => (false, -1, -1),
                    270 => (true, 1, -1),
                    _ => panic!("L instruction did not have argument that was multiple of 90"),
                };
                let mut relative_x = way_x - ship_x;
                let mut relative_y = way_y - ship_y;
                if flip {
                    std::mem::swap(&mut relative_x, &mut relative_y);
                }
                relative_x *= transform_x;
                relative_y *= transform_y;
                way_x = relative_x + ship_x;
                way_y = relative_y + ship_y;
            }
            Instruction::Right(dist) => {
                let (flip, transform_x, transform_y) = match dist {
                    0 => (false, 1, 1),
                    270 => (true, -1, 1),
                    180 => (false, -1, -1),
                    90 => (true, 1, -1),
                    _ => panic!("R instruction did not have argument that was multiple of 90"),
                };
                let mut relative_x = way_x - ship_x;
                let mut relative_y = way_y - ship_y;
                if flip {
                    std::mem::swap(&mut relative_x, &mut relative_y);
                }
                relative_x *= transform_x;
                relative_y *= transform_y;
                way_x = relative_x + ship_x;
                way_y = relative_y + ship_y;
            }
            Instruction::Forward(dist) => {
                let move_x = way_x - ship_x;
                let move_y = way_y - ship_y;
                for _ in 0..*dist {
                    way_x += move_x;
                    ship_x += move_x;
                    way_y += move_y;
                    ship_y += move_y;
                }
            }
        }
    }
    ship_x.abs() + ship_y.abs()
}
