use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use crate::fileio;

fn parse_input(input: &Vec<String>) -> (Vec<Vec<usize>>, usize, usize) {
    // nodes: small caves
    // edge: path from one small cave to another
    let mut symbol_table: HashMap<String, usize> = HashMap::new();
    let mut is_big: Vec<bool> = Vec::new();
    let mut neighbors: Vec<Vec<usize>> = Vec::new();
    let mut start_index = usize::MAX;
    let mut end_index = usize::MAX;

    // resolve symbols
    for ln in input {
        let parts: Vec<&str> = ln.splitn(2,"-").collect();
        for p in parts {
            if !symbol_table.contains_key(p) {
                let l = symbol_table.len();
                symbol_table.insert(String::from(p), l);
                let b = p.as_bytes()[0];
                is_big.push(b >= 'A' as u8 && b <= 'Z' as u8);
                neighbors.push(Vec::new());
                if p == "start" {
                    start_index = l;
                } else if p == "end" {
                    end_index = l;
                }
            }
        }
    }
    let len = symbol_table.len();

    // resolve neighbors
    for ln in input {
        let parts: Vec<&str> = ln.splitn(2,"-").collect();
        let i = symbol_table[parts[0]];
        let j = symbol_table[parts[1]];
        neighbors[i].push(j);
        neighbors[j].push(i);
    }

    // flatten paths to big caves into paths to small caves
    for i in 0..len {
        if !is_big[i] {
            continue;
        }
        let mut nb_vec: Vec<usize> = Vec::new();
        for n in &neighbors[i] {
            nb_vec.push(*n);
        }
        for j in 0..nb_vec.len() {
            let mut count = 0;
            for n in &neighbors[nb_vec[j]] {
                if *n == i {
                    count += 1;
                }
            }
            let mut pos_vec: Vec<usize> = Vec::new();
            let mut pos = 0;
            for n in &neighbors[nb_vec[j]] {
                if *n == i {
                    pos_vec.push(pos);
                }
                pos += 1;
            }
            for p in pos_vec.iter().rev() {
                neighbors[nb_vec[j]].remove(*p);
            }
            for k in 0..nb_vec.len() {
                if nb_vec[k] == i {
                    continue;
                }
                for _ in 0..count {
                    neighbors[nb_vec[j]].push(nb_vec[k]);
                }
            }
        }
        neighbors[i].clear();
    }

    // remove big caves
    let mut small_neighbors: Vec<Vec<usize>> = Vec::new();
    let mut new_indices: Vec<usize> = Vec::new();
    let mut next_index: usize = 0;
    for i in 0..len {
        if is_big[i] {
            new_indices.push(usize::MAX);
            continue;
        }
        new_indices.push(next_index);
        next_index += 1;
        small_neighbors.push(neighbors[i].clone());
    }
    for i in 0..small_neighbors.len() {
        for n in &mut small_neighbors[i] {
            *n = new_indices[*n];
        }
    }
    return (small_neighbors, new_indices[start_index], new_indices[end_index]);
}

pub fn solve_a() {
    let input = fileio::input("src/year2021/input/day12.txt");
    let (adj_list, si, ei) = parse_input(&input);
    let visited: Rc<RefCell<Vec<bool>>> = Rc::new(RefCell::new(vec![false;adj_list.len()]));
    visited.borrow_mut()[si] = true;
    fn dfs(adj_list: &Vec<Vec<usize>>, visited: Rc<RefCell<Vec<bool>>>, i: usize, ei: usize) -> u32 {
        if i == ei {
            return 1;
        }
        let mut paths = 0;
        for n in &adj_list[i] {
            if visited.borrow()[*n] {
                continue;
            }
            visited.borrow_mut()[*n] = true;
            paths += dfs(adj_list, visited.clone(), *n, ei);
            visited.borrow_mut()[*n] = false;
        }
        return paths;
    }
    let paths = dfs(&adj_list, visited.clone(), si, ei);
    println!("{}", paths);
}

pub fn solve_b() {
    let input = fileio::input("src/year2021/input/day12.txt");
    let (mut adj_list, si, ei) = parse_input(&input);
    let mut adj_mat: Vec<Vec<u32>> = vec![vec![0;adj_list.len()];adj_list.len()];
    for i in 0..adj_list.len() {
        for n in &adj_list[i] {
            adj_mat[i][*n] += 1;
            adj_mat[*n][i] += 1;
        }
    }
    for i in 0..adj_list.len() {
        for j in 0..adj_list.len() {
            adj_mat[i][j] >>= 1;
        }
    }
    let visited: Rc<RefCell<Vec<bool>>> = Rc::new(RefCell::new(vec![false;adj_list.len()]));
    visited.borrow_mut()[si] = true;
    fn dfs(adj_mat: &Vec<Vec<u32>>, visited: Rc<RefCell<Vec<bool>>>, i: usize, paths: u32, si: usize, ei: usize, dup_used: bool) -> u32 {
        if i == ei {
            return paths;
        }
        let mut path_total = 0;
        for j in 0..adj_mat.len() {
            if adj_mat[i][j] == 0 {
                continue;
            }
            let use_dup = visited.borrow()[j];
            if use_dup && (dup_used || j == si) {
                continue;
            }
            visited.borrow_mut()[j] = true;
            path_total += dfs(adj_mat, visited.clone(), j, paths * adj_mat[i][j], si, ei, use_dup | dup_used);
            visited.borrow_mut()[j] = use_dup;
        }
        return path_total;
    }
    let paths = dfs(&adj_mat, visited.clone(), si, 1, si, ei, false);
    println!("{}", paths);
}