use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
pub struct Password {
    n1: usize,
    n2: usize,
    letter: char,
    password: String,
}

enum ParseState {
    N1,
    N2,
    Letter,
    Password,
}

type Input = Password;

#[aoc_generator(day2)]
pub fn generator(input: &str) -> Vec<Input> {
    input
        .lines()
        .map(|entry| {
            let mut state = ParseState::N1;
            let mut n1 = 0;
            let mut n2 = 0;
            let mut letter = ' ';
            let mut buf = String::new();
            let chars = entry.chars();
            for c in chars {
                match &state {
                    ParseState::N1 => {
                        if c == '-' {
                            state = ParseState::N2;
                            n1 = buf.parse().unwrap();
                            buf.clear();
                        } else {
                            buf.push(c);
                        }
                    }
                    ParseState::N2 => {
                        if c == ' ' {
                            state = ParseState::Letter;
                            n2 = buf.parse().unwrap();
                            buf.clear();
                        } else {
                            buf.push(c);
                        }
                    }
                    ParseState::Letter => {
                        if c == ' ' {
                            state = ParseState::Password;
                        } else if letter == ' ' {
                            letter = c;
                        }
                    }
                    ParseState::Password => {
                        buf.push(c);
                    }
                }
            }
            Password {
                n1,
                n2,
                letter,
                password: buf,
            }
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn p1_solver(input: &[Input]) -> usize {
    input
        .iter()
        .filter(|password| {
            let num_of_chars = password
                .password
                .chars()
                .filter(|c| *c == password.letter)
                .count();
            num_of_chars >= password.n1 && num_of_chars <= password.n2
        })
        .count()
}

#[aoc(day2, part2)]
pub fn p2_solver(input: &[Input]) -> usize {
    input
        .iter()
        .filter(|password| {
            (password.password.chars().nth(password.n1 - 1) == Some(password.letter))
                ^ (password.password.chars().nth(password.n2 - 1) == Some(password.letter))
        })
        .count()
}
