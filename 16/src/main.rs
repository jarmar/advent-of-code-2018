use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

use std::num::ParseIntError;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
enum Opcode {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

const ALL_OPCODES: &'static [Opcode] = &[
    Opcode::Addr,
    Opcode::Addi,
    Opcode::Mulr,
    Opcode::Muli,
    Opcode::Banr,
    Opcode::Bani,
    Opcode::Borr,
    Opcode::Bori,
    Opcode::Setr,
    Opcode::Seti,
    Opcode::Gtir,
    Opcode::Gtri,
    Opcode::Gtrr,
    Opcode::Eqir,
    Opcode::Eqri,
    Opcode::Eqrr,
];

impl Opcode {
    fn is_a_reg(&self) -> bool {
        use Opcode::*;
        match &self {
            Seti => false,
            Gtir => false,
            Eqir => false,
            _    => true
        }
    }
    fn is_b_reg(&self) -> bool {
        use Opcode::*;
        match &self {
            Addi => false,
            Muli => false,
            Bani => false,
            Bori => false,
            Setr => false,
            Seti => false,
            Gtri => false,
            Eqri => false,
            _    => true
        }
    }
}

type MemState = [u32; 4];

fn exec(op: Opcode, a: u32, b: u32, c: u32, state: MemState) -> MemState {
    let mut res = state;
    let op_result = exec_helper(op, a, b, state);
    res[c as usize] = op_result;
    res
}

fn exec_helper(op: Opcode, a: u32, b: u32, state: MemState) -> u32 {
    use Opcode::*;
    assert!(a < 4 || !op.is_a_reg());
    assert!(b < 4 || !op.is_b_reg());
    let a_reg = if a < 4 { state[a as usize] } else { 129 };
    let b_reg = if b < 4 { state[b as usize] } else { 129 };
    match op {
        Addr => a_reg + b_reg,
        Addi => a_reg + b,
        Mulr => a_reg * b_reg,
        Muli => a_reg * b,
        Banr => a_reg & b_reg,
        Bani => a_reg & b,
        Borr => a_reg | b_reg,
        Bori => a_reg | b,
        Setr => a_reg,
        Seti => a,
        Gtir => (a > b_reg) as u32,
        Gtri => (a_reg > b) as u32,
        Gtrr => (a_reg > b_reg) as u32,
        Eqir => (a == b_reg) as u32,
        Eqri => (a_reg == b) as u32,
        Eqrr => (a_reg == b_reg) as u32,
    }
}

#[derive(Debug)]
struct Sample {
    before: MemState,
    after: MemState,
    op_number: u32,
    a: u32,
    b: u32,
    c: u32,
}

struct Instruction {
    op_number: u32,
    a: u32,
    b: u32,
    c: u32
}

impl FromStr for Instruction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut vals = s.split(' ').map(|num| num.parse::<u32>().unwrap());
        let op_number = vals.next().unwrap();
        let a = vals.next().unwrap();
        let b = vals.next().unwrap();
        let c = vals.next().unwrap();
        Ok(Instruction { op_number: op_number, a: a, b: b, c: c })
    }
}

fn parse_mem_line(line: &str) -> MemState {
    let nums: Vec<_> = line
        .rsplit('[')
        .next()
        .unwrap()
        .split(']')
        .next()
        .unwrap()
        .split(", ")
        .map(|num| num.parse::<u32>().unwrap())
        .collect();
    [nums[0], nums[1], nums[2], nums[3]]
}

impl Sample {
    fn from_lines<T: AsRef<str>>(lines: &[T]) -> Self {
        let mem_before = parse_mem_line(lines[0].as_ref());
        let instrs: Vec<_> = lines[1]
            .as_ref()
            .split(' ')
            .map(|num| num.parse::<u32>().unwrap())
            .collect();
        let mem_after = parse_mem_line(lines[2].as_ref());
        Sample {
            before: mem_before,
            after: mem_after,
            op_number: instrs[0],
            a: instrs[1],
            b: instrs[2],
            c: instrs[3],
        }
    }
}

fn match_sample(sample: &Sample, op: &Opcode) -> bool {
    let exec_result = exec(*op, sample.a, sample.b, sample.c, sample.before);
    exec_result == sample.after
}

fn sample_matches(sample: &Sample) -> Vec<Opcode> {
    ALL_OPCODES
        .iter()
        .filter(|&op| match_sample(sample, op))
        .map(|&op| op)
        .collect()
}

fn part1(samples: &[Sample]) -> usize {
    samples
        .iter()
        .map(|s| sample_matches(s).len())
        .filter(|&len| len >= 3)
        .count()
}

fn figure_out(samples: &[Sample]) -> Vec<Opcode> {
    let mut possible: Vec<HashSet<Opcode>> = (0..16)
        .map(|_| ALL_OPCODES.iter().map(|&op| op).collect())
        .collect();
    for sample in samples {
        let possible_insns = sample_matches(sample);
        let possible_insns_set: HashSet<Opcode> = possible_insns.iter().map(|&op| op).collect();
        possible[sample.op_number as usize] = possible[sample.op_number as usize]
            .intersection(&possible_insns_set)
            .map(|&op| op)
            .collect();
    }
    satisfy(&mut possible)
}

fn satisfy(sets: &mut Vec<HashSet<Opcode>>) -> Vec<Opcode> {
    let mut res = vec![Opcode::Addi; 16];
    for _i in 0..16 {
        let singleton_pos = sets.iter().position(|s| s.len() == 1).unwrap();
        let figured_code = sets[singleton_pos].drain().next().unwrap();
        for set in sets.iter_mut() {
            set.remove(&figured_code);
        }
        res[singleton_pos] = figured_code;
    }
    res
}

fn main() {
    let s = Sample {
        before: [3, 2, 1, 1],
        after: [3, 2, 2, 1],
        op_number: 9,
        a: 2,
        b: 1,
        c: 2,
    };
    let sm = sample_matches(&s);
    println!("{:?}", sm);
    let args: Vec<String> = env::args().collect();
    let f = &args.get(1).expect("No input file given");
    let f = File::open(f).expect("File not found");
    let lines: Result<Vec<_>, _> = BufReader::new(f).lines().collect();
    let lines = lines.expect("Could not read lines from file");
    let samples: Vec<_> = lines
        .chunks(4)
        .map(|chunk| Sample::from_lines(chunk))
        .collect();
    let result_1 = part1(&samples);
    println!("{}", result_1);
    let figured = figure_out(&samples);
    println!("{:?}", figured);
    let f2 = &args.get(2).expect("No input file given");
    let f2 = File::open(f2).expect("File not found");
    let lines2: Result<Vec<_>, _> = BufReader::new(f2).lines().collect();
    let lines2 = lines2.expect("Could not read lines from file");
    let instructions: Vec<_> = lines2
        .iter()
        .map(|line| line.parse::<Instruction>().unwrap())
        .collect();
    let mut mem_state: MemState = [0; 4];
    for instr in &instructions {
        let op = figured[instr.op_number as usize];
        mem_state = exec(op, instr.a, instr.b, instr.c, mem_state);
    }
    println!("{}", mem_state[0]);
}
