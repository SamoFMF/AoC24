use std::fmt::{Debug, Formatter, Write};

use hashbrown::HashMap;

use crate::{read_input, Solution, SolutionPair};

const NUMERIC_KEYPAD: [[NumericKey; 3]; 4] = [
    [
        NumericKey::Number(7),
        NumericKey::Number(8),
        NumericKey::Number(9),
    ],
    [
        NumericKey::Number(4),
        NumericKey::Number(5),
        NumericKey::Number(6),
    ],
    [
        NumericKey::Number(1),
        NumericKey::Number(2),
        NumericKey::Number(3),
    ],
    [NumericKey::Gap, NumericKey::Number(0), NumericKey::Activate],
];

const DIRECTIONAL_KEYPAD: [[DirectionalKey; 3]; 2] = [
    [
        DirectionalKey::Gap,
        DirectionalKey::Up,
        DirectionalKey::Activate,
    ],
    [
        DirectionalKey::Left,
        DirectionalKey::Down,
        DirectionalKey::Right,
    ],
];

const KEYS: [DirectionalKey; 5] = [
    DirectionalKey::Up,
    DirectionalKey::Down,
    DirectionalKey::Left,
    DirectionalKey::Right,
    DirectionalKey::Activate,
];

pub fn solve() -> SolutionPair {
    let codes = parse_input();

    let sol1 = part1(&codes);
    let sol2 = part2(&codes);

    (Solution::from(sol1), Solution::from(sol2))
}

fn part1(codes: &Vec<Vec<NumericKey>>) -> usize {
    let lengths = get_sequence_lengths(2);
    get_result(codes, &lengths)
}

fn part2(codes: &Vec<Vec<NumericKey>>) -> usize {
    let lengths = get_sequence_lengths(25);
    get_result(codes, &lengths)
}

fn get_result(
    codes: &Vec<Vec<NumericKey>>,
    lengths: &HashMap<DirectionalKey, HashMap<DirectionalKey, usize>>,
) -> usize {
    codes
        .iter()
        .map(|code| get_shortest_path_len(code, &lengths) * get_numeric_part(code))
        .sum()
}

fn get_shortest_path_len(
    code: &[NumericKey],
    lengths: &HashMap<DirectionalKey, HashMap<DirectionalKey, usize>>,
) -> usize {
    let mut result = 0;
    let mut cur = DirectionalKey::Activate;
    for key in get_shortest_path_numeric1(code) {
        result += lengths.get(&cur).unwrap().get(&key).unwrap();
        cur = key;
    }

    result
}

fn get_sequence_lengths(n: usize) -> HashMap<DirectionalKey, HashMap<DirectionalKey, usize>> {
    let mut lengths = HashMap::new();
    for from_key in KEYS {
        let mut key_lengths = HashMap::new();
        for to_key in KEYS {
            key_lengths.insert(to_key, 1);
        }
        lengths.insert(from_key, key_lengths);
    }

    let mut shortest_paths = HashMap::new();

    for _ in 0..n {
        let mut next = HashMap::new();
        for from_key in KEYS {
            let mut key_lengths = HashMap::new();
            for to_key in KEYS {
                let path = shortest_paths
                    .entry((from_key, to_key))
                    .or_insert_with(|| get_shortest_path_directional1(from_key, to_key));

                let mut cur_key = DirectionalKey::Activate;
                let mut dist = 0;
                for key in path {
                    dist += lengths.get(&cur_key).unwrap().get(key).unwrap();
                    cur_key = *key;
                }
                key_lengths.insert(to_key, dist);
            }

            next.insert(from_key, key_lengths);
        }

        lengths = next;
    }

    lengths
}

fn get_numeric_part(code: &[NumericKey]) -> usize {
    *&code[..3]
        .iter()
        .map(|key| u8::from(*key))
        .fold(0, |acc, n| acc * 10 + (n as usize))
}

fn get_shortest_path_directional1(
    start: DirectionalKey,
    end: DirectionalKey,
) -> Vec<DirectionalKey> {
    let ps = find_position_directional(start);
    let pe = find_position_directional(end);

    let dir0 = if pe.0 < ps.0 {
        DirectionalKey::Up
    } else {
        DirectionalKey::Down
    };
    let dir0_len = pe.0.abs_diff(ps.0);
    let dir1 = if pe.1 < ps.1 {
        DirectionalKey::Left
    } else {
        DirectionalKey::Right
    };
    let dir1_len = pe.1.abs_diff(ps.1);

    let (dir0, dir1, dir0_len, dir1_len) = if dir1 == DirectionalKey::Left {
        (dir0, dir1, dir0_len, dir1_len)
    } else {
        (dir1, dir0, dir1_len, dir0_len)
    };

    let mut dirs = Vec::with_capacity(dir0_len + dir1_len + 1);
    dirs.extend(std::iter::repeat(dir1).take(dir1_len));
    dirs.extend(std::iter::repeat(dir0).take(dir0_len));
    dirs.push(DirectionalKey::Activate);

    let mut dirs2 = Vec::with_capacity(dir0_len + dir1_len + 1);
    dirs2.extend(std::iter::repeat(dir0).take(dir0_len));
    dirs2.extend(std::iter::repeat(dir1).take(dir1_len));
    dirs2.push(DirectionalKey::Activate);

    vec![dirs, dirs2]
        .into_iter()
        .filter(|path| verify_path_directional(ps, path))
        .next()
        .unwrap()
}

fn get_shortest_path_numeric1(code: &[NumericKey]) -> Vec<DirectionalKey> {
    let mut pc = (3, 2);
    let mut path = Vec::new();

    for key in code {
        let pn = find_position_numeric(*key);
        let dir0 = if pn.0 < pc.0 {
            DirectionalKey::Up
        } else {
            DirectionalKey::Down
        };
        let dir0_len = pn.0.abs_diff(pc.0);
        let dir1 = if pn.1 < pc.1 {
            DirectionalKey::Left
        } else {
            DirectionalKey::Right
        };
        let dir1_len = pn.1.abs_diff(pc.1);

        let (dir0, dir1, dir0_len, dir1_len) = if dir1 == DirectionalKey::Left {
            (dir0, dir1, dir0_len, dir1_len)
        } else {
            (dir1, dir0, dir1_len, dir0_len)
        };

        let mut dirs = Vec::with_capacity(dir0_len + dir1_len + 1);
        dirs.extend(std::iter::repeat(dir1).take(dir1_len));
        dirs.extend(std::iter::repeat(dir0).take(dir0_len));
        dirs.push(DirectionalKey::Activate);

        let mut dirs2 = Vec::with_capacity(dir0_len + dir1_len + 1);
        dirs2.extend(std::iter::repeat(dir0).take(dir0_len));
        dirs2.extend(std::iter::repeat(dir1).take(dir1_len));
        dirs2.push(DirectionalKey::Activate);

        let mut to_add = vec![dirs, dirs2]
            .into_iter()
            .filter(|path| verify_path_numeric(pc, path))
            .next()
            .unwrap();

        path.append(&mut to_add);
        pc = pn;
    }

    path
}

fn verify_path_numeric(mut cur: (usize, usize), path: &Vec<DirectionalKey>) -> bool {
    for dir in path {
        cur = match dir {
            DirectionalKey::Up => (cur.0 - 1, cur.1),
            DirectionalKey::Down => (cur.0 + 1, cur.1),
            DirectionalKey::Left => (cur.0, cur.1 - 1),
            DirectionalKey::Right => (cur.0, cur.1 + 1),
            _ => cur,
        };

        if NUMERIC_KEYPAD[cur.0][cur.1] == NumericKey::Gap {
            return false;
        }
    }

    true
}

fn verify_path_directional(mut cur: (usize, usize), path: &Vec<DirectionalKey>) -> bool {
    for dir in path {
        cur = match dir {
            DirectionalKey::Up => (cur.0 - 1, cur.1),
            DirectionalKey::Down => (cur.0 + 1, cur.1),
            DirectionalKey::Left => (cur.0, cur.1 - 1),
            DirectionalKey::Right => (cur.0, cur.1 + 1),
            _ => cur,
        };

        if DIRECTIONAL_KEYPAD[cur.0][cur.1] == DirectionalKey::Gap {
            return false;
        }
    }

    true
}

fn find_position_numeric(key: NumericKey) -> (usize, usize) {
    for i in 0..4 {
        for j in 0..3 {
            if NUMERIC_KEYPAD[i][j] == key {
                return (i, j);
            }
        }
    }

    unreachable!("key = {key:?}")
}

fn find_position_directional(key: DirectionalKey) -> (usize, usize) {
    for i in 0..2 {
        for j in 0..3 {
            if DIRECTIONAL_KEYPAD[i][j] == key {
                return (i, j);
            }
        }
    }

    unreachable!("key = {key:?}")
}

fn parse_input() -> Vec<Vec<NumericKey>> {
    read_input!(21)
        .trim()
        .split("\n")
        .map(|line| line.trim())
        .map(|line| line.bytes().map(|byte| NumericKey::from(byte)).collect())
        .collect()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum NumericKey {
    Number(u8),
    Activate,
    Gap,
}

impl From<NumericKey> for u8 {
    fn from(key: NumericKey) -> Self {
        match key {
            NumericKey::Number(n) => n,
            _ => unimplemented!("key = {key:?}"),
        }
    }
}

impl From<u8> for NumericKey {
    fn from(byte: u8) -> Self {
        match byte {
            48..=57 => Self::Number(byte - 48),
            65 => Self::Activate,
            _ => unreachable!("byte = {byte}"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum DirectionalKey {
    Up,
    Down,
    Left,
    Right,
    Activate,
    Gap,
}

impl Debug for DirectionalKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DirectionalKey::Up => f.write_char('^'),
            DirectionalKey::Down => f.write_char('v'),
            DirectionalKey::Left => f.write_char('<'),
            DirectionalKey::Right => f.write_char('>'),
            DirectionalKey::Activate => f.write_char('A'),
            DirectionalKey::Gap => unreachable!(),
        }
    }
}
