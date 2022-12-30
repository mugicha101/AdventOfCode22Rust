use std::cmp::{max, min};
use std::collections::HashMap;
use crate::fileio;

fn parse_input(input: &Vec<String>) -> (HashMap<(u8, u8), usize>, Vec<u8>, Vec<Vec<usize>>, Vec<u8>) {
    let mut pair_index: HashMap<(u8,u8), usize> = HashMap::new(); // pair -> pair index
    let mut pair_char: Vec<u8> = Vec::new(); // pair index -> new char
    let mut pair_desc: Vec<Vec<usize>> = Vec::new(); // pair index -> descendant pair indices
    for l in 2..input.len() {
        let line = input[l].as_bytes();
        pair_index.insert((line[0], line[1]), pair_index.len());
        pair_char.push(line[6]);
        pair_desc.push(Vec::new());
    }
    for (pair, index) in &pair_index {
        let c = pair_char[*index];
        if pair_index.contains_key(&(pair.0, c)) {
            pair_desc[*index].push(pair_index[&(pair.0, c)]);
        }
        if pair_index.contains_key(&(c, pair.1)) {
            pair_desc[*index].push(pair_index[&(c, pair.1)]);
        }
    }
    let mut start_state: Vec<u8> = Vec::new();
    for b in input[0].bytes() {
        start_state.push(b);
    }
    return (pair_index, pair_char, pair_desc, start_state);
}

fn solve(rounds: u32) {
    let input = fileio::input("src/year2021/input/day14.txt");
    let (pair_index, pair_char, pair_desc, start_state) = parse_input(&input);
    let mut pair_counter: Vec<u64> = vec![0;pair_index.len()]; // # of each pair on current state
    let mut char_counter: [u64;26] = [0;26]; // # of each char on current state
    for i in 0..start_state.len()-1 {
        if pair_index.contains_key(&(start_state[i], start_state[i+1])) {
            pair_counter[pair_index[&(start_state[i], start_state[i+1])]] += 1;
        }
        char_counter[(start_state[i] - 'A' as u8) as usize] += 1;
    }
    char_counter[(start_state[start_state.len()-1] - 'A' as u8) as usize] += 1;
    for _ in 0..rounds {
        let mut new_pair_counter = vec![0; pair_counter.len()];
        for i in 0..pair_counter.len() {
            char_counter[(pair_char[i] - 'A' as u8) as usize] += pair_counter[i];
            for desc in &pair_desc[i] {
                new_pair_counter[*desc] += pair_counter[i];
            }
        }
        pair_counter = new_pair_counter;
    }
    let mut min_count = u64::MAX;
    let mut max_count = u64::MIN;
    for count in &char_counter {
        if *count == 0 {
            continue;
        }
        min_count = min(min_count, *count);
        max_count = max(max_count, *count);
    }
    println!("{}", max_count - min_count);
}

pub fn solve_a() {
    solve(10);
}

pub fn solve_b() {
    solve(40);
}