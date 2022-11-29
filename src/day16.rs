use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;
use std::ops::RangeInclusive;

type Ticket = Vec<u64>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Input {
    fields: HashMap<String, (RangeInclusive<u64>, RangeInclusive<u64>)>,
    my_ticket: Ticket,
    other_tickets: Vec<Ticket>,
}

enum ParseState {
    Fields,
    Mine,
    Other,
}

#[aoc_generator(day16)]
pub fn generator(input: &str) -> Input {
    let mut fields = HashMap::new();
    let mut my_ticket = vec![];
    let mut other_tickets = vec![];

    let mut parse_state = ParseState::Fields;

    for line in input.lines() {
        if line == "" {
            continue;
        }
        match parse_state {
            ParseState::Fields => {
                if line == "your ticket:" {
                    parse_state = ParseState::Mine;
                    continue;
                }
                let mut parts = line.split(": ");
                let name = parts.next().unwrap();
                let rest = parts.next().unwrap();
                let mut rules = rest.split(" or ").map(|s| {
                    let mut nums = s.split('-');
                    (nums.next().unwrap().parse().unwrap())
                        ..=(nums.next().unwrap().parse().unwrap())
                });
                fields.insert(
                    name.to_owned(),
                    (rules.next().unwrap(), rules.next().unwrap()),
                );
            }
            ParseState::Mine => {
                if line == "nearby tickets:" {
                    parse_state = ParseState::Other;
                    continue;
                }
                my_ticket = line.split(',').map(|s| s.parse().unwrap()).collect();
            }
            ParseState::Other => {
                other_tickets.push(line.split(',').map(|s| s.parse().unwrap()).collect());
            }
        }
    }

    Input {
        fields,
        my_ticket,
        other_tickets,
    }
}

#[aoc(day16, part1)]
pub fn p1_solver(input: &Input) -> u64 {
    let rules: Vec<_> = input.fields.values().collect();

    input
        .other_tickets
        .iter()
        .map(|ticket| -> u64 {
            ticket
                .iter()
                .filter(|n| {
                    !rules
                        .iter()
                        .any(|(range1, range2)| range1.contains(n) || range2.contains(n))
                })
                .sum()
        })
        .sum()
}

#[aoc(day16, part2)]
pub fn p2_solver(input: &Input) -> u64 {
    let rules: Vec<_> = input.fields.iter().collect();

    let valid_tickets = input.other_tickets.iter().filter(|ticket: &&Ticket| {
        !ticket.iter().any(|n| {
            !rules
                .iter()
                .any(|(_, (range1, range2))| range1.contains(n) || range2.contains(n))
        })
    });

    let mut could_be = vec![rules.clone(); rules.len()];
    for ticket in valid_tickets {
        for (index, n) in ticket.iter().enumerate() {
            let deduced_index = could_be.get_mut(index).unwrap();
            let invalid_rules: Vec<_> = rules
                .iter()
                .filter(|(_, (range1, range2))| !range1.contains(n) && !range2.contains(n))
                .map(|(name, _)| name)
                .collect();
            for invalid_rule in invalid_rules {
                if let Some(pos) = deduced_index
                    .iter()
                    .position(|(name, _)| name == invalid_rule)
                {
                    deduced_index.swap_remove(pos);
                }
            }
        }
    }

    loop {
        let to_remove: Vec<_> = could_be
            .iter()
            .filter(|names| names.len() == 1)
            .map(|names| names.get(0).unwrap().0)
            .collect();

        let mut did_op = false;
        for names in &mut could_be {
            for name_to_remove in &to_remove {
                if names.len() != 1 {
                    if let Some(pos) = names.iter().position(|(name, _)| name == name_to_remove) {
                        did_op = true;
                        names.swap_remove(pos);
                    }
                }
            }
        }
        if !did_op {
            break;
        }
    }

    could_be
        .iter()
        .enumerate()
        .filter(|(_, names)| names.first().unwrap().0.starts_with("departure "))
        .map(|(index, _)| input.my_ticket.get(index).unwrap())
        .product()
}
