use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::vec::Vec;

use std::num::ParseIntError;

struct Node {
    metadata: Vec<u8>,
    children: Vec<Node>,
}

impl Node {
    fn new(input: &[u8]) -> (Self, &[u8]) {
        let n_children = input[0];
        let n_metadata = input[1] as usize;
        let (_, mut input) = input.split_at(2);
        let mut children = Vec::new();
        for _i in 0..n_children {
            let (child, rest_input) = Node::new(input);
            children.push(child);
            input = rest_input;
        }
        let (metadata_input, input) = input.split_at(n_metadata);
        let node = Node {
            metadata: metadata_input.to_vec(),
            children: children,
        };
        (node, input)
    }

    fn metadata_sum(&self) -> u32 {
        let own_sum: u32 = self.metadata.iter().map(|&m| m as u32).sum();
        let children_sums: u32 = self.children.iter().map(|c| c.metadata_sum()).sum();
        own_sum + children_sums
    }

    fn metadata_sum_part2(&self) -> u32 {
        if self.children.is_empty() {
            self.metadata.iter().map(|&m| m as u32).sum()
        } else {
            self.metadata
                .iter()
                .map(|&m| (m - 1) as usize)
                .filter(|&m| m < self.children.len())
                .map(|m| self.children[m].metadata_sum_part2())
                .sum()
        }
    }
}

impl FromStr for Node {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input: Vec<u8> = s.split(' ').map(|n| n.parse().unwrap()).collect();
        let (node, rest) = Node::new(&input);
        assert!(rest.is_empty());
        Ok(node)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let f = &args.get(1).expect("No input file given");
    let f = File::open(f).expect("File not found");
    let lines: Result<Vec<_>, _> = BufReader::new(f).lines().collect();
    let lines = lines.expect("Could not read lines from file");
    let tree: Node = lines[0].parse().unwrap();
    let result_1 = tree.metadata_sum();
    println!("Answer 1: {}", result_1);
    let result_2 = tree.metadata_sum_part2();
    println!("Answer 2: {}", result_2);
}
