use std::cell::RefCell;
use std::collections::VecDeque;
use std::hash::Hash;
use std::rc::Rc;

use hashbrown::HashMap;

use crate::{read_input, Solution, SolutionPair};

type WireName = &'static str;

pub fn solve() -> SolutionPair {
    let (init, mut gates) = parse_input();

    let sol1 = part1(&init, &gates);
    let sol2 = part2(&mut gates);

    (Solution::from(sol1), Solution::from(sol2))
}

fn part1(init: &[(WireName, u8)], gates: &[Gate]) -> u64 {
    let mut undetermined = HashMap::new();
    for gate in gates {
        let gate_shared = Rc::new(RefCell::new(gate.clone()));
        for wire in gate.get_undetermined() {
            undetermined
                .entry(wire)
                .or_insert(Vec::new())
                .push(Rc::clone(&gate_shared));
        }
    }

    let mut queue = VecDeque::new();
    for wire in init {
        queue.push_back(*wire);
    }

    let mut results = Vec::new();
    while !queue.is_empty() {
        let (wire, value) = queue.pop_front().unwrap();

        if let Some(gates) = undetermined.remove(wire) {
            let mut gates_next = Vec::new();
            for gate in gates {
                let mut gate_mut = gate.borrow_mut();
                gate_mut.insert(wire, value);

                if let Some(result) = gate_mut.calculate() {
                    queue.push_back((gate_mut.gout, result));

                    if gate_mut.gout.starts_with("z") {
                        results.push((gate_mut.gout, result));
                    }
                } else {
                    gates_next.push(Rc::clone(&gate));
                }
            }

            if gates_next.len() > 0 {
                undetermined.insert(wire, gates_next);
            }
        }
    }

    results.sort();

    as_number(&results)
}

fn part2(gates: &mut [Gate]) -> String {
    let mut swaps = Vec::with_capacity(8);
    while let Some((w0, w1)) = get_swap_wires(gates) {
        swaps.push(w0);
        swaps.push(w1);

        for gate in &mut *gates {
            if gate.gout == w0 {
                gate.gout = w1;
            } else if gate.gout == w1 {
                gate.gout = w0;
            }
        }
    }

    swaps.sort();
    swaps.join(",")
}

/// Get Wires for Swapping
///
/// This takes a lot of assumptions about the input and does not cover all cases.
///
/// We assume that the circuit is a simple adder and that `i`th bit & corresponding
/// carry-over are always calculated using the same gate structure:
///
/// `z_i = (x_i ^ y_i) ^ c_i`, where `c_i = (x_i ^ y_i) | ((x_i ^ y_i) & c_{i-1})`
/// is the carry-over.
///
/// Edge cases are `z_0 = x_0 ^ y_0` and `z_45 = c_44` and are not covered by the current
/// implementation. They should be simple to add though as they are simplified versions
/// of the inner bits.
fn get_swap_wires(gates: &[Gate]) -> Option<(WireName, WireName)> {
    let x_xor_y: HashMap<_, _> = gates
        .iter()
        .filter(|gate| {
            if gate.op != Operation::Xor {
                return false;
            }

            if let Wire::Name(wire1) = gate.gin.0 {
                if let Wire::Name(wire2) = gate.gin.1 {
                    return ((wire1.starts_with('x') && wire2.starts_with('y'))
                        || (wire1.starts_with('y') && wire2.starts_with('x')))
                        && &wire1[1..] == &wire2[1..];
                }
            }

            false
        })
        .map(|gate| (gate.gout, &gate.get_undetermined()[0][1..]))
        .collect();

    let mut to_fix: Vec<_> = gates
        .iter()
        .filter(|gate| gate.gout.starts_with('z'))
        .filter(|gate| gate.gout != "z45" && gate.gout != "z00")
        .filter(|gate| {
            gate.op != Operation::Xor
                || !gate.get_undetermined().iter().any(|wire| {
                    if let Some(&xy) = x_xor_y.get(wire) {
                        return xy == &gate.gout[1..];
                    }

                    false
                })
        })
        .collect();
    to_fix.sort_by(|g1, g2| g1.gout.cmp(g2.gout));
    if to_fix.len() == 0 {
        return None;
    }

    let zi = *to_fix.first().unwrap();
    let gates1: HashMap<_, _> = gates
        .iter()
        .map(|gate| {
            let mut wires = gate.get_undetermined();
            wires.sort();

            (
                (wires[0].to_string(), wires[1].to_string(), gate.op),
                gate.gout,
            )
        })
        .collect();

    let mut cs = Vec::with_capacity(45); // carry-overs
    cs.push(
        *gates1
            .get(&("x00".to_string(), "y00".to_string(), Operation::And))
            .unwrap(),
    );

    for i in 1..45 {
        let xor = if let Some(wire) =
            gates1.get(&(format!("x{i:02}"), format!("y{i:02}"), Operation::Xor))
        {
            *wire
        } else {
            break;
        };

        let cp = cs[i - 1];
        let mut wires = vec![xor, cp];
        wires.sort();
        let and = if let Some(wire) =
            gates1.get(&(wires[0].to_string(), wires[1].to_string(), Operation::And))
        {
            *wire
        } else {
            break;
        };

        let xy = if let Some(wire) =
            gates1.get(&(format!("x{i:02}"), format!("y{i:02}"), Operation::And))
        {
            *wire
        } else {
            break;
        };

        let mut c = vec![xy, and];
        c.sort();

        let c = if let Some(wire) = gates1.get(&(c[0].to_string(), c[1].to_string(), Operation::Or))
        {
            *wire
        } else {
            break;
        };

        cs.push(c);
    }

    let i = (&zi.gout[1..]).parse::<usize>().unwrap();
    assert!(i <= cs.len());

    let (&cur, _) = x_xor_y
        .iter()
        .find(|(_, j)| **j == format!("{i:02}"))
        .unwrap();
    let c = cs[i - 1];

    let z_wires = zi.get_undetermined();
    if z_wires.contains(&cur) {
        if z_wires.contains(&c) {
            if zi.op == Operation::Xor {
                panic!("This should never happen!");
            }
        } else {
            let w = *z_wires.iter().find(|w| **w != cur).unwrap();
            return Some((c, w));
        }
    } else if z_wires.contains(&c) {
        let w = *z_wires.iter().find(|w| **w != c).unwrap();
        return Some((cur, w));
    }

    let mut wires = vec![cur, c];
    wires.sort();
    if let Some(wire) = gates1.get(&(wires[0].to_string(), wires[1].to_string(), Operation::Xor)) {
        if *wire != zi.gout {
            return Some((zi.gout, wire));
        }
    }

    None // never happens for my input, but some cases are not covered, e.g. issue with z45
}

fn as_number(bits: &[(WireName, u8)]) -> u64 {
    bits.iter()
        .rev()
        .fold(0, |acc, (_, v)| acc * 2 + (*v as u64))
}

fn parse_input() -> (Vec<(WireName, u8)>, Vec<Gate>) {
    let mut init_values = Vec::new();
    let mut gates = Vec::new();

    let mut lines = read_input!(24).trim().split("\n").map(|line| line.trim());

    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            break;
        }

        let mut split = line.split(": ");
        init_values.push((
            split.next().unwrap(),
            split.next().unwrap().parse::<u8>().unwrap(),
        ));
    }

    for line in lines {
        gates.push(Gate::new(line));
    }

    (init_values, gates)
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Gate {
    gin: (Wire, Wire),
    gout: WireName,
    op: Operation,
}

impl Gate {
    pub fn new(line: &'static str) -> Self {
        let mut split = line.split(" ");
        let gin1 = split.next().unwrap();
        let op = Operation::from(split.next().unwrap());
        let gin2 = split.next().unwrap();
        split.next().unwrap();
        let gout = split.next().unwrap();

        Self {
            gin: (Wire::Name(gin1), Wire::Name(gin2)),
            gout,
            op,
        }
    }

    pub fn get_undetermined(&self) -> Vec<WireName> {
        let mut wires = Vec::new();
        if let Wire::Name(wire) = self.gin.0 {
            wires.push(wire);
        }

        if let Wire::Name(wire) = self.gin.1 {
            wires.push(wire);
        }

        wires
    }

    pub fn insert(&mut self, wire: WireName, value: u8) {
        if let Wire::Name(name) = self.gin.0 {
            if name == wire {
                self.gin.0 = Wire::Value(value);
            }
        }

        if let Wire::Name(name) = self.gin.1 {
            if name == wire {
                self.gin.1 = Wire::Value(value);
            }
        }
    }

    pub fn calculate(&self) -> Option<u8> {
        if let Wire::Value(v0) = self.gin.0 {
            if let Wire::Value(v1) = self.gin.1 {
                return Some(self.op.run(v0, v1));
            }
        }

        None
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Wire {
    Name(WireName),
    Value(u8),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Operation {
    And,
    Or,
    Xor,
}

impl Operation {
    pub fn run(&self, v0: u8, v1: u8) -> u8 {
        match self {
            Operation::And => v0 & v1,
            Operation::Or => v0 | v1,
            Operation::Xor => v0 ^ v1,
        }
    }
}

impl From<&'static str> for Operation {
    fn from(value: &'static str) -> Self {
        match value {
            "AND" => Self::And,
            "OR" => Self::Or,
            "XOR" => Self::Xor,
            _ => unreachable!(),
        }
    }
}
