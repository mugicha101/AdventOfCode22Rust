use std::collections::VecDeque;
use crate::fileio;
fn parse_input(input: &Vec<String>) -> (Vec<i64>, Vec<(usize, usize)>) {
    let mut output: Vec<i64> = Vec::new();
    for ln in input {
        output.push(ln.parse::<i64>().unwrap());
    }
    let mut links: Vec<(usize, usize)> = Vec::new();
    links.push((input.len() - 1, 1));
    for i in 2..input.len() {
        links.push((i-2, i));
    }
    links.push((input.len()-2, 0));
    return (output, links);
}

fn find_sum(input: &Vec<i64>, links: &Vec<(usize, usize)>) -> i64 {
    let mut index = 0;
    while input[index] != 0 {
        index = links[index].1;
    }
    let mut sum = 0;
    for _ in 0..3 {
        for _ in 0..1000 {
            index = links[index].1;
        }
        sum += input[index];
    }
    return sum;
}

fn mix(input: &Vec<i64>, links: &mut Vec<(usize, usize)>) {
    let m = input.len() as i64 - 1;
    for c in 0..input.len() {
        let i = input[c];

        // go to index
        let mut index = c;

        // remove from linked list
        {
            let prev = links[index].0;
            let next = links[index].1;
            links[prev].1 = next;
            links[next].0 = prev;
            index = prev;
        }

        // traverse linked list
        if i >= 0 {
            let c = i % m;
            for _ in 0..c {
                index = links[index].1;
            }
        } else {
            let c = -i % m;
            for _ in 0..c {
                index = links[index].0;
            }
        }

        // insert into linked list
        {
            let next = links[index].1;
            links[next].0 = c;
            links[index].1 = c;
            links[c] = (index, next);
        }
    }
}

pub fn solve_a() {
    let (input, mut links) = parse_input(&fileio::input("src/day20.txt"));
    mix(&input, &mut links);
    println!("{}", find_sum(&input, &links));
}

pub fn solve_b() {
    let (mut input, mut links) = parse_input(&fileio::input("src/day20.txt"));
    for i in &mut input {
        *i *= 811589153;
    }
    for _ in 0..10 {
        mix(&input, &mut links);
    }
    println!("{}", find_sum(&input, &links));
}