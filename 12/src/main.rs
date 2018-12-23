use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Rule = [bool; 5];

fn parse_init_state(line: &str) -> Vec<bool> {
    line.chars().filter(|&c| c == '#' || c == '.').map(|c| c == '#').collect()
}

fn parse_rule(line: &str) -> Option<Rule> {
    let parts: Vec<_> = line.split(" => ").collect();
    let result = (parts[1].chars().next().unwrap() == '#');
    if result {
        let mut rule = [false; 5];
        for (i, rule_val) in rule[..].iter_mut().zip(parts[0].chars().map(|c| c == '#')) {
            *i = rule_val;
        }
        Some(rule)
    } else {
        None
    }
}

struct RuleLookup {
    rules: Vec<Rule>
}

impl RuleLookup {
    fn new<T: AsRef<str>>(lines: &[T]) -> Self {
        let mut rules: Vec<_> = lines.iter().filter_map(|s| parse_rule(s.as_ref())).collect();
        rules.sort();
        RuleLookup { rules: rules }
    }

    fn rule(&self, state: &[bool; 5]) -> bool {
        self.rules.contains(state)
    }
}

fn print_state(state: &[bool], start_ix: isize) {
    for i in -15..start_ix {
        print!(".");
    }
    for &b in state {
        if b {
            print!("#");
        } else {
            print!(".");
        }
    }
    println!("");
}

fn main() {
    // TODO: do some clever cycle/offset analysis and use that to
    // analytically find a solution.
    // Currently, part 2 solved by looking at 5000, 50000, 500000 etc.
    // and observing a very clear pattern...
    let args: Vec<String> = env::args().collect();
    let f = &args.get(1).expect("No input file given");
    let f = File::open(f).expect("File not found");
    let lines: Result<Vec<_>, _> = BufReader::new(f).lines().collect();
    let lines = lines.expect("Could not read lines from file");
    let init_state = parse_init_state(&lines[0]);
    let rule_lookup = RuleLookup::new(&lines[2..]);
    let mut state = vec![false; 3];
    let mut start_ix = -3;
    state.extend(init_state);
    state.extend(&[false; 3]);
    for i in 0..5000000 {
        //print_state(&state, start_ix);
        let len_before = state.len() as isize;
        let next_vals: Vec<_> = state
            .windows(5)
            .map(|w| rule_lookup.rule(&[w[0], w[1], w[2], w[3], w[4]]))
            .skip_while(|&b| !b)
            .collect();
        state.clear();
        state.extend(&[false; 3]);
        state.extend(next_vals);
        state.extend(&[false; 3]);
        let len_after = state.len() as isize;
        let n_dropped = len_before - len_after + 1;
        start_ix += n_dropped;
    }
    //print_state(&state, start_ix);
    println!("{}", state.len());
    let n_plants: isize = state
        .iter()
        .zip(start_ix..)
        .filter(|&(b, _)| *b)
        .map(|(_, i)| i)
        //.collect();
        .sum();
    println!("Hello, world! {}", n_plants);
}
