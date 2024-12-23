use hashbrown::{HashMap, HashSet};

use crate::{read_input, Solution, SolutionPair};

pub fn solve() -> SolutionPair {
    let graph = parse_input();

    let sol1 = part1(&graph);
    let sol2 = part2c(&graph);

    (Solution::from(sol1), Solution::from(sol2))
}

fn part1(graph: &HashMap<&'static str, HashSet<&'static str>>) -> usize {
    let mut clique3 = HashSet::new();
    for (&v0, ns) in graph {
        for &v1 in ns {
            for &v2 in ns.intersection(graph.get(v1).unwrap()) {
                if v0.starts_with('t') || v1.starts_with('t') || v2.starts_with('t') {
                    let mut set = vec![v0, v1, v2];
                    set.sort();
                    clique3.insert(set);
                }
            }
        }
    }

    clique3.len()
}

fn part2c(graph: &HashMap<&'static str, HashSet<&'static str>>) -> String {
    let p = graph.keys().map(|v| *v).collect();
    let mut r = HashSet::new();
    let x = HashSet::new();

    let best = get_maximum_clique(&mut r, p, x, graph);
    let mut best = Vec::from_iter(best.iter().map(|v| *v));
    best.sort();

    best.join(",")
}

// Bron Kerbosch Algorithm
fn get_maximum_clique(
    r: &mut HashSet<&'static str>,
    mut p: HashSet<&'static str>,
    mut x: HashSet<&'static str>,
    graph: &HashMap<&'static str, HashSet<&'static str>>,
) -> HashSet<&'static str> {
    if p.is_empty() && x.is_empty() {
        return r.clone();
    }

    let u = p.union(&x).map(|v| *v).next().unwrap(); // pivot
    let p1 = &p - graph.get(u).unwrap();

    let mut best: Option<HashSet<&'static str>> = None;
    for v in p1 {
        r.insert(v);
        let nv = graph.get(v).unwrap();
        let result = get_maximum_clique(
            r,
            p.intersection(nv).map(|v| *v).collect(),
            x.intersection(nv).map(|v| *v).collect(),
            graph,
        );
        p.remove(v);
        x.remove(v);
        r.remove(v);

        if best.is_none() || best.as_ref().unwrap().len() < result.len() {
            best = Some(result);
        }
    }

    best.unwrap_or_default()
}

fn parse_input() -> HashMap<&'static str, HashSet<&'static str>> {
    let mut graph = HashMap::new();

    read_input!(23)
        .trim()
        .split("\n")
        .map(|line| line.trim())
        .for_each(|line| {
            let mut edge = line.split("-");
            let u = edge.next().unwrap();
            let v = edge.next().unwrap();

            graph.entry(u).or_insert(HashSet::new()).insert(v);
            graph.entry(v).or_insert(HashSet::new()).insert(u);
        });

    graph
}
