use std::collections::{HashMap, HashSet};
use std::cmp::{max,min};
use crate::fileio;

pub fn solve_a() {
    let input = fileio::input("src/day23.txt");
    let mut elves: HashSet<(i32,i32)> = HashSet::new();
    for r in 0..input.len() {
        let line = input[r].as_bytes();
        for c in 0..line.len() {
            if line[c] == '#' as u8 {
                elves.insert((r as i32, c as i32));
            }
        }
    }
    const ROUNDS: usize = 10;
    for round in 0..ROUNDS {
        let mut dest_vec: Vec<((i32, i32),(i32,i32))> = Vec::new(); // original pos, dest pos
        let mut dest_count: HashMap<(i32, i32), u32> = HashMap::new(); // frequency of dest tiles
        let mut occupied: [bool; 8] = [false; 8];
        for elf in &elves {
            let mut contains_any = false;
            for i in 0..8 {
                let offset = [(-1,-1),(-1,0),(-1,1),(0,-1),(0,1),(1,-1),(1,0),(1,1)][i];
                occupied[i] = elves.contains(&(elf.0 + offset.0, elf.1 + offset.1));
                contains_any |= occupied[i];
            }
            let mut dest = *elf;
            if contains_any {
                let mut dir = round & 3;
                let mut chosen = false;
                for _ in 0..4 {
                    match dir {
                        0 => {
                            if !(occupied[0] || occupied[1] || occupied[2]) { // move north
                                dest.0 -= 1;
                                chosen = true;
                            }
                        }
                        1 => {
                            if !(occupied[5] || occupied[6] || occupied[7]) { // move south
                                dest.0 += 1;
                                chosen = true;
                            }
                        }
                        2 => {
                            if !(occupied[0] || occupied[3] || occupied[5]) { // move west
                                dest.1 -= 1;
                                chosen = true;
                            }
                        }
                        3 => {
                            if !(occupied[2] || occupied[4] || occupied[7]) { // move east
                                dest.1 += 1;
                                chosen = true;
                            }
                        }
                        _ => {
                            panic!("invalid direction");
                        }
                    }
                    if chosen {
                        break;
                    }
                    dir = (dir + 1) & 3;
                }
            }
            dest_vec.push((*elf, dest));
            let d = &dest_vec[dest_vec.len() - 1].1;
            if dest_count.contains_key(d) {
                *dest_count.get_mut(d).unwrap() += 1;
            } else {
                dest_count.insert(*d, 1);
            }
        }
        elves.clear();
        for dest in &dest_vec {
            if dest_count[&dest.1] > 1 {
                elves.insert(dest.0);
            } else {
                elves.insert(dest.1);
            }
        }
    }
    let mut bounds = (i32::MAX, i32::MAX, i32::MIN, i32::MIN);
    for elf in &elves {
        bounds.0 = min(bounds.0, elf.0);
        bounds.1 = min(bounds.1, elf.1);
        bounds.2 = max(bounds.2, elf.0);
        bounds.3 = max(bounds.3, elf.1);
    }
    let empty_tiles = (bounds.2 - bounds.0 + 1) * (bounds.3 - bounds.1 + 1) - elves.len() as i32;
    println!("{}", empty_tiles);
}

pub fn solve_b() {
    let input = fileio::input("src/day23.txt");
    let mut elves: HashSet<(i32,i32)> = HashSet::new();
    for r in 0..input.len() {
        let line = input[r].as_bytes();
        for c in 0..line.len() {
            if line[c] == '#' as u8 {
                elves.insert((r as i32, c as i32));
            }
        }
    }
    let mut change: bool = true;
    let mut round: u32 = 1;
    while change {
        round += 1;
        let mut dest_vec: Vec<((i32, i32),(i32,i32))> = Vec::new(); // original pos, dest pos
        let mut dest_count: HashMap<(i32, i32), u32> = HashMap::new(); // frequency of dest tiles
        let mut occupied: [bool; 8] = [false; 8];
        for elf in &elves {
            let mut contains_any = false;
            for i in 0..8 {
                let offset = [(-1,-1),(-1,0),(-1,1),(0,-1),(0,1),(1,-1),(1,0),(1,1)][i];
                occupied[i] = elves.contains(&(elf.0 + offset.0, elf.1 + offset.1));
                contains_any |= occupied[i];
            }
            let mut dest = *elf;
            if contains_any {
                let mut dir = round & 3;
                let mut chosen = false;
                for _ in 0..4 {
                    match dir {
                        0 => {
                            if !(occupied[0] || occupied[1] || occupied[2]) { // move north
                                dest.0 -= 1;
                                chosen = true;
                            }
                        }
                        1 => {
                            if !(occupied[5] || occupied[6] || occupied[7]) { // move south
                                dest.0 += 1;
                                chosen = true;
                            }
                        }
                        2 => {
                            if !(occupied[0] || occupied[3] || occupied[5]) { // move west
                                dest.1 -= 1;
                                chosen = true;
                            }
                        }
                        3 => {
                            if !(occupied[2] || occupied[4] || occupied[7]) { // move east
                                dest.1 += 1;
                                chosen = true;
                            }
                        }
                        _ => {
                            panic!("invalid direction");
                        }
                    }
                    if chosen {
                        break;
                    }
                    dir = (dir + 1) & 3;
                }
            }
            dest_vec.push((*elf, dest));
            let d = &dest_vec[dest_vec.len() - 1].1;
            if dest_count.contains_key(d) {
                *dest_count.get_mut(d).unwrap() += 1;
            } else {
                dest_count.insert(*d, 1);
            }
        }
        elves.clear();
        change = false;
        for dest in &dest_vec {
            if dest_count[&dest.1] > 1 {
                elves.insert(dest.0);
            } else {
                elves.insert(dest.1);
                if dest.0.0 != dest.1.0 || dest.0.1 != dest.1.1 {
                    change = true;
                }
            }
        }
    }
    println!("{}", round);
}