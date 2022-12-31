use std::cmp::min;
use crate::fileio;

fn parse_input(input: &Vec<String>) -> ((i64, i64), (i64, i64)) {
    let parts: Vec<&str> = input[0].split("=").collect();
    let x_sec: Vec<&str> = parts[1][0..parts[1].len()-3].split("..").collect();
    let y_sec: Vec<&str> = parts[2].split("..").collect();
    let x_bounds = (x_sec[0].parse::<i64>().unwrap(), x_sec[1].parse::<i64>().unwrap());
    let y_bounds = (y_sec[0].parse::<i64>().unwrap(), y_sec[1].parse::<i64>().unwrap());
    return (x_bounds, y_bounds);
}

fn find_max_y_vel0(x_bounds: (i64,i64), y_bounds: (i64,i64)) -> i64 {
    let x_vel0 = (-0.5 + (((x_bounds.0 << 3) + 1)  as f64).sqrt() * 0.5).ceil() as i64;
    // let max_x = (x_vel0 * (x_vel0 + 1)) >> 1;
    let mut y_vel0 = x_vel0;
    let mut y_vel = y_vel0;
    let mut y = 0;
    let mut t = 0;
    let mut max_y_vel0 = 0;
    const MAX_LOOPS: u32 = 100000;
    for _ in 0..MAX_LOOPS {
        y_vel0 += 1;
        y_vel += 1;
        y += t;
        while y > y_bounds.1 {
            y += y_vel;
            y_vel -= 1;
            t += 1;
        }
        if y >= y_bounds.0 {
            max_y_vel0 = y_vel0;
        }
    }
    return max_y_vel0;
}

pub fn solve_a() {
    let input = fileio::input("src/year2021/input/day17.txt");
    let (x_bounds, y_bounds) = parse_input(&input);
    let max_y_vel0 = find_max_y_vel0(x_bounds, y_bounds);
    let max_y = (max_y_vel0 * (max_y_vel0 + 1)) >> 1;
    println!("{}", max_y);
}

pub fn solve_b() {
    let input = fileio::input("src/year2021/input/day17.txt");
    let (x_bounds, y_bounds) = parse_input(&input);
    let min_x_vel = (-0.5 + (((x_bounds.0 << 3) + 1)  as f64).sqrt() * 0.5).ceil() as i64;
    let vel_bounds = (
        min_x_vel,
        y_bounds.0,
        x_bounds.1,
        find_max_y_vel0(x_bounds, y_bounds),
    );
    let mut combos = 0;
    for x_vel0 in vel_bounds.0..=vel_bounds.2 {
        for y_vel0 in vel_bounds.1..=vel_bounds.3 {
            let mut x_vel = x_vel0;
            let mut y_vel = y_vel0;
            let mut x = 0;
            let mut y = 0;
            while x <= x_bounds.1 && y >= y_bounds.0 {
                x += x_vel;
                y += y_vel;
                if x >= x_bounds.0 && x <= x_bounds.1 && y >= y_bounds.0 && y <= y_bounds.1 {
                    combos += 1;
                    break;
                }
                x_vel = if x_vel == 0 {0} else {x_vel - x_vel.signum()};
                y_vel -= 1;
            }
        }
    }
    println!("{}", combos);
}