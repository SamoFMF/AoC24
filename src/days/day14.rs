use std::hash::{Hash, Hasher};

use hashbrown::HashSet;
use regex::Regex;

use crate::{read_input, Solution, SolutionPair};

static DIM: Point = Point(101, 103);

pub fn solve() -> SolutionPair {
    let robots = parse_input();

    let sol1 = part1(&robots);
    let sol2 = part2(&robots);

    (Solution::from(sol1), Solution::from(sol2))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Point(i64, i64);

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_i64((self.0 << 8) | self.1)
    }
}

fn part1(robots: &[(Point, Point)]) -> usize {
    let mut quadrants = [0, 0, 0, 0];
    robots
        .iter()
        .map(|robot| {
            Point(
                ((robot.0 .0 + 100 * robot.1 .0) % DIM.0 + DIM.0) % DIM.0,
                ((robot.0 .1 + 100 * robot.1 .1) % DIM.1 + DIM.1) % DIM.1,
            )
        })
        .filter(|p| p.0 != DIM.0 / 2 && p.1 != DIM.1 / 2)
        .for_each(|p| {
            let xq = if p.0 < DIM.0 / 2 { 0 } else { 1 };
            let yq = if p.1 < DIM.1 / 2 { 0 } else { 1 };
            quadrants[2 * yq + (xq as usize)] += 1;
        });

    quadrants.iter().product()
}

fn part2(robots: &[(Point, Point)]) -> usize {
    let mut i = 0;
    loop {
        let points: HashSet<Point> = robots
            .iter()
            .map(|robot| {
                Point(
                    ((robot.0 .0 + i * robot.1 .0) % DIM.0 + DIM.0) % DIM.0,
                    ((robot.0 .1 + i * robot.1 .1) % DIM.1 + DIM.1) % DIM.1,
                )
            })
            .collect();

        if points
            .iter()
            .find(|p| {
                for i in 1..=5 {
                    if !points.contains(&Point(p.0 + i, p.1))
                        || !points.contains(&Point(p.0, p.1 + i))
                    {
                        return false;
                    }
                }

                true
            })
            .is_some()
        {
            return i as usize;
        }

        i += 1;
    }
}

fn parse_input() -> Vec<(Point, Point)> {
    let re = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();
    re.captures_iter(read_input!(14))
        .map(|c| c.extract())
        .map(|(_, [px, py, vx, vy])| {
            (
                Point(px.parse().unwrap(), py.parse().unwrap()),
                Point(vx.parse().unwrap(), vy.parse().unwrap()),
            )
        })
        .collect()
}
