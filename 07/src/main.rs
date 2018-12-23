#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::env;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec::Vec;
use std::str::FromStr;

use regex::Regex;

#[derive(Eq)]
struct Node {
    name: char,
    incoming: Vec<char>
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} <- {:?}", self.name, self.incoming)
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        self.name.eq(&other.name)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<Ordering> {
        Some(self.name.partial_cmp(&other.name)?.reverse())
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Node) -> Ordering {
        self.name.cmp(&other.name).reverse()
    }
}

#[derive(PartialEq, Eq, Hash)]
struct Edge {
    before: char,
    after: char
}

impl FromStr for Edge {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref edge_regex: Regex =
                Regex::new("Step (?P<before>.) must be finished before step (?P<after>.) can begin.")
                    .unwrap();
        }
        let captures = edge_regex.captures(s).ok_or("Could not parse line")?;
        let before = captures["before"].chars().next().unwrap();
        let after = captures["after"].chars().next().unwrap();
        Ok(Edge { before: before, after: after })
    }
}

fn create_dag(edges: &[Edge]) -> HashMap<char, Node> {
    let mut nodes: HashMap<char, Node> = HashMap::new();
    for edge in edges {
        nodes.entry(edge.before).or_insert(Node { name: edge.before, incoming: Vec::new() });
        let mut after = nodes.entry(edge.after).or_insert(Node { name: edge.after, incoming: Vec::new() });
        after.incoming.push(edge.before);
    }
    nodes
}

fn topological_sort(nodes: HashMap<char, Node>) -> String {
    let mut removed_edges: HashMap<char, Vec<char>> = HashMap::new();
    let mut no_incoming: BinaryHeap<&Node> = BinaryHeap::new();
    let mut result = String::new();
    for node in nodes.values() {
        if node.incoming.is_empty() {
            no_incoming.push(&node);
        }
    }
    while let Some(top) = no_incoming.pop() {
        result.push(top.name);
        for node in nodes.values() {
            if node.incoming.contains(&top.name) {
                let mut removed_from_node = removed_edges.entry(node.name).or_insert(Vec::new());
                removed_from_node.push(top.name);
                if node.incoming.iter().all(|i| removed_from_node.contains(i)) {
                    no_incoming.push(&node);
                }
            }
        }
    }
    assert_eq!(result.len(), nodes.len());
    result
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let f = &args.get(1).expect("No input file given");
    let f = File::open(f).expect("File not found");
    let lines: Result<Vec<_>, _> = BufReader::new(f).lines().collect();
    let lines = lines.expect("Could not read lines from file");
    let edges: Result<Vec<Edge>, _> = lines.iter().map(|s| s.parse::<Edge>()).collect();
    let edges = edges.unwrap();
    let dag = create_dag(&edges);
    let sorted = topological_sort(dag);
    println!("{:?}", sorted);
}
