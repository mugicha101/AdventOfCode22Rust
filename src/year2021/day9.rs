use std::collections::VecDeque;
use crate::fileio;

fn parse_input(input: &Vec<String>) -> Vec<Vec<u8>> {
    let mut grid: Vec<Vec<u8>> = vec![vec![0;input[0].len()];input.len()];
    for r in 0..input.len() {
        let line = input[r].as_bytes();
        for c in 0..line.len() {
            grid[r][c] = line[c] - '0' as u8;
        }
    }
    return grid;
}

fn get_low_pts(grid: &Vec<Vec<u8>>) -> Vec<(usize, usize)> {
    let mut low_pts: Vec<(usize,usize)> = Vec::new();
    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            let v = grid[r][c];
            if (r == 0 || grid[r-1][c] > v)
                && (c == 0 || grid[r][c-1] > v)
                && (r == grid.len()-1 || grid[r+1][c] > v)
                && (c == grid[r].len()-1 || grid[r][c+1] > v) {
                low_pts.push((r,c));
            }
        }
    }
    return low_pts;
}

pub fn solve_a() {
    let input = fileio::input("src/year2021/input/day9.txt");
    let grid = parse_input(&input);
    let mut risk = 0;
    let low_pts = get_low_pts(&grid);
    for (r,c) in low_pts {
        risk += grid[r][c] as u32 + 1;
    }
    println!("{}", risk);
}

pub fn solve_b() {
    let input = fileio::input("src/year2021/input/day9.txt");
    let grid = parse_input(&input);
    let low_pts = get_low_pts(&grid);
    let mut visited: Vec<Vec<bool>> = vec![vec![false;grid[0].len()];grid.len()];
    let mut q: VecDeque<(usize,usize)> = VecDeque::new();
    let mut largest: [u32;3] = [0,0,0];
    let mut min_index = 0;
    for i in 1..=low_pts.len() {
        let mut basin_size: u32 = 1;
        q.push_back(low_pts[i-1]);
        visited[low_pts[i-1].0][low_pts[i-1].1] = true;
        while q.len() != 0 {
            let (r,c) = q.pop_front().unwrap();
            let mut visit = |r: usize, c: usize| {
                if grid[r][c] == 9 || visited[r][c] {
                    return;
                }
                visited[r][c] = true;
                basin_size += 1;
                q.push_back((r,c));
            };
            if r != 0 {
                visit(r-1,c);
            }
            if c != 0 {
                visit(r,c-1);
            }
            if r != grid.len()-1 {
                visit(r+1,c);
            }
            if c != grid[r].len()-1 {
                visit(r,c+1);
            }
        }
        if basin_size > largest[min_index] {
            largest[min_index] = basin_size;
            min_index = if largest[0] < largest[1] {if largest[2] < largest[0] {2} else {0}} else {if largest[2] < largest[1] {2} else {1}};
        }
    }
    let mut prod = 1;
    for b in largest {
        prod *= b;
    }
    println!("{}", prod);
}