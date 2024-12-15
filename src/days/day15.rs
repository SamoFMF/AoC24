use std::hash::{Hash, Hasher};

use hashbrown::HashMap;

use crate::{read_input, Solution, SolutionPair};

type Grid = HashMap<Point, Object>;

pub fn solve() -> SolutionPair {
    let (start1, mut grid1, moves) = parse_input();
    let (start2, mut grid2) = prepare_part2(start1, &grid1);

    let sol1 = part1(start1, &mut grid1, &moves);
    let sol2 = part2(start2, &mut grid2, &moves);

    (Solution::from(sol1), Solution::from(sol2))
}

fn part1(mut pos: Point, grid: &mut Grid, moves: &Vec<Move>) -> i64 {
    moves.iter().for_each(|m| pos = do_move1(pos, *m, grid));

    get_score(grid)
}

fn do_move1(p: Point, m: Move, grid: &mut Grid) -> Point {
    let mut pc = p.apply_move(m);
    loop {
        match grid.get(&pc) {
            Some(Object::Box) => {
                pc = pc.apply_move(m);
            }
            Some(_) => {
                return p; // wall
            }
            None => {
                break;
            }
        }
    }

    grid.insert(pc, Object::Box);
    let p = p.apply_move(m);
    grid.remove(&p);

    return p;
}

fn get_score(grid: &Grid) -> i64 {
    grid.iter()
        .filter(|(_, object)| *object == &Object::Box)
        .map(|(p, _)| 100 * p.0 + p.1)
        .sum()
}

fn part2(mut pos: Point, grid: &mut Grid, moves: &Vec<Move>) -> i64 {
    moves.iter().for_each(|m| {
        pos = do_move2(pos, *m, grid);
    });

    get_score(grid)
}

fn do_move2(p: Point, m: Move, grid: &mut Grid) -> Point {
    match m {
        Move::Left => push_left(p, grid),
        Move::Right => push_right(p, grid),
        Move::Up | Move::Down => push_up_down(p, m, grid),
    }
}

fn push_up_down(p: Point, m: Move, grid: &mut Grid) -> Point {
    let pc = p.apply_move(m);
    let result = match (grid.get(&Point(pc.0, pc.1 - 1)), grid.get(&pc)) {
        (Some(Object::Wall), _) | (_, Some(Object::Wall)) => Err(()),
        (_, Some(Object::Box)) => push_up_down_recursive(m, vec![pc], grid),
        (Some(Object::Box), _) => push_up_down_recursive(m, vec![Point(pc.0, pc.1 - 1)], grid),
        _ => Ok(()),
    };

    match result {
        Ok(_) => p.apply_move(m),
        Err(_) => p,
    }
}

fn push_up_down_recursive(m: Move, boxes: Vec<Point>, grid: &mut Grid) -> Result<(), ()> {
    let mut next_boxes = Vec::with_capacity(boxes.len() + 1);
    for b in &boxes {
        let p = b.apply_move(m);
        update_boxes(p, &mut next_boxes, grid)?;
        update_boxes(Point(p.0, p.1 - 1), &mut next_boxes, grid)?;
        update_boxes(Point(p.0, p.1 + 1), &mut next_boxes, grid)?;
    }

    if !next_boxes.is_empty() {
        push_up_down_recursive(m, next_boxes, grid)?;
    }

    boxes.into_iter().for_each(|b| {
        grid.remove(&b);
        grid.insert(b.apply_move(m), Object::Box);
    });

    Ok(())
}

fn update_boxes(p: Point, boxes: &mut Vec<Point>, grid: &Grid) -> Result<(), ()> {
    match grid.get(&p) {
        Some(Object::Box) => {
            boxes.push(p);
        }
        Some(Object::Wall) => {
            return Err(());
        }
        _ => {}
    }

    Ok(())
}

fn push_right(p: Point, grid: &mut Grid) -> Point {
    match push_left_right(p, Move::Right, grid) {
        Ok(_) => p.apply_move(Move::Right),
        Err(_) => p,
    }
}

fn push_left(p: Point, grid: &mut Grid) -> Point {
    match push_left_right(p.apply_move(Move::Left), Move::Left, grid) {
        Ok(_) => p.apply_move(Move::Left),
        Err(_) => p,
    }
}

fn push_left_right(p: Point, m: Move, grid: &mut Grid) -> Result<(), ()> {
    let mut pc = p.apply_move(m);
    let mut boxes = Vec::new();
    while let Some(object) = grid.get(&pc) {
        match object {
            Object::Wall => {
                return Err(());
            }
            Object::Box => {
                boxes.push(pc);
                pc = pc.apply_move(m).apply_move(m);
            }
        }
    }

    boxes.into_iter().for_each(|b| {
        grid.remove(&b);
        grid.insert(b.apply_move(m), Object::Box);
    });

    Ok(())
}

fn parse_input() -> (Point, Grid, Vec<Move>) {
    let mut grid = HashMap::new();
    let mut moves = Vec::new();
    let mut start = Point(0, 0);

    let mut lines = read_input!(15)
        .trim()
        .split("\n")
        .map(|line| line.trim())
        .enumerate();

    while let Some((i, line)) = lines.next() {
        if line.is_empty() {
            break;
        }

        line.trim()
            .chars()
            .enumerate()
            .filter(|(_, c)| *c != '.')
            .for_each(|(j, c)| {
                let p = Point(i as i64, j as i64);
                if c == '@' {
                    start = p;
                } else {
                    grid.insert(p, Object::from(c));
                }
            })
    }

    lines
        .map(|(_, line)| line.chars().map(|c| c.into()).collect::<Vec<Move>>())
        .for_each(|mut ms| moves.append(&mut ms));

    (start, grid, moves)
}

fn prepare_part2(start: Point, grid: &Grid) -> (Point, Grid) {
    let start = Point(start.0, 2 * start.1);
    let grid = grid
        .iter()
        .map(|(p, object)| (Point(p.0, 2 * p.1), *object))
        .collect();

    (start, grid)
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl From<char> for Move {
    fn from(m: char) -> Self {
        match m {
            '^' => Self::Up,
            '>' => Self::Right,
            'v' => Self::Down,
            '<' => Self::Left,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Object {
    Box,
    Wall,
}

impl From<char> for Object {
    fn from(object: char) -> Self {
        match object {
            '#' => Self::Wall,
            'O' => Self::Box,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Point(i64, i64);

impl Point {
    pub fn apply_move(&self, m: Move) -> Self {
        match m {
            Move::Up => Point(self.0 - 1, self.1),
            Move::Down => Point(self.0 + 1, self.1),
            Move::Left => Point(self.0, self.1 - 1),
            Move::Right => Point(self.0, self.1 + 1),
        }
    }
}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_i64((self.0 << 8) | self.1)
    }
}
