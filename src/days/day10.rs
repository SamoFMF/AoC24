use std::hash::Hash;

use hashbrown::HashSet;

use crate::{read_input, Solution, SolutionPair};

pub fn solve() -> SolutionPair {
    let map = parse_input();
    let (sol1, sol2) = parts(&map);

    (Solution::from(sol1), Solution::from(sol2))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Point(i32, i32);

fn parts(map: &Vec<Vec<i8>>) -> (usize, usize) {
    let mut result1 = 0;
    let mut result2 = 0;
    for (i, row) in map.iter().enumerate() {
        for (j, val) in row.iter().enumerate() {
            if *val == 0 {
                let (score1, score2) = count_trails(Point(i as i32, j as i32), map);
                result1 += score1;
                result2 += score2;
            }
        }
    }

    (result1, result2)
}

fn count_trails(p: Point, map: &Vec<Vec<i8>>) -> (usize, usize) {
    let mut nines = HashSet::new();
    let trails = count_trails_recursive(-1, p, map, &mut nines);
    (nines.len(), trails)
}

fn count_trails_recursive(
    prev: i8,
    p: Point,
    map: &Vec<Vec<i8>>,
    nines: &mut HashSet<Point>,
) -> usize {
    let cur = map[p.0 as usize][p.1 as usize];
    if cur != prev + 1 {
        return 0;
    } else if cur == 9 {
        nines.insert(p);
        return 1;
    }

    let mut result = 0;
    if p.0 > 0 {
        result += count_trails_recursive(cur, Point(p.0 - 1, p.1), map, nines);
    }
    if p.0 < (map.len() as i32) - 1 {
        result += count_trails_recursive(cur, Point(p.0 + 1, p.1), map, nines);
    }
    if p.1 > 0 {
        result += count_trails_recursive(cur, Point(p.0, p.1 - 1), map, nines);
    }
    if p.1 < (map[0].len() as i32) - 1 {
        result += count_trails_recursive(cur, Point(p.0, p.1 + 1), map, nines);
    }

    result
}

fn parse_input() -> Vec<Vec<i8>> {
    read_input!(10)
        .trim()
        .split("\n")
        .map(|line| {
            line.trim()
                .bytes()
                .map(|height| (height - 48) as i8)
                .collect()
        })
        .collect()
}
