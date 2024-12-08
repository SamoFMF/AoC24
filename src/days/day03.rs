use regex::Regex;

use crate::{read_input, Solution, SolutionPair};

pub fn solve() -> SolutionPair {
    let input = parse_input();

    let sol1: u32 = part1(&input);
    let sol2: u32 = part2(&input);

    (Solution::from(sol1), Solution::from(sol2))
}

enum Instruction {
    Mul(u32, u32),
    Do,
    Dont,
}

fn part1(input: &[Instruction]) -> u32 {
    input
        .iter()
        .map(|instruction| match instruction {
            Instruction::Mul(a, b) => (*a) * (*b),
            _ => 0,
        })
        .sum()
}

fn part2(input: &[Instruction]) -> u32 {
    let mut mul_enabled = true;
    let mut result = 0;
    for instruction in input {
        match instruction {
            Instruction::Mul(a, b) => {
                if mul_enabled {
                    result += (*a) * (*b);
                }
            }
            Instruction::Do => {
                mul_enabled = true;
            }
            Instruction::Dont => {
                mul_enabled = false;
            }
        }
    }

    result
}

fn parse_input() -> Vec<Instruction> {
    let re = Regex::new(r"(mul\((\d{1,3}),(\d{1,3})\))|((do)(\(\)))|((don't)(\(\)))").unwrap();
    let input = read_input!(03);

    re.captures_iter(input)
        .map(|c| c.extract())
        .map(|(_, [full, a, b])| {
            if full == "do()" {
                Instruction::Do
            } else if full == "don't()" {
                Instruction::Dont
            } else {
                Instruction::Mul(a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap())
            }
        })
        .collect()
}
