use regex::Regex;

use crate::{read_input, Solution, SolutionPair};

type Point = (i64, i64);

pub fn solve() -> SolutionPair {
    let machines = parse_intput();

    let sol1 = part1(&machines);
    let sol2 = part2(&machines);

    (Solution::from(sol1), Solution::from(sol2))
}

fn part1(machines: &[(Point, Point, Point)]) -> i64 {
    machines
        .iter()
        .map(|(a, b, p)| machine_price(*a, *b, *p))
        .sum()
}

fn part2(machines: &[(Point, Point, Point)]) -> i64 {
    machines
        .iter()
        .map(|(a, b, p)| (*a, *b, (p.0 + 10000000000000, p.1 + 10000000000000)))
        .map(|(a, b, p)| machine_price(a, b, p))
        .sum()
}

fn machine_price(a: Point, b: Point, p: Point) -> i64 {
    let left = a.1 * p.0 - a.0 * p.1;
    let right = a.1 * b.0 - a.0 * b.1;

    let m = left / right;
    let n = (p.0 - m * b.0) / a.0;

    if n * a.0 + m * b.0 == p.0 && n * a.1 + m * b.1 == p.1 {
        3 * n + m
    } else {
        0
    }
}

fn parse_intput() -> Vec<(Point, Point, Point)> {
    let re = Regex::new(r"X.(\d+), Y.(\d+)").unwrap();
    let points: Vec<Point> = re
        .captures_iter(read_input!(13))
        .map(|c| c.extract())
        .map(|(_, [x, y])| (x.parse().unwrap(), y.parse().unwrap()))
        .collect();

    let mut machines = Vec::with_capacity(points.len() / 3);
    for i in 0..points.len() / 3 {
        machines.push((points[3 * i], points[3 * i + 1], points[3 * i + 2]));
    }

    machines
}
