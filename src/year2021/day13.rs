use std::cmp::max;
use std::collections::HashSet;
use crate::fileio;

fn parse_input(input: &Vec<String>) -> (HashSet<(i32, i32)>, Vec<(bool, i32)>) {
    let mut pts: HashSet<(i32,i32)> = HashSet::new();
    let mut folds: Vec<(bool,i32)> = Vec::new();
    let mut fold_section = false;
    for l in 0..input.len() {
        if input[l] == "" {
            fold_section = true;
            continue;
        }
        if fold_section {
            let line = input[l].as_bytes();
            folds.push((line[11] == 'y' as u8, input[l][13..line.len()].parse::<i32>().unwrap()));
        } else {
            let parts: Vec<&str> = input[l].split(",").collect();
            pts.insert((parts[0].parse::<i32>().unwrap(), parts[1].parse::<i32>().unwrap()));
        }
    }
    return (pts, folds);
}

fn fold(pts: &mut HashSet<(i32,i32)>, fold: &(bool,i32)) {
    let mut new_pts: Vec<(i32, i32)> = Vec::new();
    for p in pts.iter() {
        if fold.0 {
            new_pts.push((p.0, fold.1 - (fold.1 - p.1).abs()));
        } else {
            new_pts.push((fold.1 - (fold.1 - p.0).abs(), p.1));
        }
    }
    pts.clear();
    for p in new_pts {
        pts.insert(p);
    }
}

pub fn solve_a() {
    let input = fileio::input("src/year2021/input/day13.txt");
    let (mut pts, folds) = parse_input(&input);
    fold(&mut pts, &folds[0]);
    println!("{}", pts.len());
}

pub fn solve_b() {
    let input = fileio::input("src/year2021/input/day13.txt");
    let (mut pts, folds) = parse_input(&input);
    for f in &folds {
        fold(&mut pts, f);
    }
    let mut bound = (0, 0);
    for p in &pts {
        bound.0 = max(bound.0, p.0);
        bound.1 = max(bound.1, p.1);
    }
    let mut grid = vec![vec![false; bound.0 as usize + 1]; bound.1 as usize + 1];
    for p in &pts {
        grid[p.1 as usize][p.0 as usize] = true;
    }
    for r in 0..=bound.1 as usize {
        for c in 0..=bound.0 as usize {
            print!("{} ", if grid[r][c] {'#'} else {'.'});
        }
        println!();
    }
}