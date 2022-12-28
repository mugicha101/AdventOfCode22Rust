use std::collections::VecDeque;
use crate::fileio;

pub fn solve_a() {
    let input = fileio::input("src/year2021/input/day3.txt");
    let n = input[0].len();
    let mut bit_count: Vec<i32> = vec![0;n];
    for ln in input {
        let line = ln.as_bytes();
        for i in 0..n {
            bit_count[i] += if line[i] == '1' as u8 {1} else {-1};
        }
    }
    let mut gamma = 0;
    for i in 0..n {
        gamma = (gamma << 1) + if bit_count[i] > 0 {1} else {0};
    }
    let power = gamma * ((1 << n) - gamma - 1);
    println!("{}", power);
}

pub fn solve_b() {
    let input = fileio::input("src/year2021/input/day3.txt");
    let n = input[0].len();
    let mut vals: Vec<i32> = vec![0;input.len()];
    let mut q: VecDeque<usize> = VecDeque::new();
    for i in 0..input.len() {
        let line = input[i].as_bytes();
        for j in 0..n {
            vals[i] = (vals[i] << 1) + if line[j] == '1' as u8 {1} else {0};
        }
        q.push_back(i);
    }
    let mut m = 1 << n;
    while q.len() != 1 {
        m >>= 1;
        // get common bit
        let mut bal: i32 = 0;
        for index in &q {
            bal += if vals[*index] & m != 0 {1} else {-1};
        }
        // filter
        let a = q.len();
        for _ in 0..a {
            let i = q.pop_front().unwrap();
            if (vals[i] & m == 0) ^ (bal < 0) {
                q.push_back(i);
            }
        }
    }
    let o2val = vals[q.pop_back().unwrap()];
    for i in 0..vals.len() {
        q.push_back(i);
    }
    m = 1 << n;
    while q.len() != 1 {
        m >>= 1;
        // get common bit
        let mut bal = 0;
        for index in &q {
            bal += if vals[*index] & m != 0 {1} else {-1};
        }
        // filter
        let a = q.len();
        for _ in 0..a {
            let i = q.pop_front().unwrap();
            if (vals[i] & m != 0) ^ (bal < 0) {
                q.push_back(i);
            }
        }
    }
    let co2val = vals[q.pop_back().unwrap()];
    println!("{}", o2val * co2val);
}