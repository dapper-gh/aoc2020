use aoc_runner_derive::{aoc, aoc_generator};

type Input = Vec<String>;

const REQUIRED: [&str; 7] = ["byr:", "iyr:", "eyr:", "hgt:", "hcl:", "ecl:", "pid:"];

fn byr(body: &str) -> bool {
    if let Ok(num) = body.parse::<u32>() {
        num >= 1920 && num <= 2002
    } else {
        false
    }
}
fn iyr(body: &str) -> bool {
    if let Ok(num) = body.parse::<u32>() {
        num >= 2010 && num <= 2020
    } else {
        false
    }
}
fn eyr(body: &str) -> bool {
    if let Ok(num) = body.parse::<u32>() {
        num >= 2020 && num <= 2030
    } else {
        false
    }
}
fn hgt(body: &str) -> bool {
    if body.len() < 3 {
        return false;
    }
    let (num, unit) = body.split_at(body.len() - 2);
    if let Ok(num) = num.parse::<u32>() {
        match unit {
            "cm" => num >= 150 && num <= 193,
            "in" => num >= 59 && num <= 76,
            _ => false,
        }
    } else {
        false
    }
}
fn hcl(body: &str) -> bool {
    let mut chars = body.chars();
    if chars.next() != Some('#') {
        return false;
    }
    let mut count = 0;
    for c in chars {
        if !c.is_ascii_hexdigit() {
            return false;
        }
        count += 1;
        if count > 6 {
            return false;
        }
    }
    count == 6
}
fn ecl(body: &str) -> bool {
    ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&body)
}
fn pid(body: &str) -> bool {
    let mut count = 0;
    for c in body.chars() {
        if !c.is_ascii_digit() {
            return false;
        }
        count += 1;
        if count > 9 {
            return false;
        }
    }
    count == 9
}

#[aoc_generator(day4)]
pub fn generator(input: &str) -> Vec<Input> {
    input
        .split("\n\n")
        .map(|passport| passport.split_whitespace().map(|s| s.to_owned()).collect())
        .collect()
}

#[aoc(day4, part1)]
pub fn p1_solver(input: &[Input]) -> usize {
    input
        .iter()
        .filter(|passport| {
            for required in REQUIRED.iter() {
                if !passport.iter().any(|s| s.starts_with(required)) {
                    return false;
                }
            }
            true
        })
        .count()
}

#[aoc(day4, part2)]
pub fn p2_solver(input: &[Input]) -> usize {
    input
        .iter()
        .filter(|passport| {
            let mut total: u32 = 0;
            for group in passport.iter() {
                let (prefix, body) = group.split_at(4);
                match prefix {
                    "byr:" => {
                        if byr(body) {
                            total += 1
                        }
                    }
                    "iyr:" => {
                        if iyr(body) {
                            total += 1
                        }
                    }
                    "eyr:" => {
                        if eyr(body) {
                            total += 1
                        }
                    }
                    "hgt:" => {
                        if hgt(body) {
                            total += 1
                        }
                    }
                    "hcl:" => {
                        if hcl(body) {
                            total += 1
                        }
                    }
                    "ecl:" => {
                        if ecl(body) {
                            total += 1
                        }
                    }
                    "pid:" => {
                        if pid(body) {
                            total += 1
                        }
                    }
                    _ => {}
                };
            }
            total == 7
        })
        .count()
}
