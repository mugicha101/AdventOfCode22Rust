use std::cmp::{max, min};
use std::collections::{HashSet, LinkedList};
use crate::fileio;

pub fn solve_a() {
    let input = fileio::input("src/day18.txt");
    let mut blocks: HashSet<(i32, i32, i32)> = HashSet::new();
    let mut surface_area: i32 = 0;
    for ln in input {
        let parts: Vec<&str> = ln.splitn(3, ",").collect();
        let mut pos: (i32, i32, i32) = (parts[0].parse().unwrap(), parts[1].parse().unwrap(), parts[2].parse().unwrap());
        blocks.insert(pos);
        surface_area += 6;
        let mut check_face = |offset: (i32, i32, i32)| {
            surface_area -= if blocks.contains(&(pos.0 + offset.0, pos.1 + offset.1, pos.2 + offset.2)) {2} else {0};
        };
        check_face((1, 0, 0));
        check_face((-1, 0, 0));
        check_face((0, 1, 0));
        check_face((0, -1, 0));
        check_face((0, 0, 1));
        check_face((0, 0, -1));
    }
    println!("{}", surface_area);
}

pub fn solve_b() {
    let input = fileio::input("src/day18.txt");
    let mut blocks: HashSet<(i32, i32, i32)> = HashSet::new();
    let mut min_vals = (i32::MAX, i32::MAX, i32::MAX);
    let mut max_vals = (i32::MIN, i32::MIN, i32::MIN);
    for ln in input {
        let parts: Vec<&str> = ln.splitn(3, ",").collect();
        let mut pos: (i32, i32, i32) = (parts[0].parse().unwrap(), parts[1].parse().unwrap(), parts[2].parse().unwrap());
        blocks.insert(pos);
        min_vals.0 = min(min_vals.0, pos.0);
        min_vals.1 = min(min_vals.1, pos.1);
        min_vals.2 = min(min_vals.2, pos.2);
        max_vals.0 = max(max_vals.0, pos.0);
        max_vals.1 = max(max_vals.1, pos.1);
        max_vals.2 = max(max_vals.2, pos.2);
    }
    let grid_offset = (min_vals.0 - 1, min_vals.1 - 1, min_vals.2 - 1);
    let grid_bounds = (
        (max_vals.0 - min_vals.0 + 3) as usize,
        (max_vals.1 - min_vals.1 + 3) as usize,
        (max_vals.2 - min_vals.2 + 3) as usize
    );
    let mut grid: Vec<Vec<Vec<(bool, bool)>>> = vec![
        vec![
            vec![
                (false, false); grid_bounds.2
            ]; grid_bounds.1
        ]; grid_bounds.0
    ];
    for pos in blocks {
        grid[(pos.0 - grid_offset.0) as usize]
            [(pos.1 - grid_offset.1) as usize]
            [(pos.2 - grid_offset.2) as usize].0 = true;
    }
    let mut surface_area: u32 = 0;
    let mut q: LinkedList<(usize, usize, usize)> = LinkedList::new();
    q.push_back((0,0,0));
    while q.len() != 0 {
        let pos = q.pop_front().unwrap();
        let mut check_pos = |x: usize, y: usize, z: usize| -> bool {
            let output = grid[x][y][z].0;
            if output {
                surface_area += 1;
            } else if !grid[x][y][z].1 {
                grid[x][y][z].1 = true;
                q.push_back((x, y, z));
            }
            output
        };
        if pos.0 != 0 {
            check_pos(pos.0-1, pos.1, pos.2);
        }
        if pos.0 != grid_bounds.0 - 1 {
            check_pos(pos.0+1, pos.1, pos.2);
        }
        if pos.1 != 0 {
            check_pos(pos.0, pos.1-1, pos.2);
        }
        if pos.1 != grid_bounds.1 - 1 {
            check_pos(pos.0, pos.1+1, pos.2);
        }
        if pos.2 != 0 {
            check_pos(pos.0, pos.1, pos.2-1);
        }
        if pos.2 != grid_bounds.2 - 1 {
            check_pos(pos.0, pos.1, pos.2+1);
        }
    }
    println!("{}", surface_area);
}