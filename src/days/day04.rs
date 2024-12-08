use std::cmp::{max, min};

use crate::{read_input, Solution, SolutionPair};

static XMAS: &[u8] = "XMAS".as_bytes();
static A: u8 = 65;
static M: u8 = 77;
static S: u8 = 83;
static DIRS: &[(i8, i8)] = &[
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];

pub fn solve() -> SolutionPair {
    let lines = parse_input();
    let sol1: u32 = part1(&lines);
    let sol2: u32 = part2(&lines);

    (Solution::from(sol1), Solution::from(sol2))
}

fn part1(lines: &Vec<Vec<u8>>) -> u32 {
    let mut result = 0;
    for i in 0..lines.len() {
        for j in 0..lines[i].len() {
            result += search_at(XMAS, (i, j), lines);
        }
    }

    result
}

fn part2(lines: &Vec<Vec<u8>>) -> u32 {
    let mut result = 0;
    for i in 1..lines.len() - 1 {
        for j in 1..lines[i].len() - 1 {
            if lines[i][j] == A {
                if max(lines[i - 1][j - 1], lines[i + 1][j + 1]) == S
                    && min(lines[i - 1][j - 1], lines[i + 1][j + 1]) == M
                    && max(lines[i - 1][j + 1], lines[i + 1][j - 1]) == S
                    && min(lines[i - 1][j + 1], lines[i + 1][j - 1]) == M
                {
                    result += 1;
                }
            }
        }
    }

    result
}

fn search_at(word: &[u8], pos: (usize, usize), lines: &Vec<Vec<u8>>) -> u32 {
    let mut result = 0;
    for dir in DIRS {
        if search_at_dir(word, pos, *dir, lines) {
            result += 1;
        }
    }

    result
}

fn search_at_dir(
    word: &[u8],
    (i, j): (usize, usize),
    (di, dj): (i8, i8),
    lines: &Vec<Vec<u8>>,
) -> bool {
    let (mut i_opt, mut j_opt) = (Some(i), Some(j));
    for k in 0..word.len() {
        if i_opt.is_none() || j_opt.is_none() {
            return false;
        }
        let (i, j) = (i_opt.unwrap(), j_opt.unwrap());

        if i == lines.len() || j == lines[i].len() || word[k] != lines[i][j] {
            return false;
        }

        if di < 0 {
            i_opt = i.checked_sub(di.abs() as usize);
        } else {
            i_opt = i.checked_add(di as usize);
        }
        if dj < 0 {
            j_opt = j.checked_sub(dj.abs() as usize);
        } else {
            j_opt = j.checked_add(dj as usize);
        }
    }

    true
}

fn parse_input() -> Vec<Vec<u8>> {
    read_input!(04)
        .trim()
        .split("\n")
        .map(|line| line.trim())
        .map(|line| line.bytes().collect())
        .collect()
}
