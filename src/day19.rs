use aoc_runner_derive::{aoc, aoc_generator};

type Rule = Vec<Vec<Token>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Input {
    messages: Vec<String>,
    rules: Vec<Rule>,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum Token {
    Char(char),
    Rule(usize),
}

#[aoc_generator(day19)]
pub fn generator(input: &str) -> Input {
    let mut done_with_rules = false;
    let mut rules = vec![vec![]; 1000];
    let mut messages = vec![];

    let mut max_rule_num = 0;
    for line in input.lines() {
        if done_with_rules {
            messages.push(line.to_owned());
        } else {
            if line == "" {
                done_with_rules = true;
                continue;
            }
            let mut parts = line.split(':');
            let rule_num: usize = parts.next().unwrap().parse().unwrap();
            if max_rule_num < rule_num {
                max_rule_num = rule_num;
            }
            let rule = parts.next().unwrap().get(1..).unwrap();
            let mut matches: Vec<Vec<_>> = rule
                .split(" | ")
                .map(|part| {
                    part.split(' ')
                        .map(|token| {
                            let mut chars = token.chars();
                            if chars.next().unwrap() == '"' {
                                Token::Char(chars.next().unwrap())
                            } else {
                                Token::Rule(token.parse().unwrap())
                            }
                        })
                        .collect()
                })
                .collect();
            rules.get_mut(rule_num).unwrap().append(&mut matches);
        }
    }

    Input { rules, messages }
}

fn compile_rules(rules: &[Rule], rule: usize) -> Vec<String> {
    let mut all = vec![];
    for opt in rules.get(rule).unwrap() {
        let mut res = vec![String::new()];
        for token in opt {
            match token {
                Token::Char(c) => {
                    for possible in &mut res {
                        possible.push(*c);
                    }
                }
                Token::Rule(index) => {
                    let new_possible = compile_rules(rules, *index);
                    res = res
                        .into_iter()
                        .map(|p| new_possible.iter().map(move |s| format!("{}{}", p, s)))
                        .flatten()
                        .collect();
                }
            }
        }
        all.append(&mut res);
    }

    all
}

#[aoc(day19, part1)]
pub fn p1_solver(input: &Input) -> usize {
    let compiled = compile_rules(&input.rules, 0);
    input
        .messages
        .iter()
        .filter(|msg| compiled.contains(msg))
        .count()
}

fn process_message(msg: &str, possible: &[String]) -> Vec<(usize, usize)> {
    let mut ends: Vec<_> = possible
        .iter()
        .map(|pat| {
            if msg.starts_with(pat) {
                match msg.get(pat.len()..) {
                    Some(sub) => process_message(sub, possible)
                        .into_iter()
                        .map(|(a, b)| (a + pat.len(), b + 1))
                        .collect(),
                    None => vec![],
                }
            } else {
                vec![]
            }
        })
        .flatten()
        .collect();
    ends.push((0, 0));
    ends
}

#[aoc(day19, part2)]
pub fn p2_solver(input: &Input) -> usize {
    let rules = input.rules.to_vec();

    let rule_42 = compile_rules(&rules, 42);
    let rule_31 = compile_rules(&rules, 31);

    input
        .messages
        .iter()
        .filter(|msg| {
            for (possible_1, num_1) in process_message(msg, &rule_42) {
                if num_1 == 0 {
                    continue;
                }
                let sub = match msg.get(possible_1..) {
                    Some(x) => x,
                    None => continue,
                };
                for (possible_2, num_2) in process_message(sub, &rule_31) {
                    if num_2 == 0 {
                        continue;
                    }
                    if possible_1 + possible_2 == msg.len() && num_2 < num_1 {
                        return true;
                    }
                }
            }
            false
        })
        .count()
}
