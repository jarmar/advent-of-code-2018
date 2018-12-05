#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::str::FromStr;
use std::vec::Vec;

use regex::Regex;

struct Claim {
    num: i32,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

impl FromStr for Claim {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref claim_regex: Regex =
                Regex::new(r"#(?P<num>\d+) @ (?P<x>\d+),(?P<y>\d+): (?P<w>\d+)x(?P<h>\d+)")
                    .unwrap();
        }
        let captures = claim_regex.captures(s).expect("Could not parse line");
        let num_fromstr = captures["num"].parse::<i32>()?;
        let x_fromstr = captures["x"].parse::<i32>()?;
        let y_fromstr = captures["y"].parse::<i32>()?;
        let w_fromstr = captures["w"].parse::<i32>()?;
        let h_fromstr = captures["h"].parse::<i32>()?;
        Ok(Claim {
            num: num_fromstr,
            x: x_fromstr,
            y: y_fromstr,
            w: w_fromstr,
            h: h_fromstr,
        })
    }
}

fn get_cloth(claims: &Vec<Claim>) -> [[u16; 1000]; 1000] {
    let mut cloth = [[0u16; 1000]; 1000];
    for claim in claims {
        for i in 0..claim.w {
            let x_coord = (claim.x + i) as usize;
            for j in 0..claim.h {
                let y_coord = (claim.y + j) as usize;
                cloth[x_coord][y_coord] += 1;
            }
        }
    }
    cloth
}

fn part1(cloth: &[[u16; 1000]; 1000]) -> i32 {
    let mut overlaps = 0;
    for row in cloth.iter() {
        for cell in row.iter() {
            if *cell > 1 {
                overlaps += 1;
            }
        }
    }
    overlaps
}

fn part2(claims: &Vec<Claim>, cloth: &[[u16; 1000]; 1000]) -> Option<i32> {
    'claim_loop: for claim in claims {
        for i in 0..claim.w {
            let x_coord = (claim.x + i) as usize;
            for j in 0..claim.h {
                let y_coord = (claim.y + j) as usize;
                if cloth[x_coord][y_coord] != 1 {
                    continue 'claim_loop;
                }
            }
        }
        return Some(claim.num);
    }
    None
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let f = &args.get(1).expect("No input file given");
    let f = File::open(f).expect("File not found");
    let lines: Result<Vec<_>, _> = BufReader::new(f).lines().collect();
    let lines = lines.expect("Could not read lines from file");
    let claims: Result<Vec<Claim>, _> = lines.iter().map(|s| s.parse::<Claim>()).collect();
    let claims = claims.expect("Could not parse claims");
    let cloth = get_cloth(&claims);
    let result_1 = part1(&cloth);
    println!("Answer: {}", result_1);
    let result_2 = part2(&claims, &cloth).expect("No answer for part 2?!");
    println!("Answer: {}", result_2);
}
