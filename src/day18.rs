use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token {
    Add,
    Mult,
    OpenParen,
    CloseParen,
    Num(u64),
}
type Input = Vec<Token>;

#[aoc_generator(day18)]
pub fn generator(input: &str) -> Vec<Input> {
    input
        .split('\n')
        .map(|line| {
            let mut current_num = 1;
            let mut doing_num = false;

            let mut tokens = vec![];

            for c in line.chars() {
                if let Some(digit) = c.to_digit(10) {
                    current_num *= digit;
                    doing_num = true;
                } else {
                    match c {
                        ' ' if doing_num => {
                            tokens.push(Token::Num(current_num as u64));
                            doing_num = false;
                            current_num = 1;
                        }
                        '+' => {
                            tokens.push(Token::Add);
                        }
                        '*' => {
                            tokens.push(Token::Mult);
                        }
                        '(' => {
                            tokens.push(Token::OpenParen);
                        }
                        ')' => {
                            if doing_num {
                                tokens.push(Token::Num(current_num as u64));
                                doing_num = false;
                                current_num = 1;
                            }
                            tokens.push(Token::CloseParen);
                        }
                        _ => {}
                    };
                }
            }

            if doing_num {
                tokens.push(Token::Num(current_num as u64));
            }

            tokens
        })
        .collect()
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct ExecutionRet {
    num: u64,
    remaining: Vec<Token>,
}

fn execute_tokens_1(tokens: &[Token]) -> ExecutionRet {
    let mut iter = tokens.to_vec().into_iter();

    let mut current_num = match iter.next().unwrap() {
        Token::Num(num) => num,
        Token::OpenParen => {
            let ret = execute_tokens_1(&iter.collect::<Vec<_>>());
            iter = ret.remaining.into_iter();
            ret.num
        }
        _ => panic!("Invalid token at start of expression"),
    };
    let mut operation = Token::Add;

    while let Some(token) = iter.next() {
        match token {
            Token::Num(num) => {
                match operation {
                    Token::Add => current_num += num,
                    Token::Mult => current_num *= num,
                    _ => panic!("Invalid operation!"),
                };
            }
            Token::OpenParen => {
                let ret = execute_tokens_1(&iter.collect::<Vec<_>>());
                iter = ret.remaining.into_iter();
                match operation {
                    Token::Add => current_num += ret.num,
                    Token::Mult => current_num *= ret.num,
                    _ => panic!("Invalid operation!"),
                };
            }
            Token::Add => {
                operation = Token::Add;
            }
            Token::Mult => {
                operation = Token::Mult;
            }
            Token::CloseParen => {
                return ExecutionRet {
                    num: current_num,
                    remaining: iter.collect(),
                };
            }
        }
    }

    ExecutionRet {
        num: current_num,
        remaining: vec![],
    }
}

#[aoc(day18, part1)]
pub fn p1_solver(input: &[Input]) -> u64 {
    input
        .iter()
        .map(|tokens| execute_tokens_1(tokens).num)
        .sum()
}

fn execute_tokens_2(tokens: &[Token]) -> ExecutionRet {
    let mut iter = tokens.to_vec().into_iter().peekable();

    let mut current_num = match iter.next().unwrap() {
        Token::Num(num) => num,
        Token::OpenParen => {
            let ret = execute_tokens_2(&iter.collect::<Vec<_>>());
            iter = ret.remaining.into_iter().peekable();
            ret.num
        }
        _ => panic!("Invalid token at start of expression"),
    };
    let mut operation = Token::Add;
    let mut within_add = None;

    while let Some(token) = iter.next() {
        match token {
            Token::Num(mut num) => {
                let next = iter.peek();
                if Some(&Token::Add) == next {
                    iter.next().unwrap();
                    within_add = Some(if let Some(old_num) = within_add {
                        old_num + num
                    } else {
                        num
                    });
                } else {
                    if let Some(old_num) = within_add {
                        within_add = None;
                        num += old_num;
                    }
                    match operation {
                        Token::Add => current_num += num,
                        Token::Mult => current_num *= num,
                        _ => panic!("Invalid operation!"),
                    };
                }
            }
            Token::OpenParen => {
                let ret = execute_tokens_2(&iter.collect::<Vec<_>>());
                iter = ret.remaining.into_iter().peekable();
                let next = iter.peek();
                let mut num = ret.num;
                if Some(&Token::Add) == next {
                    iter.next().unwrap();
                    within_add = Some(if let Some(old_num) = within_add {
                        old_num + num
                    } else {
                        num
                    });
                } else {
                    if let Some(old_num) = within_add {
                        within_add = None;
                        num += old_num;
                    }
                    match operation {
                        Token::Add => current_num += num,
                        Token::Mult => current_num *= num,
                        _ => panic!("Invalid operation!"),
                    };
                }
            }
            Token::Add => {
                operation = Token::Add;
            }
            Token::Mult => {
                operation = Token::Mult;
            }
            Token::CloseParen => {
                return ExecutionRet {
                    num: current_num,
                    remaining: iter.collect(),
                };
            }
        }
    }

    ExecutionRet {
        num: current_num,
        remaining: vec![],
    }
}

#[aoc(day18, part2)]
pub fn p2_solver(input: &[Input]) -> u64 {
    input
        .iter()
        .map(|tokens| execute_tokens_2(tokens).num)
        .sum()
}
