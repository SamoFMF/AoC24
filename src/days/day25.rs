use crate::{read_input, Solution, SolutionPair};

type KeyLock = [u8; 5];

pub fn solve() -> SolutionPair {
    let (locks, keys) = parse_input();

    let sol1 = part1(&locks, &keys);
    let sol2: u64 = 0;

    (Solution::from(sol1), Solution::from(sol2))
}

fn part1(locks: &[KeyLock], keys: &[KeyLock]) -> usize {
    keys.iter()
        .map(|key| {
            locks
                .iter()
                .filter(|lock| verify_key_lock(key, *lock))
                .count()
        })
        .sum()
}

fn verify_key_lock(key: &KeyLock, lock: &KeyLock) -> bool {
    for i in 0..key.len() {
        if key[i] + lock[i] > 5 {
            return false;
        }
    }

    true
}

fn parse_input() -> (Vec<KeyLock>, Vec<KeyLock>) {
    let mut locks = Vec::new();
    let mut keys = Vec::new();

    let mut lines = read_input!(25).split("\n").map(|line| line.trim());

    while let Some(line) = lines.next() {
        let mut key_lock = [0; 5];
        for _ in 0..5 {
            let line = lines.next().unwrap();
            parse_line(line, &mut key_lock);
        }

        lines.next().unwrap();
        lines.next().unwrap();

        if line.starts_with('#') {
            locks.push(key_lock);
        } else {
            keys.push(key_lock);
        }
    }

    (locks, keys)
}

fn parse_line(line: &str, key_lock: &mut [u8; 5]) {
    for (i, c) in line.chars().enumerate() {
        if c == '#' {
            key_lock[i] += 1;
        }
    }
}
