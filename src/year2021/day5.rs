use std::collections::HashMap;
use crate::fileio;

fn solve(exclude_diagonals: bool) {
    let input = fileio::input("src/year2021/input/day5.txt");
    let mut count = 0;
    let mut overlaps: HashMap<(i32, i32),bool> = HashMap::new();
    for ln in input {
        let parts: Vec<&str> = ln.split(" -> ").collect();
        let mut start: (i32,i32) = {
            let segs: Vec<&str> = parts[0].split(",").collect();
            (segs[0].parse::<i32>().unwrap(), segs[1].parse::<i32>().unwrap())
        };
        let mut end: (i32,i32) = {
            let segs: Vec<&str> = parts[1].split(",").collect();
            (segs[0].parse::<i32>().unwrap(), segs[1].parse::<i32>().unwrap())
        };
        let mut offset = ((end.0 - start.0).signum(), (end.1 - start.1).signum());
        if offset.0 == 0 && offset.1 == 0 {
            offset.0 = 1;
        } else if exclude_diagonals && (offset.0 != 0) ^ (offset.1 == 0) {
            continue;
        }
        end = (end.0 + offset.0, end.1 + offset.1);
        while start.0 != end.0 || start.1 != end.1 {
            if !overlaps.contains_key(&start) {
                overlaps.insert(start, false);
            } else {
                count += if !overlaps.insert(start, true).unwrap() {1} else {0};
            }
            start.0 += offset.0;
            start.1 += offset.1;
        }
    }
    println!("{}", count);
}

pub fn solve_a() {
    solve(true);
}

pub fn solve_b() {
    solve(false);
}