use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelRefIterator;

use crate::{read_input, Solution, SolutionPair};

pub fn solve() -> SolutionPair {
    let input = parse_input();

    let sol1: u64 = part1(&input);
    let sol2: u64 = part2(&input);

    (Solution::from(sol1), Solution::from(sol2))
}

fn part1(input: &Vec<(u64, Vec<u64>)>) -> u64 {
    input
        .par_iter()
        .filter(|(target, vals)| check_recursive_1(*target, 0, vals))
        .map(|(target, _)| target)
        .sum()
}

fn check_recursive_1(target: u64, cur: u64, vals: &[u64]) -> bool {
    if vals.is_empty() {
        return target == cur;
    } else if cur > target {
        return false;
    }

    check_recursive_1(target, cur + vals[0], &vals[1..])
        || check_recursive_1(target, cur * vals[0], &vals[1..])
}

fn part2(input: &Vec<(u64, Vec<u64>)>) -> u64 {
    input
        .par_iter()
        .filter(|(target, vals)| check_recursive_2(*target, 0, vals))
        .map(|(target, _)| target)
        .sum()
}

fn check_recursive_2(target: u64, cur: u64, vals: &[u64]) -> bool {
    if vals.is_empty() {
        return target == cur;
    } else if cur > target {
        return false;
    }

    let all_concat = vals.iter().fold(cur, |acc, val| concat(acc, *val));
    if all_concat < target {
        // concat grows the fastest - if it can't reach target, return false
        return false;
    }

    check_recursive_2(target, cur + vals[0], &vals[1..])
        || check_recursive_2(target, cur * vals[0], &vals[1..])
        || check_recursive_2(target, concat(cur, vals[0]), &vals[1..])
}

fn concat(lhs: u64, rhs: u64) -> u64 {
    let rhs_digits = rhs.ilog10() + 1;
    lhs * 10u64.pow(rhs_digits) + rhs
}

fn parse_input() -> Vec<(u64, Vec<u64>)> {
    read_input!(07)
        .trim()
        .split("\n")
        .map(|line| {
            let mut split = line.trim().split(":");
            let target = split.next().unwrap().parse::<u64>().unwrap();
            let vals = split
                .next()
                .unwrap()
                .trim()
                .split(" ")
                .map(|val| val.parse::<u64>().unwrap())
                .collect();
            (target, vals)
        })
        .collect()
}
