use std::collections::VecDeque;
use std::hash::{Hash, Hasher};

use hashbrown::HashMap;

use crate::{read_input, Solution, SolutionPair};

const DIRS: [Direction; 4] = [
    Direction::Up,
    Direction::Right,
    Direction::Down,
    Direction::Left,
];

pub fn solve() -> SolutionPair {
    let grid = parse_input();
    let start = get_object(Object::Start, &grid);
    let end = get_object(Object::End, &grid);
    let dist_start = get_distances(start, &grid);
    let dist_end = get_distances(end, &grid);
    let dim = Point(grid.len(), grid[0].len());

    let sol1 = part1_new(start, dim, &dist_start, &dist_end);
    let sol2 = part2(start, dim, &dist_start, &dist_end);

    (Solution::from(sol1), Solution::from(sol2))
}

fn part1_new(
    start: Point,
    dim: Point,
    dist_start: &HashMap<Point, usize>,
    dist_end: &HashMap<Point, usize>,
) -> usize {
    let fair_score = *dist_end.get(&start).unwrap() - 100;
    count_cheats(2, fair_score, dim, dist_start, dist_end)
}

fn part2(
    start: Point,
    dim: Point,
    dist_start: &HashMap<Point, usize>,
    dist_end: &HashMap<Point, usize>,
) -> usize {
    let fair_score = *dist_end.get(&start).unwrap() - 100;
    count_cheats(20, fair_score, dim, dist_start, dist_end)
}

fn count_cheats(
    t: usize,
    fair_score: usize,
    dim: Point,
    dist_start: &HashMap<Point, usize>,
    dist_end: &HashMap<Point, usize>,
) -> usize {
    let mut result = 0;
    for (&p, &ds) in dist_start {
        for i in 0..=t {
            let j0 = if i == 0 { 1 } else { 0 };
            if p.0 > i {
                for j in j0..=(t - i) {
                    if p.1 > j
                        && check_cheat(ds, fair_score - i - j, Point(p.0 - i, p.1 - j), &dist_end)
                    {
                        result += 1;
                    }
                    if j > 0
                        && p.1 + j < dim.1
                        && check_cheat(ds, fair_score - i - j, Point(p.0 - i, p.1 + j), &dist_end)
                    {
                        result += 1;
                    }
                }
            }

            if i == 0 {
                continue;
            }
            if p.0 + i < dim.0 {
                for j in j0..=(t - i) {
                    if p.1 > j
                        && check_cheat(ds, fair_score - i - j, Point(p.0 + i, p.1 - j), &dist_end)
                    {
                        result += 1;
                    }
                    if j > 0
                        && p.1 + j < dim.1
                        && check_cheat(ds, fair_score - i - j, Point(p.0 + i, p.1 + j), &dist_end)
                    {
                        result += 1;
                    }
                }
            }
        }
    }

    result
}

fn check_cheat(ds: usize, goal: usize, p: Point, dists: &HashMap<Point, usize>) -> bool {
    match dists.get(&p) {
        Some(de) => *de + ds <= goal,
        None => false,
    }
}

fn get_distances(goal: Point, grid: &Vec<Vec<Object>>) -> HashMap<Point, usize> {
    let mut seen = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_back((goal, 0));
    seen.insert(goal, 0);
    while !queue.is_empty() {
        let (p, score) = queue.pop_front().unwrap();
        for dir in DIRS {
            let pc = p.move_dir(dir);
            if grid[pc.0][pc.1] != Object::Wall && !seen.contains_key(&pc) {
                queue.push_back((pc, score + 1));
                seen.insert(pc, score + 1);
            }
        }
    }

    seen
}

fn get_object(object: Object, grid: &Vec<Vec<Object>>) -> Point {
    for (i, row) in grid.iter().enumerate() {
        for (j, obj) in row.iter().enumerate() {
            if *obj == object {
                return Point(i, j);
            }
        }
    }

    unreachable!()
}

fn parse_input() -> Vec<Vec<Object>> {
    read_input!(20)
        .trim()
        .split("\n")
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().map(|c| Object::from(c)).collect())
        .collect()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Object {
    Start,
    End,
    Wall,
    Empty,
}

impl From<char> for Object {
    fn from(c: char) -> Self {
        match c {
            '#' => Self::Wall,
            '.' => Self::Empty,
            'S' => Self::Start,
            'E' => Self::End,
            _ => unreachable!("c = {c}"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Point(usize, usize);

impl Point {
    pub fn move_dir(&self, direction: Direction) -> Self {
        match direction {
            Direction::Up => Self(self.0 - 1, self.1),
            Direction::Down => Self(self.0 + 1, self.1),
            Direction::Left => Self(self.0, self.1 - 1),
            Direction::Right => Self(self.0, self.1 + 1),
        }
    }
}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_usize((self.0 << 8) | self.1)
    }
}
