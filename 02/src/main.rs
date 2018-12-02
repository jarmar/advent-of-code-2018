use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn count_ascii_chars(boxid: &String) -> Vec<i64> {
    let mut counts = vec![0; 26];
    for letter in boxid.chars() {
        assert!(letter.is_ascii_lowercase());
        let ix = letter as usize - ('a' as usize);
        counts[ix] += 1;
    }
    counts
}

fn part1(lines: &Vec<String>) -> i64 {
    let mut n_has_two = 0;
    let mut n_has_three = 0;
    for line in lines {
        let counts = count_ascii_chars(&line);
        if counts.iter().any(|count| *count == 2) {
            n_has_two += 1;
        }
        if counts.iter().any(|count| *count == 3) {
            n_has_three += 1;
        }
    }
    n_has_two * n_has_three
}

fn chars_diff(first: &String, second: &String) -> usize {
    first
        .chars()
        .zip(second.chars())
        .filter(|(a, b)| a != b)
        .count()
}

fn common_chars(first: &String, second: &String) -> String {
    first
        .chars()
        .zip(second.chars())
        .filter(|(a, b)| a == b)
        .map(|(a, _)| a)
        .collect()
}

fn part2(lines: &Vec<String>) -> Result<String, &'static str> {
    for line_a in lines {
        for line_b in lines {
            if chars_diff(line_a, line_b) == 1 {
                return Ok(common_chars(line_a, line_b));
            }
        }
    }
    Err("Could not find matching IDs")
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let f = &args.get(1).expect("No input file given");
    let f = File::open(f).expect("File not found");
    let lines: Result<Vec<_>, _> = BufReader::new(f).lines().collect();
    let lines = lines.expect("Could not read lines from file");
    let result_1 = part1(&lines);
    println!("Answer 1: {}", result_1);
    let result_2 = part2(&lines).unwrap();
    println!("Answer 2: {}", result_2);
}
