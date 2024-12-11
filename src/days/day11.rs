use hashbrown::HashMap;

use crate::{read_input, Solution, SolutionPair};

pub fn solve() -> SolutionPair {
    let stones = parse_input();

    let sol1 = part1(&stones);
    let sol2 = part2(&stones);

    (Solution::from(sol1), Solution::from(sol2))
}

fn part1(stones: &[u64]) -> usize {
    get_count(25, stones)
}

fn part2(stones: &[u64]) -> usize {
    get_count(75, stones)
}

fn get_count(blinks: usize, stones: &[u64]) -> usize {
    let mut cur_stones = HashMap::new();
    for stone in stones {
        add_map(*stone, 1, &mut cur_stones);
    }

    for _ in 0..blinks {
        let mut stones_new = HashMap::new();
        for (stone, count) in cur_stones {
            if stone == 0 {
                add_map(1, count, &mut stones_new);
            } else {
                let num_digits = stone.ilog10() + 1;
                if num_digits % 2 == 0 {
                    let div = 10u64.pow(num_digits / 2);
                    add_map(stone / div, count, &mut stones_new);
                    add_map(stone % div, count, &mut stones_new);
                } else {
                    add_map(stone * 2024, count, &mut stones_new);
                }
            }
        }

        cur_stones = stones_new;
    }

    cur_stones.values().sum()
}

fn add_map(key: u64, value: usize, map: &mut HashMap<u64, usize>) {
    if let Some(map_value) = map.get_mut(&key) {
        *map_value += value;
    } else {
        map.insert(key, value);
    }
}

fn parse_input() -> Vec<u64> {
    read_input!(11)
        .trim()
        .split(" ")
        .map(|x| x.parse::<u64>().unwrap())
        .collect()
}
