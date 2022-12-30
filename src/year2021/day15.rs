use std::collections::VecDeque;
use priority_queue::PriorityQueue;
use crate::fileio;

fn parse_input(input: &Vec<String>) -> Vec<Vec<u32>> {
    let mut grid: Vec<Vec<u32>> = vec![vec![0;input[0].len()];input.len()];
    for r in 0..input.len() {
        let line = input[r].as_bytes();
        for c in 0..line.len() {
            grid[r][c] = (line[c] - '0' as u8) as u32;
        }
    }
    return grid;
}

fn traverse(risk_grid: &Vec<Vec<u32>>) {
    let mut total_risk: Vec<Vec<u32>> = vec![vec![u32::MAX;risk_grid[0].len()];risk_grid.len()];
    let mut pq: PriorityQueue<(usize, usize, u32), i32> = PriorityQueue::new(); // priority = r + c - risk
    pq.push((0,0,0),0);
    while !pq.is_empty() {
        let ((r,c,risk),_) = pq.pop().unwrap();
        if total_risk[r][c] <= risk {
            continue;
        }
        total_risk[r][c] = risk;
        if r+1 == risk_grid.len() && c+1 == risk_grid[0].len() {
            break;
        }
        let mut to_pos = |r: usize, c: usize| {
            let new_risk = risk+risk_grid[r][c];
            pq.push((r,c,new_risk), r as i32 + c as i32 - new_risk as i32);
        };
        if r != 0 {
            to_pos(r-1, c);
        }
        if c != 0 {
            to_pos(r, c-1);
        }
        if r+1 != risk_grid.len() {
            to_pos(r+1, c);
        }
        if c+1 != risk_grid[0].len() {
            to_pos(r, c+1);
        }
    }
    println!("{}", total_risk[risk_grid.len()-1][risk_grid[0].len()-1]);
}

pub fn solve_a() {
    let input = fileio::input("src/year2021/input/day15.txt");
    let risk_grid = parse_input(&input);
    traverse(&risk_grid);
}

pub fn solve_b() {
    let input = fileio::input("src/year2021/input/day15.txt");
    let mut risk_grid = parse_input(&input);
    let rows = risk_grid.len();
    let cols = risk_grid[0].len();
    for _ in 0..4*rows {
        risk_grid.push(Vec::new());
    }
    // add rows
    for r in 0..rows*4 {
        for c in 0..cols {
            let v = risk_grid[r][c] + 1;
            risk_grid[r+rows].push(if v == 10 {1} else {v});
        }
    }
    // add cols
    for r in 0..rows*5 {
        for c in 0..cols*4 {
            let v = risk_grid[r][c] + 1;
            risk_grid[r].push(if v == 10 {1} else {v});
        }
    }
    traverse(&risk_grid);
}