use hashbrown::HashMap;

use crate::{read_input, Solution, SolutionPair};

pub fn solve() -> SolutionPair {
    let (towels, designs) = parse_input();

    let scores: Vec<usize> = designs
        .iter()
        .map(|design| count_combinations(&design, &towels, &mut HashMap::new()))
        .collect();

    let sol1 = part1(&scores);
    let sol2 = part2(&scores);

    (Solution::from(sol1), Solution::from(sol2))
}

fn part1(scores: &[usize]) -> usize {
    scores.iter().filter(|score| **score > 0).count()
}

fn part2(scores: &[usize]) -> usize {
    scores.iter().sum()
}

fn count_combinations<'a>(
    design: &'a str,
    towels: &[&str],
    seen: &mut HashMap<&'a str, usize>,
) -> usize {
    if design.is_empty() {
        return 1;
    } else if let Some(count) = seen.get(design) {
        return *count;
    }

    let mut count = 0;
    for &towel in towels {
        if design.starts_with(towel) {
            count += count_combinations(&design[towel.len()..], towels, seen);
        }
    }

    seen.insert(design, count);

    count
}

fn parse_input<'a>() -> (Vec<&'a str>, Vec<&'a str>) {
    let mut lines = read_input!(19)
        .trim()
        .split("\n")
        .map(|line| line.trim())
        .filter(|line| !line.is_empty());

    let towels = lines.next().unwrap().split(", ").collect();

    let designs = lines.collect();

    (towels, designs)
}
