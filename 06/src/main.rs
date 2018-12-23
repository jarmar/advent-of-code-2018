use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Hash, PartialEq, Eq, Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.splitn(2, ", ").collect();
        assert_eq!(parts.len(), 2);
        let x = parts[0].parse::<i32>()?;
        let y = parts[1].parse::<i32>()?;
        Ok(Point { x: x, y: y })
    }
}

// At first, I thought computing the convex hull would give the set of points
// with infinite area. But that didn't hold in taxicab geometry, it turned out.
// So this is left only for posterity.
fn orientation(l_a: &Point, l_b: &Point, c: &Point) -> Ordering {
    let res =
        (l_b.x - l_a.x) * (c.y - l_a.y) - (c.x - l_a.x) * (l_b.y - l_a.y);
    res.cmp(&0)
}

fn giftwrap_next<'a>(points: &'a [Point], current: &Point) -> &'a Point {
    points
        .iter()
        .filter(|p| p != &current)
        .min_by(|a, b| orientation(current, a, b))
        .unwrap()
}

fn giftwrap(points: &[Point]) -> HashSet<&Point> {
    let start = points.iter().min_by_key(|p| p.x).unwrap();
    let mut convex_hull: HashSet<&Point> = HashSet::new();
    convex_hull.insert(start);
    let mut current = start;
    loop {
        let next = giftwrap_next(points, current);
        if next == start {
            break;
        } else {
            assert_eq!(true, convex_hull.insert(next));
            current = next;
        }
    }
    convex_hull
}

#[derive(Copy, Clone, Debug)]
enum AreaState<'a> {
    Nothing,
    Contested,
    Preliminary(&'a Point),
    Owned(&'a Point),
}

fn silly_hash(point: &Point) -> char {
    ((((point.x * 397 + point.y) % 26) + ('A' as i32) - 1) as u8) as char
}

impl<'a> fmt::Display for AreaState<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use AreaState::*;
        let disp = match self {
            Nothing => '.',
            Contested => '#',
            Owned(p) => silly_hash(p),
            _ => 'ö',
        };
        write!(f, "{}", disp)
    }
}

fn take<'a>(area: &AreaState, point: &'a Point) -> Option<AreaState<'a>> {
    use AreaState::*;
    match area {
        Nothing => Some(Preliminary(point)),
        Preliminary(p) if p == &point => None,
        Preliminary(_) => Some(Contested),
        _ => None,
    }
}

fn manhattan_step_new<'a>(
    area: &mut [[AreaState<'a>; 360]; 360],
    points: &[(&'a Point, Point)],
) -> Vec<(&'a Point, Point)> {
    let mut neighbours: Vec<(&Point, Point)> = Vec::new();
    for (owner, p) in points {
        if p.x < 0 || p.x >= 360 || p.y < 0 || p.y >= 360 {
            continue;
        }

        let state = &mut area[p.x as usize][p.y as usize];
        let before = &mut AreaState::Nothing;
        std::mem::swap(state, before);
        let new_state = take(&before, owner);
        match new_state {
            Some(inner) => {
                *state = inner;
                for adjacent in adjacent_points(p).iter() {
                    neighbours.push((owner, *adjacent));
                }
            }
            None => {
                *state = *before;
            }
        }
    }
    neighbours
}

fn adjacent_points(p: &Point) -> [Point; 4] {
    [Point { x: p.x - 1, y: p.y },
     Point { x: p.x + 1, y: p.y },
     Point { x: p.x, y: p.y - 1 },
     Point { x: p.x, y: p.y + 1 }]
}

fn lock<'a>(area: &AreaState<'a>) -> Option<AreaState<'a>> {
    use AreaState::*;
    match area {
        Preliminary(p) => Some(Owned(p)),
        _ => None,
    }
}

fn lock_step<'a>(area: &mut [[AreaState<'a>; 360]; 360]) {
    for mut row in area.iter_mut() {
        for mut elem in row.iter_mut() {
            let new_state = lock(elem);
            match new_state {
                Some(inner) => *elem = inner,
                None => (),
            }
        }
    }
}

fn manhattan_new<'a>(area: &mut [[AreaState<'a>; 360]; 360], points: &'a [Point]) {
    let mut step_points: Vec<(&Point, Point)> = points.iter().map(|p| (p, *p)).collect();
    loop {
        step_points = manhattan_step_new(area, &step_points);
        if step_points.is_empty() {
            break;
        }
        lock_step(area);
    }
}

fn get_owner<'a>(area: &AreaState<'a>) -> Option<&'a Point> {
    match area {
        AreaState::Owned(p) => Some(p),
        _ => None,
    }
}

fn do_thing<'a>(set: &mut HashSet<&'a Point>, area: &[[AreaState<'a>; 360]; 360], i: usize) {
    let areas = [area[0][i], area[i][0], area[359][i], area[i][359]];
    let vals = areas.iter().filter_map(|a| get_owner(a));
    for val in vals {
        set.insert(val);
    }
}

type CostsCalculator = [i32; 360];

fn get_axis_costs(points: &[Point], f: &Fn(&Point) -> i32) -> CostsCalculator {
    let n_points = points.len() as i32;
    let mut points_sorted: Vec<&Point> = points.iter().collect();
    points_sorted.sort_by_key(|p| f(p));

    let mut costs: [i32; 360] = [0; 360];
    let mut costs_ix: usize = 0;
    let mut points_to_left = 0;
    let mut points_to_right = n_points;
    let mut current_cost = points_sorted.iter().map(|p| f(p)).sum();
    for point in points_sorted {
        while costs_ix < (f(point) as usize) {
            current_cost += points_to_left;
            current_cost -= points_to_right;
            costs[costs_ix] = current_cost;
            costs_ix += 1;
        }
        points_to_left += 1;
        points_to_right -= 1;
    }
    assert_eq!(points_to_right, 0);
    assert_eq!(points_to_left, n_points);
    for last_part_ix in costs_ix..360 {
        current_cost += n_points;
        costs[last_part_ix] = current_cost;
    }
    costs
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let f = &args.get(1).expect("No input file given");
    let f = File::open(f).expect("File not found");
    let lines: Result<Vec<_>, _> = BufReader::new(f).lines().collect();
    let lines = lines.expect("Could not read lines from file");
    let points: Result<Vec<Point>, _> = lines.iter().map(|s| s.parse::<Point>()).collect();
    let points = points.expect("Could not parse points");
    let mut area: [[AreaState; 360]; 360] = [[AreaState::Nothing; 360]; 360];
    manhattan_new(&mut area, &points);
    let mut counts: HashMap<&Point, i64> = HashMap::new();
    //for row in area.iter() {
    //    for elem in row.iter() {
    //        print!("{}", elem);
    //    }
    //    println!("");
    //}
    let owners: Vec<&Point> = area
        .iter()
        .map(|row| row
             .iter()
             .filter_map(|a| get_owner(a))
             .collect::<Vec<_>>())
        .collect::<Vec<_>>()
        .as_slice()
        .concat();
    let mut edgy: HashSet<&Point> = HashSet::new();
    for i in 0..360 {
        do_thing(&mut edgy, &area, i);
    }
    for owner in owners {
        if edgy.contains(&owner) {
            continue;
        }
        counts.entry(owner)
            .and_modify(|c| *c += 1)
            .or_insert(1);
    }
    let result_1 = counts.iter().max_by_key(|&(_, c)| c).unwrap();
    println!("Answer 1: {} ({:?})", result_1.1, result_1.0);
    let x_costs = get_axis_costs(&points, &|p| p.x);
    let y_costs = get_axis_costs(&points, &|p| p.y);
    let mut count = 0;
    for x_cost in x_costs.iter() {
        for y_cost in y_costs.iter() {
            if x_cost + y_cost < 10000 {
                count += 1;
            }
        }
    }
    println!("Answer 2: {}", count);
}
