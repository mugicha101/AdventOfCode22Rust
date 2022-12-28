use std::collections::{HashMap, HashSet};
use std::cmp::{max,min};
use crate::fileio;

fn parse_input(input: &Vec<String>) -> HashSet<(i32, i32)> {
    let mut elves: HashSet<(i32,i32)> = HashSet::new();
    for r in 0..input.len() {
        let line = input[r].as_bytes();
        for c in 0..line.len() {
            if line[c] == '#' as u8 {
                elves.insert((r as i32, c as i32));
            }
        }
    }
    return elves;
}

fn advance_round(elves: &mut HashSet<(i32, i32)>, proposed: &mut HashMap<(i32, i32), (i32, i32)>, round: u32) -> bool {
    proposed.clear();
    let mut still_elves: Vec<(i32, i32)> = Vec::new();
    for elf in &*elves {
        let n = [
            elves.contains(&(elf.0 - 1, elf.1 - 1)),
            elves.contains(&(elf.0 - 1, elf.1)),
            elves.contains(&(elf.0 - 1, elf.1 + 1)),
            elves.contains(&(elf.0, elf.1 - 1)),
            elves.contains(&(elf.0, elf.1 + 1)),
            elves.contains(&(elf.0 + 1, elf.1 - 1)),
            elves.contains(&(elf.0 + 1, elf.1)),
            elves.contains(&(elf.0 + 1, elf.1 + 1))
        ];
        if !(n[0] || n[1] || n[2] || n[3] || n[4] || n[5] || n[6] || n[7]) {
            still_elves.push(*elf);
            continue;
        }
        let mut t = round & 3;
        let mut matched = false;
        let mut dest = *elf;
        for _ in 0..4 {
            match t {
                0 => { // ^ move north
                    if !n[0] && !n[1] && !n[2] {
                        dest = (elf.0 - 1, elf.1);
                        matched = true;
                    }
                }
                1 => { // v move south
                    if !n[5] && !n[6] && !n[7] {
                        dest = (elf.0 + 1, elf.1);
                        matched = true;
                    }
                }
                2 => { // < move west
                    if !n[0] && !n[3] && !n[5] {
                        dest = (elf.0, elf.1 - 1);
                        matched = true;
                    }
                }
                3 => { // > move east
                    if !n[2] && !n[4] && !n[7] {
                        dest = (elf.0, elf.1 + 1);
                        matched = true;
                    }
                }
                _ => {
                    panic!("invalid direction");
                }
            }
            if matched {
                break;
            }
            t = (t + 1) & 3;
        }
        if matched {
            if proposed.contains_key(&dest) {
                still_elves.push(proposed[&dest]);
                still_elves.push(*elf);
                proposed.remove(&dest);
            } else {
                proposed.insert(dest, *elf);
            }
        } else {
            still_elves.push(*elf);
        }
    }
    let elf_count = elves.len();
    elves.clear();
    for (dest, elf) in &*proposed {
        elves.insert(*dest);
    }
    for elf in &still_elves {
        elves.insert(*elf);
    }
    if elves.len() != elf_count {
        println!("elf lost: {}/{}", elf_count as i32 - elves.len() as i32, elf_count);
        panic!("elf lost");
    }
    return proposed.len() != 0;
}

pub fn solve_a() {
    let input = fileio::input("src/year2022/input/day23.txt");
    let mut elves = parse_input(&input);
    let mut proposed: HashMap<(i32, i32), (i32, i32)> = HashMap::new(); // proposed dest -> source
    const ROUNDS: u32 = 10;
    for round in 0..ROUNDS {
        advance_round(&mut elves, &mut proposed, round);
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
    let input = fileio::input("src/year2022/input/day23.txt");
    let mut elves = parse_input(&input);
    let mut proposed: HashMap<(i32, i32), (i32, i32)> = HashMap::new(); // proposed dest -> source
    let mut round = 0;
    while advance_round(&mut elves, &mut proposed, round) {
        round += 1;
    }
    println!("{}", round + 1);
}