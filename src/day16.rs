use std::cell::RefCell;
use std::cmp::max;
use std::collections::{HashMap, VecDeque};
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;
use priority_queue::PriorityQueue;
use crate::fileio;

struct Valve {
    flow: u32,
    dests: Vec<u32>,
    neighbors: Vec<usize>,
}

fn get_valves(input: &Vec<String>) -> (usize, Vec<Valve>, Vec<usize>) {
    // resolve names
    let mut name_map: HashMap<String, usize> = HashMap::new();
    for ln in input {
        let parts: Vec<&str> = ln.split(" ").collect();
        name_map.insert(parts[1].parse().unwrap(), name_map.len());
    }
    // read input
    let mut valves: Vec<Valve> = Vec::new();
    let mut valve_neighbors: Vec<Vec<usize>> = Vec::new();
    let valve_count = input.len();
    for ln in input {
        let parts: Vec<&str> = ln.split(" ").collect();
        let index = valves.len();
        valve_neighbors.push({
            let mut neighbors: Vec<usize> = Vec::new();
            for i in 9..parts.len() {
                let key = if i == parts.len() - 1 {&parts[i]} else {&parts[i][0..parts[i].len()-1]};
                neighbors.push((name_map[key]) as usize);
            }
            neighbors
        });
        valves.push(Valve {
            flow: parts[4][5..parts[4].len()-1].parse::<u32>().unwrap(),
            dests: {
                let mut dests: Vec<u32> = vec![30; valve_count];
                dests[index] = 0;
                for n in &valve_neighbors[index] {
                    dests[*n] = 1;
                }
                dests
            },
            neighbors: valve_neighbors[index].clone()
        });
    }

    // dijkstras to get dist to each node from each node
    const DIST_CAP: u32 = u32::MAX;
    for i in 0..valves.len() {
        let mut pq: PriorityQueue<usize, u32> = PriorityQueue::new();
        for n in &valve_neighbors[i] {
            pq.push(*n, DIST_CAP - 1);
        }
        while pq.len() != 0 {
            let (index, inv_dist): (usize, u32) = pq.pop().unwrap();
            let dist = DIST_CAP - inv_dist;
            for n in &valve_neighbors[index] {
                let mut d: &mut u32 = &mut valves[i].dests[*n];
                if dist+1 < *d {
                    *d = dist+1;
                    pq.push(*n, DIST_CAP - *d);
                }
            }
        }
    }
    let mut indices: Vec<usize> = Vec::new();
    for i in 0..valves.len() {
        if valves[i].flow != 0 {
            indices.push(i);
        }
    }
    return (name_map["AA"], valves, indices);
}

fn dfs(
    valves: &Vec<Valve>, indices: &Vec<usize>,
    visited: Rc<RefCell<Vec<bool>>>,
    index: usize, time: u32
) -> u32 {
    let mut mf = 0;
    for i in indices {
        if visited.borrow()[*i] {
            continue
        }
        let time_cost = valves[index].dests[*i] + 1;
        if time_cost > time {
            continue;
        }
        let n_time = time - time_cost;
        let f = valves[*i].flow * n_time;
        visited.borrow_mut()[*i] = true;
        mf = max(mf, f + dfs(valves, indices, visited.clone(), *i, n_time));
        visited.borrow_mut()[*i] = false;
    }
    return mf;
}

pub fn solve_a() {
    let input = fileio::input("src/day16.txt");
    let (start_index, valves, indices) = get_valves(&input);
    const TIME: u32 = 30;

    // dfs
    let visited: Rc<RefCell<Vec<bool>>> = Rc::new(RefCell::new(vec![true; valves.len()]));
    for i in &indices {
        visited.borrow_mut()[*i] = false;
    }
    visited.borrow_mut()[start_index] = true;
    println!("{}", dfs(&valves, &indices, visited.clone(), start_index, TIME));
}

pub fn solve_b() {
    let input = fileio::input("src/day16.txt");
    let (start_index, valves, indices) = get_valves(&input);
    let indices = Arc::new(indices);
    let valves = Arc::new(valves);
    const TIME: u32 = 26;

    // dfs
    let mut max_flow: u32 = 0;
    let nums: u64 = 1 << indices.len();
    let mut threads = Vec::new();
    for m in 0..nums {
        let indices = indices.clone();
        let mut n: Arc<u64> = Arc::new(1);
        let valves = valves.clone();
        let indices = indices.clone();
        threads.push(thread::spawn(move || -> u32 {
            let mut i1: Vec<usize> = Vec::new();
            let mut i2: Vec<usize> = Vec::new();
            let mut n = *n.clone();
            for i in 0..indices.len() {
                if m & n == 0 {
                    i1.push(indices[i]);
                } else {
                    i2.push(indices[i]);
                }
                n <<= 1;
            }
            let visited: Rc<RefCell<Vec<bool>>> = Rc::new(RefCell::new(vec![false; valves.len()]));
            visited.borrow_mut()[0] = true;
            let mut setup_visited = |ind: &Vec<usize>| {
                for v in visited.borrow_mut().iter_mut() {
                    *v = true;
                }
                for i in ind {
                    visited.borrow_mut()[*i] = false;
                }
                visited.borrow_mut()[start_index] = true;
            };
            let flow_sum = {
                setup_visited(&i1);
                dfs(&valves, &i1, visited.clone(), start_index, TIME)
            } + {
                setup_visited(&i2);
                dfs(&valves, &i2, visited.clone(), start_index, TIME)
            };
            return flow_sum;
        }));
    }
    for t in threads {
        let r = t.join();
        max_flow = max(max_flow, r.unwrap());
    }
    println!("{}", max_flow);
}