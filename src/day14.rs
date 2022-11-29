use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

type Input = Instruction;

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum Bit {
    X,
    One,
    Zero,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Instruction {
    Mask(Vec<Bit>),
    Memory(u64, u64),
}

#[aoc_generator(day14)]
pub fn generator(input: &str) -> Vec<Input> {
    input
        .lines()
        .map(|line| match line.get(0..3).unwrap() {
            "mem" => {
                let mut chars = line.chars().skip(4);
                let mut addr = 0;
                while let Some(c) = chars.next() {
                    if let Some(digit) = c.to_digit(10) {
                        addr *= 10;
                        addr += digit as u64;
                    } else {
                        break;
                    }
                }
                Instruction::Memory(addr, chars.skip(3).collect::<String>().parse().unwrap())
            }
            "mas" => Instruction::Mask(
                line.get(7..)
                    .unwrap()
                    .chars()
                    .map(|c| match c {
                        '1' => Bit::One,
                        '0' => Bit::Zero,
                        'X' => Bit::X,
                        _ => panic!("Invalid bitmask!"),
                    })
                    .collect(),
            ),
            _ => panic!("Invalid instruction!"),
        })
        .collect()
}

#[aoc(day14, part1)]
pub fn p1_solver(input: &[Input]) -> u64 {
    let mut bit_mask = vec![];
    let mut memory = HashMap::new();
    for instruction in input.to_vec() {
        match instruction {
            Instruction::Mask(mask) => bit_mask = mask,
            Instruction::Memory(addr, val) => {
                let mut bits: Vec<_> = (0..36).rev().map(|i| (val & 2_u64.pow(i)) >> i).collect();
                for (index, bit) in bit_mask.iter().enumerate() {
                    match bit {
                        Bit::One => {
                            *bits.get_mut(index).unwrap() = 1;
                        }
                        Bit::Zero => {
                            *bits.get_mut(index).unwrap() = 0;
                        }
                        Bit::X => {}
                    }
                }
                memory.insert(addr, bits.into_iter().fold(0, |a, b| a * 2 + b));
            }
        }
    }
    memory.values().sum()
}

fn set_memory(
    index: usize,
    memory: &mut HashMap<u64, u64>,
    floating: &[usize],
    bits: &mut Vec<u64>,
    val: u64,
) {
    if let Some(bit_index) = floating.get(index) {
        *bits.get_mut(*bit_index).unwrap() = 0;
        set_memory(index + 1, memory, floating, bits, val);
        *bits.get_mut(*bit_index).unwrap() = 1;
        set_memory(index + 1, memory, floating, bits, val);
    } else {
        memory.insert(bits.iter().fold(0, |a, b| a * 2 + *b), val);
    }
}

#[aoc(day14, part2)]
pub fn p2_solver(input: &[Input]) -> u64 {
    let mut bit_mask = vec![];
    let mut memory = HashMap::new();
    for instruction in input.to_vec() {
        match instruction {
            Instruction::Mask(mask) => bit_mask = mask,
            Instruction::Memory(addr, val) => {
                let mut bits: Vec<_> = (0..36).rev().map(|i| (addr & 2_u64.pow(i)) >> i).collect();
                let mut floating = vec![];
                for (index, bit) in bit_mask.iter().enumerate() {
                    match bit {
                        Bit::One => {
                            *bits.get_mut(index).unwrap() = 1;
                        }
                        Bit::Zero => {}
                        Bit::X => {
                            floating.push(index);
                        }
                    }
                }
                set_memory(0, &mut memory, &floating, &mut bits, val);
            }
        }
    }
    memory.values().sum()
}
