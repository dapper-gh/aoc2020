use aoc_runner_derive::{aoc, aoc_generator};
use std::convert::TryFrom;

#[derive(Debug, Copy, Clone)]
pub enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}
impl Instruction {
    fn from_str(input: &str) -> Self {
        let (ins, arg_str) = input.split_at(4);
        let arg = arg_str.parse().unwrap();
        match ins {
            "acc " => Instruction::Acc(arg),
            "jmp " => Instruction::Jmp(arg),
            "nop " => Instruction::Nop(arg),
            _ => panic!("Invalid instruction!"),
        }
    }
    fn flippable(&self) -> bool {
        !matches!(self, Instruction::Acc(_))
    }
    fn flip(&self) -> Self {
        match self {
            Instruction::Acc(a) => Instruction::Acc(*a),
            Instruction::Jmp(a) => Instruction::Nop(*a),
            Instruction::Nop(a) => Instruction::Jmp(*a),
        }
    }
}

type Input = Instruction;

#[aoc_generator(day8)]
pub fn generator(input: &str) -> Vec<Input> {
    input.lines().map(Instruction::from_str).collect()
}

fn execute_instructions(input: &[Instruction]) -> (i32, bool, Vec<usize>) {
    let mut accumulator = 0;
    let mut i = 0;
    let mut visited = Vec::with_capacity(input.len());
    loop {
        if visited.contains(&i) {
            return (accumulator, false, visited);
        }
        visited.push(i);
        match input.get(i) {
            Some(x) => match x {
                Instruction::Acc(arg) => {
                    accumulator += arg;
                    i += 1;
                }
                Instruction::Jmp(arg) => {
                    if arg.is_negative() {
                        i -= usize::try_from(-*arg).unwrap();
                    } else {
                        i += usize::try_from(*arg).unwrap();
                    }
                }
                Instruction::Nop(_) => {
                    i += 1;
                }
            },
            None => return (accumulator, true, visited),
        }
    }
}

#[aoc(day8, part1)]
pub fn p1_solver(input: &[Input]) -> i32 {
    execute_instructions(input).0
}

#[aoc(day8, part2)]
pub fn p2_solver(input: &[Input]) -> i32 {
    let mut input = input.to_vec();
    for i in execute_instructions(&input).2 {
        let item = *input.get(i).unwrap();
        if item.flippable() {
            input[i] = item.flip();
            let (acc, natural, _) = execute_instructions(&input);
            if natural {
                return acc;
            }
            input[i] = item;
        }
    }
    panic!("Could not find result!");
}
