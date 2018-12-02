use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_changes(f: File) -> Vec<i64> {
    let mut changes = Vec::new();
    for line in BufReader::new(f).lines() {
        let change = line
            .expect("Could not read line")
            .parse()
            .expect("Could not parse line");
        changes.push(change);
    }
    changes
}

fn part2(changes: &Vec<i64>) -> i64 {
    let mut seen: HashSet<i64> = HashSet::new();
    let mut frequency = 0;
    for change in changes.iter().cycle() {
        frequency += change;
        if !seen.insert(frequency) {
            break;
        }
    }
    frequency
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let f = &args.get(1).expect("No input file given");
    let f = File::open(f).expect("File not found");
    let changes = read_changes(f);
    let result_1: i64 = changes.iter().sum();
    println!("Answer 1: {}", result_1);
    let result_2 = part2(&changes);
    println!("Answer 2: {}", result_2);
}
