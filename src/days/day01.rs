use std::collections::HashMap;

use crate::{read_input, Solution, SolutionPair};

pub fn solve() -> SolutionPair {
    let (mut left, mut right) = get_input();

    let sol1 = part1(&mut left, &mut right);
    let sol2 = part2(&left, &right);
    (Solution::from(sol1), Solution::from(sol2))
}

fn part1(left: &mut [u32], right: &mut [u32]) -> u32 {
    left.sort();
    right.sort();

    let mut result = 0u32;
    for i in 0..left.len() {
        result += left[i].abs_diff(right[i]);
    }

    result
}

fn part2(left: &[u32], right: &[u32]) -> u32 {
    let mut right_map = HashMap::new();
    for value in right {
        match right_map.get_mut(value) {
            Some(count) => *count += 1,
            None => {
                right_map.insert(*value, 1);
            }
        }
    }

    let mut result: u32 = 0;
    for value in left {
        let count = right_map.get(value).map(|x| *x).unwrap_or(0);
        result += *value * count;
    }

    result
}

fn get_input() -> (Vec<u32>, Vec<u32>) {
    read_input!(01)
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut split = line.split_ascii_whitespace();
            (split.next().unwrap(), split.next().unwrap())
        })
        .map(|(left, right)| (left.parse::<u32>().unwrap(), right.parse::<u32>().unwrap()))
        .unzip()
}
