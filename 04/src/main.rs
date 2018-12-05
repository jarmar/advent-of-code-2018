use std::collections::HashMap;
use std::env;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::vec::Vec;

enum Event {
    GuardBegins(u32),
    FallsAsleep(u8),
    WakesUp(u8),
}

impl FromStr for Event {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mins = s[15..17].parse::<u8>().expect("Could not parse minutes");
        match &s[19..24] {
            "Guard" => {
                let guard_no = s[26..]
                    .chars()
                    .take_while(|s| s.is_digit(10))
                    .collect::<String>()
                    .parse::<u32>()
                    .expect("Could not parse guard no");
                Ok(Event::GuardBegins(guard_no))
            }
            "falls" => Ok(Event::FallsAsleep(mins)),
            "wakes" => Ok(Event::WakesUp(mins)),
            _ => Err("Could not parse line"),
        }
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Event::*;
        match self {
            GuardBegins(guard_no) => write!(f, "Guard {} begins", guard_no),
            FallsAsleep(mins) => write!(f, "Falls asleep at {}", mins),
            WakesUp(mins) => write!(f, "Wakes up at {}", mins),
        }
    }
}

fn get_sleep_patterns(events: &Vec<Event>) -> HashMap<u32, [u32; 60]> {
    use Event::*;
    let mut sleep_patterns: HashMap<u32, [u32; 60]> = HashMap::new();
    let mut current_guard = 0;
    let mut sleep_from = 0;
    for event in events {
        match event {
            GuardBegins(guard_no) => {
                current_guard = *guard_no;
                if !sleep_patterns.contains_key(&current_guard) {
                    sleep_patterns.insert(current_guard, [0; 60]);
                }
            }
            FallsAsleep(mins) => {
                sleep_from = *mins;
            }
            WakesUp(mins) => {
                let mut guard_sleep_pattern = sleep_patterns
                    .get_mut(&current_guard)
                    .expect("General error");
                for i in sleep_from..*mins {
                    guard_sleep_pattern[i as usize] += 1;
                }
            }
        }
    }
    sleep_patterns
}

fn part1(events: &Vec<Event>) -> u32 {
    let sleep_patterns = get_sleep_patterns(events);
    let sleepiest_guard = sleep_patterns
        .iter()
        .max_by_key::<u32, _>(|(_, &pattern)| pattern.iter().sum());
    let (sleepiest_guard, sleep_pattern) = sleepiest_guard.expect("No guards");
    let (sleepiest_minute, _) = sleep_pattern
        .iter()
        .enumerate()
        .max_by_key(|&(_, sleep_count)| sleep_count)
        .expect("Could not find sleepiest minute");
    (sleepiest_minute as u32) * sleepiest_guard
}

fn part2(events: &Vec<Event>) -> u32 {
    let sleep_patterns = get_sleep_patterns(events);
    let sleepiest_minute = sleep_patterns.iter().map(|(g, pattern)| {
        (
            g,
            pattern
                .iter()
                .enumerate()
                .max_by_key(|&(_, sleep_count)| sleep_count)
                .expect("yes"),
        )
    });
    let (guard_no, (the_min, _)) = sleepiest_minute
        .max_by_key(|&(_, (_, sleep_count))| sleep_count)
        .expect("no");
    guard_no * (the_min as u32)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let f = &args.get(1).expect("No input file given");
    let f = File::open(f).expect("File not found");
    let lines: Result<Vec<_>, _> = BufReader::new(f).lines().collect();
    let mut lines = lines.expect("Could not read lines from file");
    lines.sort();
    let events: Result<Vec<Event>, _> = lines.iter().map(|l| l.parse::<Event>()).collect();
    let events = events.expect("Could not parse claims");
    let result_1 = part1(&events);
    println!("{}", result_1);
    let result_2 = part2(&events);
    println!("{}", result_2);
}
