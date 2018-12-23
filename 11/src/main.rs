use std::vec::Vec;

fn power_level(x: i32, y: i32, serial: i32) -> i32 {
    let rack_id = x + 10;
    let power = rack_id * y;
    let power = power + serial;
    let power = power * rack_id;
    let power = (power / 100) % 10;
    power - 5
}

fn block_sum(block: &[&[i32]]) -> i32 {
    block.iter().map::<i32, _>(|row| row.iter().sum()).sum()
}

fn part1(grid: &[[i32; 300]; 300]) -> (usize, usize, i32) {
    // make dirty guess for clean code
    let mut best_x = 0;
    let mut best_y = 0;
    let mut best = -99999999;
    for (x, x_window) in grid.windows(3).enumerate() {
        let x0_windows = x_window[0].windows(3);
        let x1_windows = x_window[1].windows(3);
        let x2_windows = x_window[2].windows(3);
        for (y, ((b_0, b_1), b_2)) in x0_windows.zip(x1_windows).zip(x2_windows).enumerate() {
            let res = block_sum(&[&b_0, &b_1, &b_2]);
            if res > best {
                best_x = x;
                best_y = y;
                best = res;
            }
        }
    }
    (best_x, best_y, best)
}

fn part2(grid: &[[i32; 300]; 300], size: usize) -> (usize, usize, i32) {
    // make dirty guess for clean code
    let mut best_x = 0;
    let mut best_y = 0;
    let mut best = -99999999;
    for (x, x_window) in grid.windows(size).enumerate() {
        let mut window_iterators: Vec<_> = x_window.iter().map(|w| w.windows(size)).collect();
        let block: Option<Vec<_>> = window_iterators.iter_mut().map(|w_it| w_it.next()).collect();

    }
    (best_x, best_y, best)
}

fn main() {
    assert_eq!(power_level(3, 5, 8), 4);
    assert_eq!(power_level(122, 79, 57), -5);
    assert_eq!(power_level(217, 196, 39), 0);
    assert_eq!(power_level(101, 153, 71), 4);
    let mut grid: [[i32; 300]; 300] = [[0; 300]; 300];
    for x in 0..299 {
        for y in 0..299 {
            grid[x][y] = power_level(x as i32, y as i32, 4455);
        }
    }
    let (best_x, best_y, best) = part1(&grid);
    println!("{},{}: {}", best_x, best_y, best);
}
