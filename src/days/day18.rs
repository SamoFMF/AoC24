use std::collections::VecDeque;
use std::hash::{Hash, Hasher};

use hashbrown::{HashMap, HashSet};

use crate::{read_input, Solution, SolutionPair};

const DIM: Point = Point(71, 71);
const DIRS: [Direction; 4] = [
    Direction::Up,
    Direction::Right,
    Direction::Down,
    Direction::Left,
];
const TIME: usize = 1024;

pub fn solve() -> SolutionPair {
    let grid = parse_input();

    let sol1 = part1(&grid);
    let sol2 = part2(&grid);

    (Solution::from(sol1), Solution::from(sol2))
}

fn part1(grid: &HashMap<Point, usize>) -> usize {
    bfs(TIME, grid).unwrap()
}

fn part2(grid: &HashMap<Point, usize>) -> String {
    let mut left = TIME;
    let mut right = grid.len() + 1;
    while right - left > 1 {
        let mid = (left + right) / 2;
        if bfs(mid, grid).is_none() {
            right = mid;
        } else {
            left = mid;
        }
    }

    let (p, _) = grid.iter().find(|(_, t)| right - 1 == **t).unwrap();
    format!("{},{}", p.0 - 1, p.1 - 1)
}

fn bfs(time: usize, grid: &HashMap<Point, usize>) -> Option<usize> {
    let mut seen = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((0, Point(1, 1)));
    seen.insert(Point(1, 1));
    while !queue.is_empty() {
        let (d, p) = queue.pop_front().unwrap();

        if p == DIM {
            return Some(d);
        }

        for dir in DIRS {
            let pc = p.move_dir(dir);
            if !pc.inside_dim() || seen.contains(&pc) {
                continue;
            }

            if let Some(ns) = grid.get(&pc) {
                if *ns < time {
                    continue;
                }
            }

            queue.push_back((d + 1, pc));
            seen.insert(pc);
        }
    }

    None
}

fn parse_input() -> HashMap<Point, usize> {
    read_input!(18)
        .trim()
        .split("\n")
        .map(|line| line.trim())
        .enumerate()
        .map(|(i, line)| {
            let mut split = line.split(",").map(|c| c.parse::<usize>().unwrap());
            (
                Point(split.next().unwrap() + 1, split.next().unwrap() + 1), // add 1 to avoid negative numbers when subtracting
                i,
            )
        })
        .collect()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Point(usize, usize);

impl Point {
    pub fn move_dir(&self, direction: Direction) -> Self {
        match direction {
            Direction::Up => Self(self.0, self.1 - 1),
            Direction::Down => Self(self.0, self.1 + 1),
            Direction::Left => Self(self.0 - 1, self.1),
            Direction::Right => Self(self.0 + 1, self.1),
        }
    }

    pub fn inside_dim(&self) -> bool {
        self.0 > 0 && self.0 <= DIM.0 && self.1 > 0 && self.1 <= DIM.1 // added 1 to x and y coords
    }
}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_usize((self.0 << 8) | self.1)
    }
}
