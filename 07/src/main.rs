#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, VecDeque};
use std::env;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::vec::Vec;

use regex::Regex;

#[derive(Eq)]
struct Node {
    name: char,
    incoming: Vec<char>,
}

impl Node {
    fn time_cost(&self) -> usize {
        61usize + (self.name as usize) - ('A' as usize)
    }
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
    after: char,
}

impl FromStr for Edge {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref edge_regex: Regex = Regex::new(
                "Step (?P<before>.) must be finished before step (?P<after>.) can begin."
            )
            .unwrap();
        }
        let captures = edge_regex.captures(s).ok_or("Could not parse line")?;
        let before = captures["before"].chars().next().unwrap();
        let after = captures["after"].chars().next().unwrap();
        Ok(Edge {
            before: before,
            after: after,
        })
    }
}

fn create_dag(edges: &[Edge]) -> HashMap<char, Node> {
    let mut nodes: HashMap<char, Node> = HashMap::new();
    for edge in edges {
        nodes.entry(edge.before).or_insert(Node {
            name: edge.before,
            incoming: Vec::new(),
        });
        let mut after = nodes.entry(edge.after).or_insert(Node {
            name: edge.after,
            incoming: Vec::new(),
        });
        after.incoming.push(edge.before);
    }
    nodes
}

struct TopoSort<'n> {
    nodes: &'n HashMap<char, Node>,
    removed_edges: HashMap<char, Vec<char>>,
    no_incoming: BinaryHeap<&'n Node>,
    in_progress: VecDeque<InProgress>,
    now: usize,
}

struct InProgress {
    node: char,
    done_at: usize,
}

impl<'n> TopoSort<'n> {
    fn topological_sort(nodes: &'n HashMap<char, Node>) -> String {
        Self::new(nodes).topological_sort_helper()
    }

    fn topological_worker_sort(nodes: &'n HashMap<char, Node>) -> usize {
        Self::new(nodes).topological_worker_sort_helper()
    }

    fn new(nodes: &'n HashMap<char, Node>) -> Self {
        TopoSort {
            nodes: nodes,
            removed_edges: HashMap::new(),
            no_incoming: BinaryHeap::new(),
            in_progress: VecDeque::new(),
            now: 0usize,
        }
    }

    fn remove_node<'a>(&mut self, to_remove: char) {
        for node in self.nodes.values() {
            if node.incoming.contains(&to_remove) {
                let mut removed_from_node =
                    self.removed_edges.entry(node.name).or_insert(Vec::new());
                removed_from_node.push(to_remove);
                if node.incoming.iter().all(|i| removed_from_node.contains(i)) {
                    self.no_incoming.push(&node);
                }
            }
        }
    }

    fn perform_queued_step(&mut self) {
        let performed = self.in_progress.pop_front().unwrap();
        self.now = performed.done_at;
        self.remove_node(performed.node);
    }

    fn topological_sort_helper(&mut self) -> String {
        let mut result = String::new();
        for node in self.nodes.values() {
            if node.incoming.is_empty() {
                self.no_incoming.push(&node);
            }
        }
        while let Some(top) = self.no_incoming.pop() {
            result.push(top.name);
            self.remove_node(top.name);
        }
        assert_eq!(result.len(), self.nodes.len());
        result
    }

    fn topological_worker_sort_helper(&mut self) -> usize {
        for node in self.nodes.values() {
            if node.incoming.is_empty() {
                self.no_incoming.push(&node);
            }
        }
        loop {
            if self.in_progress.is_empty() && self.no_incoming.is_empty() {
                break;
            }
            match self.no_incoming.pop() {
                Some(top) => self.in_progress.push_back(InProgress {
                    node: top.name,
                    done_at: self.now + top.time_cost(),
                }),
                None => {
                    self.perform_queued_step();
                }
            }
            if self.in_progress.len() == 5 {
                self.perform_queued_step();
            }
        }
        self.now
    }
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
    let result_1 = TopoSort::topological_sort(&dag);
    println!("Answer 1: {:?}", result_1);
    let result_2 = TopoSort::topological_worker_sort(&dag);
    println!("Answer 2: {}", result_2);
}
