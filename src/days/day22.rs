use hashbrown::{HashMap, HashSet};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{read_input, Solution, SolutionPair};

const S_MAX: usize = 2000;

pub fn solve() -> SolutionPair {
    let secrets = parse_input();

    let sol1 = part1(&secrets);
    let sol2 = part2(&secrets);

    (Solution::from(sol1), Solution::from(sol2))
}

fn part1(secrets: &[i64]) -> i64 {
    secrets
        .par_iter()
        .map(|secret| iterate_secret(*secret))
        .sum()
}

fn iterate_secret(mut secret: i64) -> i64 {
    for _ in 0..S_MAX {
        secret = next_secret(secret);
    }

    secret
}

fn part2(secrets: &[i64]) -> i64 {
    let mut sequences = HashMap::new();
    secrets
        .iter()
        .for_each(|&secret| update_sequences(secret, &mut sequences));

    *sequences.values().max().unwrap()
}

fn update_sequences(secret: i64, sequences: &mut HashMap<u32, i64>) {
    let mut seen = HashSet::new();
    let mut sequence = 0;

    let mut prev_secret = secret;
    for i in 0..S_MAX {
        let secret = next_secret(prev_secret);
        let delta = (secret % 10) - (prev_secret % 10);
        let delta = (delta + 9) as u32; // Delta between 0 & 18 inclusive (5 bits)

        sequence <<= 5;
        sequence += delta;
        sequence &= 0xFFFFF; // Only keep first 5 bits
        if i > 2 {
            if !seen.contains(&sequence) {
                *sequences.entry(sequence).or_insert(0) += secret % 10;
                seen.insert(sequence);
            }
        }

        prev_secret = secret;
    }
}

fn next_secret(mut secret: i64) -> i64 {
    secret ^= secret * 64; // mix
    secret %= 16777216; // prune

    secret ^= secret / 32;
    secret %= 16777216;

    secret ^= secret * 2048;
    secret % 16777216
}

fn parse_input() -> Vec<i64> {
    read_input!(22)
        .trim()
        .split("\n")
        .map(|line| line.trim().parse::<i64>().unwrap())
        .collect()
}
