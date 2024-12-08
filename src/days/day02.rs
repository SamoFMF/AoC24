use crate::{read_input, Solution, SolutionPair};

pub fn solve() -> SolutionPair {
    let levels = get_input();

    let sol1 = part1(&levels);
    let sol2 = part2(&levels);

    (Solution::from(sol1), Solution::from(sol2))
}

fn part1(levels: &Vec<Vec<u32>>) -> usize {
    levels
        .iter()
        .filter(|level| is_level_safe_1(*level))
        .count()
}

fn part2(levels: &Vec<Vec<u32>>) -> usize {
    levels
        .iter()
        .filter(|level| is_level_safe_2_fast(*level))
        .count()
}

fn is_level_safe_1(level: &[u32]) -> bool {
    let factor = if level[1] < level[0] { -1 } else { 1 };
    for i in 1..level.len() {
        let diff = factor * ((level[i] as i32) - (level[i - 1] as i32));
        if diff < 1 || diff > 3 {
            return false;
        }
    }

    true
}

#[allow(dead_code)]
fn is_level_safe_2_simple(level: &[u32]) -> bool {
    for i in 0..level.len() {
        let sub_level = [&level[..i], &level[i + 1..]].concat();
        if is_level_safe_1(&sub_level) {
            return true;
        }
    }

    return false;
}

#[allow(dead_code)]
fn is_level_safe_2_fast(level: &[u32]) -> bool {
    let factor = if level[1] < level[0] { -1 } else { 1 };
    let idx = match is_level_safe_2_factor(level, factor) {
        Some(i) => i,
        None => return true,
    };

    if idx == level.len() - 1 {
        return true;
    } else if idx == 1 {
        // Remove at idx = 0
        let factor = if level[2] < level[1] { -1 } else { 1 };
        if is_level_safe_2_factor(&level[1..], factor).is_none() {
            return true;
        }

        // Remove at idx = 1
        let factor = if level[2] < level[0] { -1 } else { 1 };
        return is_level_safe_2_factor(&[level[0], level[2]], factor).is_none()
            && is_level_safe_2_factor(&level[2..], factor).is_none();
    } else if idx == 2 {
        // Skip at idx = 2
        if is_level_safe_2_factor(&[level[idx - 2], level[idx - 1], level[idx + 1]], factor)
            .is_none()
            && is_level_safe_2_factor(&level[idx + 1..], factor).is_none()
        {
            return true;
        }

        // Skip at idx = 0
        let factor = if level[2] < level[1] { -1 } else { 1 };
        if is_level_safe_2_factor(&level[1..], factor).is_none() {
            return true;
        }

        // Skip at idx = 1 - requires new factor
        let factor = if level[2] < level[0] { -1 } else { 1 };
        return is_level_safe_2_factor(&[level[idx - 2], level[idx], level[idx + 1]], factor)
            .is_none()
            && is_level_safe_2_factor(&level[idx + 1..], factor).is_none();
    }

    // Left side is already valid, check right side & possible middle connections
    let mid = is_level_safe_2_factor(&[level[idx - 2], level[idx - 1], level[idx + 1]], factor)
        .is_none()
        || is_level_safe_2_factor(&[level[idx - 2], level[idx], level[idx + 1]], factor).is_none();

    mid && is_level_safe_2_factor(&level[idx + 1..], factor).is_none()
}

fn is_level_safe_2_factor(level: &[u32], factor: i32) -> Option<usize> {
    for i in 1..level.len() {
        let diff = factor * ((level[i] as i32) - (level[i - 1] as i32));
        if diff < 1 || diff > 3 {
            return Some(i);
        }
    }

    None
}

fn get_input() -> Vec<Vec<u32>> {
    read_input!(02)
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|c| c.parse::<u32>().unwrap())
                .collect()
        })
        .collect()
}
