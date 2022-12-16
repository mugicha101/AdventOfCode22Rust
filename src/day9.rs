use std::collections::HashSet;
use crate::fileio;

pub fn solve_a() {
    let input = fileio::input("src/day9.txt");
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut head = (0, 0);
    let mut tail = (0, 0);
    for ln in input {
        let parts: Vec<&str> = ln.splitn(2, " ").collect();
        let dir = parts[0];
        let amount = parts[1].parse::<u32>().unwrap();
        let mut mark = |p: (i32, i32)| { visited.insert(p) };
        match dir {
            "L" => {
                for _ in 0..amount {
                    head.0 -= 1;
                    if tail.0 == head.0 + 2 {
                        tail.0 = head.0 + 1;
                        tail.1 = head.1;
                    }
                    mark(tail);
                }
            }
            "R" => {
                for _ in 0..amount {
                    head.0 += 1;
                    if tail.0 == head.0 - 2 {
                        tail.0 = head.0 - 1;
                        tail.1 = head.1;
                    }
                    mark(tail);
                }
            }
            "D" => {
                for _ in 0..amount {
                    head.1 -= 1;
                    if tail.1 == head.1 + 2 {
                        tail.1 = head.1 + 1;
                        tail.0 = head.0;
                    }
                    mark(tail);
                }
            }
            "U" => {
                for _ in 0..amount {
                    head.1 += 1;
                    if tail.1 == head.1 - 2 {
                        tail.1 = head.1 - 1;
                        tail.0 = head.0;
                    }
                    mark(tail);
                }
            }
            _ => {
                panic!("invalid input direction")
            }
        }
    }
    println!("{}", visited.len());
}

pub fn solve_b() {
    let input = fileio::input("src/day9.txt");
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut segs: Vec<(i32, i32)> = vec!((0, 0); 10);
    for ln in input {
        let parts: Vec<&str> = ln.splitn(2, " ").collect();
        let dir = parts[0];
        let amount = parts[1].parse::<u32>().unwrap();
        let mut mark = |p: &(i32, i32)| { visited.insert(*p) };
        fn traverse(segs: &mut Vec<(i32, i32)>, index: usize) {
            {
                let s = &mut segs[index - 1..=index];
                let d = (s[0].0 - s[1].0, s[0].1 - s[1].1);
                if d.0.abs() == 2 && d.1.abs() == 2 {
                    s[1].0 += d.0.signum();
                    s[1].1 += d.1.signum();
                } else if d.0.abs() == 2 {
                    s[1].0 += d.0.signum();
                    s[1].1 = s[0].1;
                } else if d.1.abs() == 2 {
                    s[1].1 += d.1.signum();
                    s[1].0 = s[0].0;
                }
            }
            if index != 9 {
                traverse(segs, index + 1);
            }
        }
        for _ in 0..amount {
            {
                let head = &mut segs[0];
                match dir {
                    "L" => {
                        head.0 -= 1;
                    }
                    "R" => {
                        head.0 += 1;
                    }
                    "D" => {
                        head.1 -= 1;
                    }
                    "U" => {
                        head.1 += 1;
                    }
                    _ => {
                        panic!("invalid input direction")
                    }
                }
            }
            traverse(&mut segs, 1);
            mark(segs.last().unwrap());
        }
    }
    println!("{}", visited.len());
}