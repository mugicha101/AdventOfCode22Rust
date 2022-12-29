use std::cmp::min;
use crate::fileio;

pub fn solve_a() {
    let raw_input = fileio::input("src/year2021/input/day7.txt");
    let input: Vec<&str> = raw_input[0].split(",").collect();
    let n = input.len();
    let mut crabs: Vec<i32> = vec![0;n];
    for i in 0..n {
        crabs[i] = input[i].parse::<i32>().unwrap();
    }
    crabs.sort();
    let mut cost = 0;
    for c in &crabs {
        cost += c;
    }
    cost -= crabs[0] * n as i32;
    let mut min_cost = cost;
    for i in 1..n {
        let d = crabs[i] - crabs[i-1];
        cost += d * i as i32 - d * (n-i) as i32;
        min_cost = min(min_cost, cost);
    }
    println!("{}", min_cost);
}

pub fn solve_b() {
    let raw_input = fileio::input("src/year2021/input/day7.txt");
    let input: Vec<&str> = raw_input[0].split(",").collect();
    let n = input.len();
    let mut crabs: Vec<i32> = vec![0;n];
    for i in 0..n {
        crabs[i] = input[i].parse::<i32>().unwrap();
    }
    // c(n) = fuel cost of crab movement when position n chosen
    // c(n) = c(n-1) + dc(n)
    // dc(n) = dc(n-1) + ddc(n)
    // ddc(n) = 10 + # of crabs with value n-1
    // c(0) = sum(for k in crabs, k(k+1)/2)
    // c(1) = c(0) + dc(1)
    // dc(1) = -(sum of elements in crabs) + # of 0s in crabs
    let mut c0 = 0;
    let mut dc1 = 0;
    for k in &crabs {
        c0 += (*k * (*k +1)) >> 1;
        dc1 += -(*k) + (*k == 0) as i32;
    }
    // dc(1) + (n-1) * crabs.len()
    // <= dc(n)
    // = dc(1) + (n-1) * crabs.len() + # of crabs in [1,n)
    // <= dc(1) + n * crabs.len()
    // dc(1) + nl * crabs.len() = 0
    // nl = -dc(1) / crabs.len()
    // nl < n < nl + 1
    let mut n = -dc1 / crabs.len() as i32;
    while {
        let mut dcn = dc1 + (n-1) * crabs.len() as i32;
        for k in &crabs {
            dcn += if *k >= 1 && *k < n { 1 } else { 0 };
        }
        dcn <= 0
    } {
        n += 1;
    }
    n -= 1;
    // c(n) = c(0) + n*dc(1) + (n(n-1)/2)*crabs.len() + sum(for k in crabs, if k in [1,n), n-k)
    let mut cn = c0 + n * dc1 + (n * (n - 1) >> 1) * crabs.len() as i32;
    for k in &crabs {
        cn += if *k >= 1 && *k < n { n - k } else { 0 };
    }
    println!("{}", cn);
}