use std::collections::{HashMap, HashSet};

use crate::{read_input, Solution, SolutionPair};

pub fn solve() -> SolutionPair {
    let (rules, pages) = parse_input();
    let (sol1, sol2) = parts(&rules, &pages);

    (Solution::from(sol1), Solution::from(sol2))
}

fn parts(rules: &HashMap<u8, HashSet<u8>>, pages: &Vec<Vec<u8>>) -> (u32, u32) {
    let mut result_1 = 0;
    let mut result_2 = 0;
    for page in pages {
        let mut els: HashSet<u8> = HashSet::from_iter(page.iter().cloned());
        if verify_page(page, &els, rules) {
            result_1 += page[page.len() / 2] as u32;
        } else {
            let el = generate_sorted_mid(&mut els, rules);
            result_2 += el as u32;
        }
    }

    (result_1, result_2)
}

fn verify_page(page: &Vec<u8>, els: &HashSet<u8>, rules: &HashMap<u8, HashSet<u8>>) -> bool {
    let mut visited = HashSet::new();
    for el in page {
        if !verify_el(*el, &visited, els, rules) {
            return false;
        }
        visited.insert(*el);
    }

    true
}

fn verify_el(
    el: u8,
    visited: &HashSet<u8>,
    els: &HashSet<u8>,
    rules: &HashMap<u8, HashSet<u8>>,
) -> bool {
    if let Some(reqs) = rules.get(&el) {
        for req in reqs {
            if els.contains(req) && !visited.contains(req) {
                return false;
            }
        }
    }

    true
}

fn generate_sorted_mid(els: &mut HashSet<u8>, rules: &HashMap<u8, HashSet<u8>>) -> u8 {
    let mut el = 0;
    for _ in 0..=(els.len() / 2) {
        el = get_next_el(els, rules);
        els.remove(&el);
    }

    el
}

fn get_next_el(els: &HashSet<u8>, rules: &HashMap<u8, HashSet<u8>>) -> u8 {
    for el in els {
        if let Some(reqs) = rules.get(el) {
            if els.is_disjoint(reqs) {
                return *el;
            }
        } else {
            return *el;
        }
    }

    unreachable!()
}

fn parse_input() -> (HashMap<u8, HashSet<u8>>, Vec<Vec<u8>>) {
    let mut rules: HashMap<u8, HashSet<u8>> = HashMap::new();

    let split: Vec<&str> = read_input!(05).split("\n\n").take(2).collect();
    for (el, req) in split[0].split("\n").map(|line| line.trim()).map(|line| {
        let mut line_split = line.split("|");
        let left = line_split.next().unwrap().parse::<u8>().unwrap();
        let right = line_split.next().unwrap().parse::<u8>().unwrap();
        (right, left)
    }) {
        if let Some(reqs) = rules.get_mut(&el) {
            reqs.insert(req);
        } else {
            rules.insert(el, HashSet::from([req]));
        }
    }

    let pages = split[1]
        .split("\n")
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split(",")
                .map(|el| el.parse::<u8>().unwrap())
                .collect()
        })
        .collect();

    (rules, pages)
}
