#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::env;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::vec::Vec;
use std::str::FromStr;

use regex::Regex;
struct Point {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32
}

impl FromStr for Point {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref point_regex: Regex =
                Regex::new("position=<(?P<x>.+), (?P<y>.+)> velocity=<(?P<dx>.+), (?P<dy>.+)>")
                    .unwrap();
        }
        let captures = point_regex.captures(s).ok_or("Could not parse line")?;
        let x = captures["x"].trim().parse::<i32>().unwrap();
        let y = captures["y"].trim().parse::<i32>().unwrap();
        let dx = captures["dx"].trim().parse::<i32>().unwrap();
        let dy = captures["dy"].trim().parse::<i32>().unwrap();
        Ok(Point { x: x, y: y, dx: dx, dy: dy })
    }
}

fn extent(points: &[Point], steps: i32) -> (i32, i32) {
    let min_x = points.iter().map(|p| p.x + p.dx * steps).min().unwrap();
    let max_x = points.iter().map(|p| p.x + p.dx * steps).max().unwrap();
    let min_y = points.iter().map(|p| p.y + p.dy * steps).min().unwrap();
    let max_y = points.iter().map(|p| p.y + p.dy * steps).max().unwrap();
    (max_x - min_x, max_y - min_y)
}

fn extent_score(points: &[Point], steps: i32) -> i64 {
    let ex = extent(points, steps);
    (ex.0 as i64) * (ex.1 as i64)
}

fn part1(points: &[Point]) {
    let best_i = (0..20000).min_by_key(|&i| extent_score(&points, i)).unwrap();
    let answer_points: Vec<(i32, i32)> = points.
        iter()
        .map(|p| (p.x + best_i * p.dx, p.y + best_i * p.dy)).collect();
    let min_x = answer_points.iter().map(|&(x, _)| x).min().unwrap();
    let max_x = answer_points.iter().map(|&(x, _)| x).max().unwrap();
    let min_y = answer_points.iter().map(|&(_, y)| y).min().unwrap();
    let max_y = answer_points.iter().map(|&(_, y)| y).max().unwrap();

    for y in min_y ..= max_y {
        for x in min_x ..= max_x {
            let pt = (x, y);
            if answer_points.contains(&pt) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }

    println!("Wait for {} seconds.", best_i);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let f = &args.get(1).expect("No input file given");
    let f = File::open(f).expect("File not found");
    let lines: Result<Vec<_>, _> = BufReader::new(f).lines().collect();
    let lines = lines.expect("Could not read lines from file");
    let points: Result<Vec<Point>, _> = lines.iter().map(|s| s.parse::<Point>()).collect();
    let points = points.unwrap();
    part1(&points);
}
