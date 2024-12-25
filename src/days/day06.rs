use std::hash::{Hash, Hasher};

use hashbrown::HashSet;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelIterator;

use crate::{read_input, Solution, SolutionPair};

static DIM: usize = 130;

pub fn solve() -> SolutionPair {
    let (start_b, mut grid_b) = parse_input();

    let (sol1, visited) = part1(start_b, &grid_b);
    let sol2 = part2(start_b, &visited, &mut grid_b);

    (Solution::from(sol1), Solution::from(sol2))
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Position(usize, usize);

impl Position {
    pub fn move_in_dir(&self, direction: Direction) -> Option<Self> {
        match direction {
            Direction::UP => {
                if self.0 > 0 {
                    Some(Self(self.0 - 1, self.1))
                } else {
                    None
                }
            }
            Direction::RIGHT => Some(Self(self.0, self.1 + 1)),
            Direction::DOWN => Some(Self(self.0 + 1, self.1)),
            Direction::LEFT => {
                if self.1 > 0 {
                    Some(Self(self.0, self.1 - 1))
                } else {
                    None
                }
            }
        }
    }
}

impl Hash for Position {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // works for indexes up to u8::MAX - for the given input, max index is 129
        state.write_usize((self.0 << 8) | self.1)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

impl Direction {
    pub fn rotate(self) -> Self {
        match self {
            Direction::UP => Direction::RIGHT,
            Direction::RIGHT => Direction::DOWN,
            Direction::DOWN => Direction::LEFT,
            Direction::LEFT => Direction::UP,
        }
    }
}

#[derive(Debug, Clone)]
struct Grid {
    pub rows: Vec<Vec<usize>>,
    pub cols: Vec<Vec<usize>>,
    pub dim: Position,
}

fn part1(mut pos: Position, grid: &Grid) -> (usize, HashSet<Position>) {
    let mut visited = HashSet::new();
    visited.insert(pos);
    let mut dir = Direction::UP;
    loop {
        let (stop, to_break) = match find_obstacle(pos, dir, &grid) {
            Some(val) => match dir {
                Direction::UP | Direction::DOWN => ((val as isize, pos.1  as isize), false), //(Position(val, pos.1), false),
                Direction::LEFT | Direction::RIGHT => ((pos.0  as isize, val  as isize), false),
            },
            None => match dir {
                Direction::UP => ((-1, pos.1  as isize), true),
                Direction::DOWN => ((grid.dim.0  as isize, pos.1  as isize), true),
                Direction::LEFT => ((pos.0 as isize, -1), true),
                Direction::RIGHT => ((pos.0 as isize, grid.dim.1 as isize), true),
            },
        };

        loop {
            let next = if let Some(next) = pos.move_in_dir(dir) {
                next
            } else {
                break;
            };
            if stop.0 > 0 && stop.1 > 0 && next == Position(stop.0 as usize, stop.1 as usize) {
                break;
            }

            pos = next;
            visited.insert(pos);
        }

        if to_break {
            break;
        }

        dir = dir.rotate();
    }

    (visited.len(), visited)
}

fn part2(start: Position, visited: &HashSet<Position>, grid: &mut Grid) -> usize {
    let visited: Vec<Position> = Vec::from_iter(visited.iter().cloned());
    visited
        .into_par_iter()
        .filter(|&pos| pos != start)
        .filter(|&pos| is_stuck(start, pos, &grid))
        .count()
}

#[allow(dead_code)]
fn part2_clone(start: Position, visited: &HashSet<Position>, grid: &mut Grid) -> usize {
    let visited: Vec<Position> = Vec::from_iter(visited.iter().cloned());
    visited
        .into_par_iter()
        .filter(|&pos| pos != start)
        .filter(move |&Position(i, j)| {
            let mut grid = grid.clone();
            let ins_i = insert(j, &mut grid.rows[i]);
            let ins_j = insert(i, &mut grid.cols[j]);

            let stuck = is_stuck_clone(start, &grid);

            grid.rows[i].remove(ins_i);
            grid.cols[j].remove(ins_j);

            stuck
        })
        .count()
}

fn insert(target: usize, vals: &mut Vec<usize>) -> usize {
    match bisection(target, vals) {
        Bisection::EMPTY | Bisection::RIGHT => {
            vals.push(target);
            vals.len() - 1
        }
        Bisection::LEFT => {
            vals.insert(0, target);
            0
        }
        Bisection::IN(idx) => {
            vals.insert(idx + 1, target);
            idx + 1
        }
    }
}

fn find_obstacle(pos: Position, dir: Direction, grid: &Grid) -> Option<usize> {
    match dir {
        Direction::UP => {
            let col = &grid.cols[pos.1];
            match bisection(pos.0, col) {
                Bisection::LEFT | Bisection::EMPTY => None,
                Bisection::RIGHT => Some(*col.last().unwrap()),
                Bisection::IN(i) => Some(col[i]),
            }
        }
        Direction::RIGHT => {
            let row = &grid.rows[pos.0];
            match bisection(pos.1, row) {
                Bisection::EMPTY | Bisection::RIGHT => None,
                Bisection::LEFT => Some(row[0]),
                Bisection::IN(i) => Some(row[i + 1]),
            }
        }
        Direction::DOWN => {
            let col = &grid.cols[pos.1];
            match bisection(pos.0, col) {
                Bisection::RIGHT | Bisection::EMPTY => None,
                Bisection::LEFT => Some(col[0]),
                Bisection::IN(i) => Some(col[i + 1]),
            }
        }
        Direction::LEFT => {
            let row = &grid.rows[pos.0];
            match bisection(pos.1, row) {
                Bisection::EMPTY | Bisection::LEFT => None,
                Bisection::RIGHT => Some(*row.last().unwrap()),
                Bisection::IN(i) => Some(row[i]),
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Bisection {
    EMPTY,
    LEFT,
    RIGHT,
    IN(usize),
}

fn bisection(target: usize, vals: &[usize]) -> Bisection {
    if vals.is_empty() {
        return Bisection::EMPTY;
    } else if target < vals[0] {
        return Bisection::LEFT;
    } else if target > *vals.last().unwrap() {
        return Bisection::RIGHT;
    }

    let (mut left, mut right) = (0, vals.len());
    while right - left > 1 {
        let mid = (right + left) / 2;
        let val = vals[mid];
        if val <= target {
            left = mid;
        } else {
            right = mid;
        }
    }

    Bisection::IN(left)
}

fn is_stuck(mut pos: Position, obstacle: Position, grid: &Grid) -> bool {
    let mut visited = HashSet::new();
    let mut dir = Direction::UP;
    loop {
        pos = match find_obstacle(pos, dir, &grid) {
            Some(val) => match dir {
                Direction::UP => {
                    let val = if pos.1 == obstacle.1 && pos.0 > obstacle.0 && val < obstacle.0 {
                        obstacle.0
                    } else {
                        val
                    };
                    Position(val + 1, pos.1)
                }
                Direction::DOWN => {
                    let val = if pos.1 == obstacle.1 && pos.0 < obstacle.0 && val > obstacle.0 {
                        obstacle.0
                    } else {
                        val
                    };
                    Position(val - 1, pos.1)
                }
                Direction::LEFT => {
                    let val = if pos.0 == obstacle.0 && pos.1 > obstacle.1 && val < obstacle.1 {
                        obstacle.1
                    } else {
                        val
                    };
                    Position(pos.0, val + 1)
                }
                Direction::RIGHT => {
                    let val = if pos.0 == obstacle.0 && pos.1 < obstacle.1 && val > obstacle.1 {
                        obstacle.1
                    } else {
                        val
                    };
                    Position(pos.0, val - 1)
                }
            },
            None => match dir {
                Direction::UP => {
                    if pos.1 == obstacle.1 && pos.0 > obstacle.0 {
                        Position(obstacle.0 + 1, pos.1)
                    } else {
                        return false;
                    }
                }
                Direction::DOWN => {
                    if pos.1 == obstacle.1 && pos.0 < obstacle.0 {
                        Position(obstacle.0 - 1, pos.1)
                    } else {
                        return false;
                    }
                }
                Direction::LEFT => {
                    if pos.0 == obstacle.0 && pos.1 > obstacle.1 {
                        Position(pos.0, obstacle.1 + 1)
                    } else {
                        return false;
                    }
                }
                Direction::RIGHT => {
                    if pos.0 == obstacle.0 && pos.1 < obstacle.1 {
                        Position(pos.0, obstacle.1 - 1)
                    } else {
                        return false;
                    }
                }
            },
        };

        dir = dir.rotate();

        let pair = (pos, dir);
        if visited.contains(&pair) {
            return true;
        }

        visited.insert(pair);
    }
}

fn is_stuck_clone(mut pos: Position, grid: &Grid) -> bool {
    let mut visited = HashSet::new();
    let mut dir = Direction::UP;
    loop {
        pos = match find_obstacle(pos, dir, &grid) {
            Some(val) => match dir {
                Direction::UP => Position(val + 1, pos.1),
                Direction::DOWN => Position(val - 1, pos.1),
                Direction::LEFT => Position(pos.0, val + 1),
                Direction::RIGHT => Position(pos.0, val - 1),
            },
            None => {
                return false;
            }
        };

        dir = dir.rotate();

        let pair = (pos, dir);
        if visited.contains(&pair) {
            return true;
        }

        visited.insert(pair);
    }
}

fn parse_input() -> (Position, Grid) {
    let mut start = Position(0, 0);
    let mut rows = vec![Vec::with_capacity(DIM); DIM];
    let mut cols = vec![Vec::with_capacity(DIM); DIM];
    let (mut dim_i, mut dim_j) = (0, 0);
    for (i, line) in read_input!(06)
        .trim()
        .split("\n")
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .enumerate()
    {
        dim_i = i;
        dim_j = line.len();
        for (j, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    rows[i].push(j);
                    cols[j].push(i);
                }
                '^' => {
                    start = Position(i, j);
                }
                _ => {}
            }
        }
    }

    let dim = Position(dim_i + 1, dim_j);
    (start, Grid { rows, cols, dim })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bisection_test() {
        let vals = vec![1, 5, 10, 20, 100];
        assert_eq!(Bisection::EMPTY, bisection(1, &vec![]));
        assert_eq!(Bisection::LEFT, bisection(0, &vals));
        assert_eq!(Bisection::IN(0), bisection(3, &vals));
        assert_eq!(Bisection::IN(3), bisection(20, &vals));
        assert_eq!(Bisection::IN(3), bisection(26, &vals));
        assert_eq!(Bisection::IN(4), bisection(100, &vals));
        assert_eq!(Bisection::RIGHT, bisection(1234, &vals));
        assert_eq!(Bisection::RIGHT, bisection(4, &vec![1]));
        assert_eq!(Bisection::RIGHT, bisection(6, &vec![1, 4]));
    }
}
