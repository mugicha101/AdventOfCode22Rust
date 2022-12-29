use std::collections::VecDeque;
use crate::fileio;

fn parse_input(input: &Vec<String>) -> Vec<Vec<u8>> {
    let mut grid: Vec<Vec<u8>> = Vec::new();
    for ln in input {
        let r = grid.len();
        grid.push(Vec::new());
        for c in ln.bytes() {
            grid[r].push(c - '0' as u8);
        }
    }
    return grid;
}

fn sim_round(grid: &mut Vec<Vec<u8>>) -> u32 {
    const ROWS: usize = 10;
    const COLS: usize = 10;
    let mut q: VecDeque<(usize,usize)> = VecDeque::new();
    let mut flashed: Vec<(usize,usize)> = Vec::new();
    for r in 0..ROWS {
        for c in 0..COLS {
            grid[r][c] += 1;
            if grid[r][c] == 10 {
                flashed.push((r,c));
                q.push_back((r,c));
            }
        }
    }
    while !q.is_empty() {
        let (r,c) = q.pop_front().unwrap();
        let mut handle_pos = |r: usize, c: usize| {
            grid[r][c] += 1;
            if grid[r][c] == 10 {
                flashed.push((r,c));
                q.push_back((r,c));
            }
        };
        let bools = [r != 0, c != 0, r != ROWS - 1, c != COLS - 1];
        if bools[0] {
            handle_pos(r-1,c);
        }
        if bools[1] {
            handle_pos(r,c-1);
        }
        if bools[2] {
            handle_pos(r+1,c);
        }
        if bools[3] {
            handle_pos(r,c+1);
        }
        if bools[0] && bools[1] {
            handle_pos(r-1,c-1);
        }
        if bools[0] && bools[3] {
            handle_pos(r-1,c+1);
        }
        if bools[2] && bools[1] {
            handle_pos(r+1,c-1);
        }
        if bools[2] && bools[3] {
            handle_pos(r+1,c+1);
        }
    }
    for (r,c) in &flashed {
        grid[*r][*c] = 0;
    }
    return flashed.len() as u32;
}

pub fn solve_a() {
    let input = fileio::input("src/year2021/input/day11.txt");
    let mut grid = parse_input(&input);
    let mut flashes: u32 = 0;
    const ROUNDS: u32 = 100;
    for _ in 0..ROUNDS {
        flashes += sim_round(&mut grid);
    }
    println!("{}", flashes);
}

pub fn solve_b() {
    let input = fileio::input("src/year2021/input/day11.txt");
    let mut grid = parse_input(&input);
    let mut round: u32 = 1;
    while sim_round(&mut grid) != 100 {
        round += 1;
    }
    println!("{}", round);
}