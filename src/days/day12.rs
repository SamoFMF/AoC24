use std::collections::VecDeque;
use std::hash::{Hash, Hasher};

use hashbrown::{HashMap, HashSet};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{read_input, Solution, SolutionPair};

static DIRS: [Dir; 4] = [Dir::UP, Dir::RIGHT, Dir::DOWN, Dir::LEFT];

pub fn solve() -> SolutionPair {
    let grid = parse_input();

    let (sol1, gardens) = part1(grid);
    let sol2 = part2(gardens);

    (Solution::from(sol1), Solution::from(sol2))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Point(i32, i32);

impl Point {
    pub fn move_in(&self, dir: Dir) -> Self {
        match dir {
            Dir::UP => Self(self.0 - 1, self.1),
            Dir::DOWN => Self(self.0 + 1, self.1),
            Dir::LEFT => Self(self.0, self.1 - 1),
            Dir::RIGHT => Self(self.0, self.1 + 1),
        }
    }
}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_i32((self.0 << 8) | self.1)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Dir {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Dir {
    pub fn rotate_left(&self) -> Self {
        match self {
            Dir::UP => Self::LEFT,
            Dir::DOWN => Self::RIGHT,
            Dir::LEFT => Self::DOWN,
            Dir::RIGHT => Self::UP,
        }
    }

    pub fn rotate_right(&self) -> Self {
        match self {
            Dir::UP => Self::RIGHT,
            Dir::DOWN => Self::LEFT,
            Dir::LEFT => Self::UP,
            Dir::RIGHT => Self::DOWN,
        }
    }
}

fn parse_input() -> HashMap<Point, u8> {
    let mut grid = HashMap::new();
    read_input!(12)
        .trim()
        .split("\n")
        .map(|line| line.trim())
        .enumerate()
        .for_each(|(i, line)| {
            line.bytes().enumerate().for_each(|(j, v)| {
                grid.insert(Point(i as i32, j as i32), v);
            })
        });

    grid
}

fn part1(mut grid: HashMap<Point, u8>) -> (usize, Vec<HashSet<Point>>) {
    let mut result = 0;
    let mut gardens: Vec<HashSet<Point>> = Vec::new();

    while !grid.is_empty() {
        let p = grid.keys().next().unwrap();
        let garden = search_garden(*p, &mut grid);
        result += get_garden_cost(&garden);
        gardens.push(garden);
    }

    (result, gardens)
}

fn get_garden_cost(garden: &HashSet<Point>) -> usize {
    let perimeter: usize = garden
        .iter()
        .map(|p| {
            DIRS.iter()
                .map(|dir| p.move_in(*dir))
                .filter(|p| garden.contains(p))
                .count()
        })
        .map(|neighbours| 4 - neighbours)
        .sum();

    garden.len() * perimeter
}

fn search_garden(p: Point, grid: &mut HashMap<Point, u8>) -> HashSet<Point> {
    let v = grid.remove(&p).unwrap();
    let mut area = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(p);

    while let Some(p) = queue.pop_front() {
        area.insert(p);
        DIRS.iter().map(|dir| p.move_in(*dir)).for_each(|p_next| {
            if let Some(v1) = grid.get(&p_next) {
                if *v1 == v {
                    grid.remove(&p_next);
                    queue.push_back(p_next);
                }
            }
        });
    }

    area
}

fn part2(gardens: Vec<HashSet<Point>>) -> usize {
    gardens.par_iter().map(get_sides_count).sum()
}

fn get_sides_count(garden: &HashSet<Point>) -> usize {
    let mut sides = HashMap::new();
    for p in garden {
        for dir in DIRS {
            let p1 = p.move_in(dir);
            if garden.contains(&p1) {
                continue;
            }

            update_sides(*p, dir, &mut sides);
        }
    }

    let doubles = sides.iter().filter(|((p0, _), p1)| p0 != *p1).count() / 2;
    let sides_count = sides.len() - doubles;

    garden.len() * sides_count
}

fn update_sides(p: Point, dir: Dir, sides: &mut HashMap<(Point, Dir), Point>) {
    let pl = p.move_in(dir.rotate_left());
    let pl = sides.remove(&(pl, dir)).unwrap_or(p);

    let pr = p.move_in(dir.rotate_right());
    let pr = sides.remove(&(pr, dir)).unwrap_or(p);

    sides.insert((pl, dir), pr);
    sides.insert((pr, dir), pl);
}
