use std::borrow::BorrowMut;
use std::cmp::min;
use std::collections::{HashMap, HashSet, VecDeque};
use priority_queue::PriorityQueue;
use crate::fileio;

fn parse_input(input: &Vec<String>) -> (Vec<((i32, i32), u8)>, (i32, i32)) {
    let mut blizzards: Vec<((i32,i32),u8)> = Vec::new();
    let dim: (i32, i32) = (input.len() as i32 - 2, input[0].len() as i32 - 2);
    for r in 1..input.len()-1 {
        let line = input[r].as_bytes();
        for c in 1..line.len()-1 {
            if line[c] == '.' as u8 {
                continue;
            }
            let p = (r as i32 - 1, c as i32 - 1);
            blizzards.push((p, match line[c] as char {
                '>' => {0},
                'v' => {1},
                '<' => {2},
                '^' => {3}
                _ => {panic!("invalid blizzard")}
            }));
        }
    }
    return (blizzards, dim);
}

fn move_blizzards(blizzards: &mut Vec<((i32, i32), u8)>, dim: &(i32, i32)) {
    for b in blizzards {
        b.0 = match b.1 {
            0 => {
                if b.0.1 == dim.1 - 1 { (b.0.0, 0) } else { (b.0.0, b.0.1 + 1) }
            }
            1 => {
                if b.0.0 == dim.0 - 1 { (0, b.0.1) } else { (b.0.0 + 1, b.0.1) }
            }
            2 => {
                if b.0.1 == 0 { (b.0.0, dim.1 - 1) } else { (b.0.0, b.0.1 - 1) }
            }
            3 => {
                if b.0.0 == 0 { (dim.0 - 1, b.0.1) } else { (b.0.0 - 1, b.0.1) }
            }
            _ => {
                panic!("invalid blizzard dir");
            }
        };
    }
}

fn solve(blizzards: &mut Vec<((i32, i32), u8)>, dim: &(i32, i32), inverse: bool) -> u32 {
    // possible positions you are at
    let mut pos_set: HashSet<(i32,i32)> = HashSet::new();
    let mut t = 0;
    let mut end = (dim.0 - 1, dim.1 - 1);
    let mut start = (0, 0);
    if inverse {
        let temp = start;
        start = end;
        end = temp;
    }
    while !pos_set.contains(&end) {
        t += 1;
        // spread your possible positions
        let mut pos_vec: Vec<(i32,i32)> = Vec::new();
        for pos in &pos_set {
            pos_vec.push(*pos);
        }
        for pos in &pos_vec {
            if pos.0 != 0 {
                pos_set.insert((pos.0 - 1, pos.1));
            }
            if pos.0 != dim.0 - 1 {
                pos_set.insert((pos.0 + 1, pos.1));
            }
            if pos.1 != 0 {
                pos_set.insert((pos.0, pos.1 - 1));
            }
            if pos.1 != dim.1 - 1 {
                pos_set.insert((pos.0, pos.1 + 1));
            }
        }
        pos_set.insert(start);

        // move blizzards
        move_blizzards(blizzards, dim);

        // remove positions blizzards are on from your possible positions
        for i in 0..blizzards.len() {
            pos_set.remove(&blizzards[i].0);
        }
    }
    return t+1;
}

pub fn solve_a() {
    let input = fileio::input("src/day24.txt");
    let (mut blizzards, dim) = parse_input(&input);
    println!("{}", solve(&mut blizzards, &dim, false));
}

pub fn solve_b() {
    let input = fileio::input("src/day24.txt");
    let (mut blizzards, dim) = parse_input(&input);
    let mut sum = solve(&mut blizzards, &dim, false);
    move_blizzards(&mut blizzards, &dim);
    sum += solve(&mut blizzards, &dim, true);
    move_blizzards(&mut blizzards, &dim);
    sum += solve(&mut blizzards, &dim, false);
    println!("{}", sum);
}