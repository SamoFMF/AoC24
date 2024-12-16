use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use hashbrown::{HashMap, HashSet};

use crate::{read_input, Solution, SolutionPair};

pub fn solve() -> SolutionPair {
    let (grid, start) = parse_input();

    let sol1 = part1(start, &grid);
    let sol2 = part2(start, sol1, &grid);

    (Solution::from(sol1), Solution::from(sol2))
}

fn part1(start: Point, grid: &Vec<Vec<Object>>) -> u64 {
    let mut seen = HashSet::new();
    let mut queue = BinaryHeap::with_capacity(grid.len() * grid[0].len());
    queue.push(State(0, start, Direction::Right));
    while !queue.is_empty() {
        let State(score, p, dir) = queue.pop().unwrap();

        // Check if end
        if grid[p.0][p.1] == Object::End {
            return score;
        }

        // Try to move forward
        let pf = p.move_dir(dir);
        if grid[pf.0][pf.1] != Object::Wall && !seen.contains(&(pf, dir)) {
            queue.push(State(score + 1, pf, dir));
            seen.insert((pf, dir));
        }

        // Rotate
        if !seen.contains(&(p, dir.rotate_right())) {
            queue.push(State(score + 1000, p, dir.rotate_right()));
            seen.insert((p, dir.rotate_right()));
        }
        if !seen.contains(&(p, dir.rotate_left())) {
            queue.push(State(score + 1000, p, dir.rotate_left()));
            seen.insert((p, dir.rotate_left()));
        }
    }

    unreachable!()
}

fn part2(start: Point, opt: u64, grid: &Vec<Vec<Object>>) -> usize {
    let mut seen: HashMap<(Point, Direction), u64> = HashMap::new();
    seen.insert((start, Direction::Right), 0);
    let mut on_opt_path: HashSet<Point> = HashSet::new();
    let mut queue = BinaryHeap::with_capacity(grid.len() * grid[0].len());
    queue.push(State2(0, start, Direction::Right, Rc::new(vec![start])));
    while !queue.is_empty() {
        let State2(score, p, dir, path) = queue.pop().unwrap();

        // Check score
        if score > opt {
            break;
        }

        // Check if end
        if grid[p.0][p.1] == Object::End {
            on_opt_path.extend(path.iter());
        }

        // Try to move forward
        let pf = p.move_dir(dir);
        if grid[pf.0][pf.1] != Object::Wall {
            if match seen.get(&(pf, dir)) {
                Some(opt_score) => score <= *opt_score,
                None => true,
            } {
                let mut path = (*path).clone();
                path.push(pf);
                queue.push(State2(score + 1, pf, dir, Rc::new(path)));
                seen.insert((pf, dir), score + 1);
            }
        }

        // Rotate
        if match seen.get(&(p, dir.rotate_right())) {
            Some(opt_score) => score <= *opt_score,
            None => true,
        } {
            queue.push(State2(score + 1000, p, dir.rotate_right(), path.clone()));
            seen.insert((p, dir.rotate_right()), score + 1000);
        }

        if match seen.get(&(p, dir.rotate_left())) {
            Some(opt_score) => score <= *opt_score,
            None => true,
        } {
            queue.push(State2(score + 1000, p, dir.rotate_left(), path));
            seen.insert((p, dir.rotate_left()), score + 1000);
        }
    }

    on_opt_path.len()
}

fn parse_input() -> (Vec<Vec<Object>>, Point) {
    let mut start = None;
    let grid = read_input!(16)
        .trim()
        .split("\n")
        .enumerate()
        .map(|(i, line)| {
            line.trim()
                .chars()
                .enumerate()
                .map(|(j, c)| {
                    if c == 'S' {
                        start = Some(Point(i, j));
                    }
                    Object::from(c)
                })
                .collect()
        })
        .collect();

    (grid, start.unwrap())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn rotate_right(&self) -> Self {
        match self {
            Direction::Up => Self::Right,
            Direction::Down => Self::Left,
            Direction::Left => Self::Up,
            Direction::Right => Self::Down,
        }
    }

    pub fn rotate_left(&self) -> Self {
        match self {
            Direction::Up => Self::Left,
            Direction::Down => Self::Right,
            Direction::Left => Self::Down,
            Direction::Right => Self::Up,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Object {
    Start,
    End,
    Empty,
    Wall,
}

impl From<char> for Object {
    fn from(object: char) -> Self {
        match object {
            '#' => Self::Wall,
            'S' => Self::Start,
            'E' => Self::End,
            _ => Self::Empty,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Point(usize, usize);

impl Point {
    pub fn move_dir(&self, m: Direction) -> Self {
        match m {
            Direction::Up => Point(self.0 - 1, self.1),
            Direction::Down => Point(self.0 + 1, self.1),
            Direction::Left => Point(self.0, self.1 - 1),
            Direction::Right => Point(self.0, self.1 + 1),
        }
    }
}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_usize((self.0 << 8) | self.1)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct State(u64, Point, Direction);

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.cmp(&self.0)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct State2(u64, Point, Direction, Rc<Vec<Point>>);

impl Ord for State2 {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.cmp(&self.0)
    }
}

impl PartialOrd for State2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
