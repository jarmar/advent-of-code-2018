use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn transform_char(input: char) -> Option<i8> {
    if input.is_ascii_uppercase() {
        Some((input as i8) - ('A' as i8) + 1)
    } else if input.is_ascii_lowercase() {
        Some(-((input as i8) - ('a' as i8) + 1))
    } else {
        None
    }
}

fn transform_back(input: &i8) -> Option<char> {
    if *input == 0 {
        None
    } else if *input < 0 {
        Some(((-*input + ('a' as i8) - 1) as u8) as char)
    } else {
        Some(((*input + ('A' as i8) - 1) as u8) as char)
    }
}

fn part1_splitnext<'a>(frontier: &mut &'a mut [i8], stack: &mut Vec<&'a mut [i8]>) {
    let tmp_frontier: &'a mut [i8] = ::std::mem::replace(&mut *frontier, &mut []);
    let first_removal_match = tmp_frontier.windows(2).position(|s| s[0] == -s[1]);
    *frontier = match first_removal_match {
        Some(first_removal_ix) => {
            let (mut new_stack_elem, mut new_frontier) = tmp_frontier.split_at_mut(first_removal_ix);
            if !new_stack_elem.is_empty() {
                stack.push(new_stack_elem);
            }
            new_frontier[0] = 0;
            new_frontier[1] = 0;
            &mut new_frontier[2..]
        }
        None => {
            stack.push(tmp_frontier);
            &mut []
        }
    };
}

fn part1_tryconsume<'a>(frontier: &mut &'a mut [i8], stack: &mut Vec<&'a mut [i8]>) -> bool {
    let last_stack_elem = stack.last_mut().unwrap();
    let stack_back: &'a mut [i8] = ::std::mem::replace(&mut *last_stack_elem, &mut []);
    let tmp_frontier: &'a mut [i8] = ::std::mem::replace(&mut *frontier, &mut []);
    let stack_back_len = stack_back.len();
    if stack_back[stack_back_len - 1] == -tmp_frontier[0] {
        stack_back[stack_back_len - 1] = 0;
        tmp_frontier[0] = 0;
        if stack_back_len > 1 {
            *last_stack_elem = &mut stack_back[..stack_back_len - 1];
        }
        *frontier = &mut tmp_frontier[1..];
        true
    } else {
        *last_stack_elem = &mut stack_back[..];
        *frontier = &mut tmp_frontier[..];
        false
    }
}

fn part1_step<'a>(frontier: &mut &'a mut [i8], stack: &mut Vec<&'a mut [i8]>) {
    let stack_len = stack.len();
    if stack_len > 0 {
        if part1_tryconsume(frontier, stack) {
            if stack.last().unwrap().is_empty() {
                stack.pop();
            }
        } else {
            part1_splitnext(frontier, stack);
        }
    } else {
        part1_splitnext(frontier, stack);
    }
}

fn do_part1(mut nifty: &mut [i8]) {
    let mut stack: Vec<&mut [i8]> = Vec::new();
    while !nifty.is_empty() {
        part1_step(&mut nifty, &mut stack);
    }
}

fn part1(input: &str) -> usize {
    let mut nifty: Vec<i8> = input.chars().filter_map(transform_char).collect();
    do_part1(&mut nifty);
    let answer: String = nifty.iter().filter_map(transform_back).collect();
    answer.len()
}

fn part2_char(c: char, input: &str) -> usize {
    let c_lower = c as char;
    let c_upper = c_lower.to_ascii_uppercase();
    let new_str: String = input
        .chars()
        .filter(|x| *x != c_lower && *x != c_upper)
        .collect();
    part1(&new_str)
}

fn part2(input: &str) -> usize {
    (b'a'..b'z')
        .map(|c| part2_char(c as char, input))
        .min()
        .unwrap()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let f = &args.get(1).expect("No input file given");
    let f = File::open(f).expect("File not found");
    let lines: Result<Vec<_>, _> = BufReader::new(f).lines().collect();
    let lines = lines.expect("Could not read lines from file");
    let result_1 = part1(&lines[0]);
    println!("Answer 1: {}", result_1);
    let result_2 = part2(&lines[0]);
    println!("Answer 2: {}", result_2);
}
