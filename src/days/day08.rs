use std::hash::{Hash, Hasher};
use std::ops::{Add, Sub};

use hashbrown::{HashMap, HashSet};

use crate::{read_input, Solution, SolutionPair};

pub fn solve() -> SolutionPair {
    let (nodes, dim) = parse_input();

    let sol1 = part1(&nodes, dim);
    let sol2 = part2(&nodes, dim);

    (Solution::from(sol1), Solution::from(sol2))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Point(i32, i32);

impl Point {
    pub fn within_dim(&self, dim: Point) -> bool {
        self.0 >= 0 && self.0 < dim.0 && self.1 >= 0 && self.1 < dim.1
    }
}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_i32((self.0 << 8) | self.1)
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

fn part1(nodes: &HashMap<u8, Vec<Point>>, dim: Point) -> usize {
    let mut antinodes = HashSet::new();
    for ps in nodes.values() {
        for i in 0..ps.len() {
            for j in (i + 1)..ps.len() {
                let mut p1 = ps[i];
                let mut p2 = ps[j];
                let d = p1 - p2;
                p1 = p1 + d;
                p2 = p2 - d;
                if p1.within_dim(dim) {
                    antinodes.insert(p1);
                }
                if p2.within_dim(dim) {
                    antinodes.insert(p2);
                }
            }
        }
    }

    antinodes.len()
}

fn part2(nodes: &HashMap<u8, Vec<Point>>, dim: Point) -> usize {
    let mut antinodes = HashSet::new();
    for ps in nodes.values() {
        for i in 0..ps.len() {
            for j in (i + 1)..ps.len() {
                let mut p1 = ps[i];
                let mut p2 = ps[j];
                let d = p1 - p2;
                while p1.within_dim(dim) {
                    antinodes.insert(p1);
                    p1 = p1 + d;
                }

                while p2.within_dim(dim) {
                    antinodes.insert(p2);
                    p2 = p2 - d;
                }
            }
        }
    }

    antinodes.len()
}

fn parse_input() -> (HashMap<u8, Vec<Point>>, Point) {
    let mut antinodes: HashMap<u8, Vec<Point>> = HashMap::new();
    let (mut dim_i, mut dim_j) = (0, 0);
    read_input!(08)
        .trim()
        .split("\n")
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .enumerate()
        .for_each(|(i, line)| {
            dim_i = i as i32;
            dim_j = line.len() as i32;
            for (j, c) in line.bytes().enumerate() {
                if c != 46 {
                    let p = Point(i as i32, j as i32);
                    match antinodes.get_mut(&c) {
                        Some(ps) => {
                            ps.push(p);
                        }
                        None => {
                            let ps = vec![p];
                            antinodes.insert(c, ps);
                        }
                    };
                }
            }
        });

    (antinodes, Point(dim_i + 1, dim_j))
}
