use crate::{read_input, Solution, SolutionPair};

pub fn solve() -> SolutionPair {
    let (mut registers, program) = parse_input();

    let sol1 = part1(&mut registers, &program);
    let sol2 = part2(&program);

    (Solution::from(sol1), Solution::from(sol2))
}

fn part1(registers: &mut [u64], program: &[u64]) -> String {
    let mut outputs = Vec::new();
    let mut i = 0;
    while i < program.len() {
        let (i_next, output) = run_opcode(i, program[i], program[i + 1], registers);
        if let Some(output) = output {
            outputs.push(output.to_string());
        }

        i = i_next;
    }

    outputs.join(",")
}

fn run_opcode(i: usize, opcode: u64, operand: u64, registers: &mut [u64]) -> (usize, Option<u64>) {
    let mut i_next = i + 2;
    let mut output = None;
    match opcode {
        0 => {
            let operand = get_combo_operand(operand, registers);
            let div = 1 << operand;
            registers[0] /= div;
        }
        1 => {
            registers[1] ^= operand;
        }
        2 => {
            let operand = get_combo_operand(operand, registers);
            registers[1] = operand % 8;
        }
        3 => {
            if registers[0] != 0 {
                i_next = operand as usize;
            }
        }
        4 => {
            registers[1] ^= registers[2];
        }
        5 => {
            let operand = get_combo_operand(operand, registers);
            output = Some(operand % 8);
        }
        6 => {
            let operand = get_combo_operand(operand, registers);
            let div = 1 << operand;
            registers[1] = registers[0] / div;
        }
        7 => {
            let operand = get_combo_operand(operand, registers);
            let div = 1 << operand;
            registers[2] = registers[0] / div;
        }
        _ => unreachable!(),
    }

    (i_next, output)
}

fn part2(program: &[u64]) -> u64 {
    let mut start = Vec::new();
    start.push(Number::default());
    program
        .iter()
        .enumerate()
        .fold(start, |acc, (i, t)| find_a(i, *t ^ 6, acc))
        .into_iter()
        .map(|number| u64::from(number))
        .min()
        .unwrap()
}

/// Find A
///
/// Let Am = A % 8, Ac = Am XOR 3, and target = prog_el XOR 6, then candidates for A are:
///
/// Am XOR ((target XOR Am) << Ac), if there are no conflicting bits between 2 outer values.
fn find_a(idx: usize, target: u64, previous_as: Vec<Number>) -> Vec<Number> {
    let mut r#as = Vec::new();
    for am in 0..8 {
        let mut a = Number::default();
        let mut offset = 3 * idx;
        for i in 0..3 {
            a.insert(Bit::ith(am, i), offset + i);
        }

        offset += (am ^ 3) as usize;
        let val = target ^ am;
        let mut is_valid = true;
        for i in 0..3 {
            let bit = Bit::ith(val, i);
            if !a.verify(bit, offset) {
                is_valid = false;
                break;
            }
            a.insert(bit, offset);
            offset += 1;
        }

        if is_valid {
            r#as.push(a);
        }
    }

    combine_as(previous_as, r#as)
}

fn combine_as(previous_as: Vec<Number>, r#as: Vec<Number>) -> Vec<Number> {
    let mut combined_as = Vec::new();
    for a in r#as {
        for prev_a in &previous_as {
            if let Ok(new_a) = a.combine(prev_a) {
                combined_as.push(new_a);
            }
        }
    }

    combined_as
}

fn get_combo_operand(operand: u64, registers: &[u64]) -> u64 {
    match operand {
        0..=3 => operand,
        4..=6 => registers[operand as usize - 4],
        _ => unreachable!(),
    }
}

fn parse_input() -> ([u64; 3], Vec<u64>) {
    let mut lines = read_input!(17)
        .trim()
        .split("\n")
        .map(|line| line.trim())
        .filter(|line| !line.is_empty());

    let mut registers = [0, 0, 0];
    for i in 0..3 {
        let register = lines
            .next()
            .unwrap()
            .split(": ")
            .skip(1)
            .next()
            .unwrap()
            .parse()
            .unwrap();

        registers[i] = register;
    }

    let program = lines
        .next()
        .unwrap()
        .split(": ")
        .skip(1)
        .next()
        .unwrap()
        .split(",")
        .map(|c| c.parse().unwrap())
        .collect();

    (registers, program)
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
enum Bit {
    #[default]
    Any,
    One,
    Zero,
}

impl Bit {
    fn ith(x: u64, i: usize) -> Self {
        let b = (x >> i) & 1;
        Bit::from(b)
    }
}

impl From<u64> for Bit {
    fn from(value: u64) -> Self {
        match value {
            0 => Self::Zero,
            1 => Self::One,
            _ => panic!("Unsupported!"),
        }
    }
}

impl From<Bit> for u64 {
    fn from(bit: Bit) -> Self {
        match bit {
            Bit::Any | Bit::Zero => 0,
            Bit::One => 1,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Number([Bit; 64]);

impl Number {
    pub fn insert(&mut self, bit: Bit, offset: usize) {
        self.0[offset] = bit;
    }

    pub fn verify(&self, bit: Bit, offset: usize) -> bool {
        let existing = self.0[offset];
        existing == Bit::Any || existing == bit
    }

    pub fn combine(&self, other: &Number) -> Result<Number, ()> {
        let mut number = self.clone();
        for (i, b) in self.0.iter().enumerate() {
            match b {
                Bit::Any => {
                    number.0[i] = other.0[i];
                }
                Bit::One => {
                    if other.0[i] == Bit::Zero {
                        return Err(());
                    }
                }
                Bit::Zero => {
                    if other.0[i] == Bit::One {
                        return Err(());
                    }
                }
            }
        }

        Ok(number)
    }
}

impl Default for Number {
    fn default() -> Self {
        Number([Bit::default(); 64])
    }
}

impl From<Number> for u64 {
    fn from(number: Number) -> Self {
        number
            .0
            .iter()
            .rev()
            .fold(0u64, |acc, bit| (acc << 1) + u64::from(*bit))
    }
}
