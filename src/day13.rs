use std::cmp::min;
use std::str::from_utf8;
use crate::fileio;

fn is_arr(s: &str) -> bool {
    s.len() >= 2 && s.as_bytes()[0] == '[' as u8 && s.as_bytes()[s.len() - 1] == ']' as u8
}

fn compare(lines: &[String; 2]) -> i32 {
    if !is_arr(&lines[0]) && !is_arr(&lines[1]) {
        return (lines[1].parse::<i32>().unwrap() - lines[0].parse::<i32>().unwrap()).signum();
    }
    let split_str = |s: &String| -> Vec<String> {
        if !is_arr(s) {
            return [s.clone()].to_vec();
        }
        let txt = s[1..s.len()-1].as_bytes();
        let mut start: usize = 0;
        let mut bal: i32 = 0;
        let mut parts: Vec<String> = Vec::new();
        for i in 0..txt.len() {
            if txt[i] == '[' as u8 {
                bal += 1;
            } else if txt[i] == ']' as u8 {
                bal -= 1;
            } else if txt[i] == ',' as u8 && bal == 0 {
                parts.push(String::from(from_utf8(&txt[start..i]).unwrap()));
                start = i+1;
            }
        }
        if start < txt.len() {
            parts.push(String::from(from_utf8(&txt[start..txt.len()]).unwrap()));
        }
        return parts;
    };
    let parts: [Vec<String>; 2] = [
        split_str(&lines[0]),
        split_str(&lines[1]),
    ];
    let min_len = min(parts[0].len(), parts[1].len());
    for i in 0..min_len {
        let c = compare(&[parts[0][i].clone(), parts[1][i].clone()]);
        if c != 0 {
            return c;
        }
    }
    return (parts[1].len() as i32 - parts[0].len() as i32).signum();
}

pub fn solve_a() {
    let input = fileio::input("src/day13.txt");
    let mut sum = 0;
    let mut p_num = 1;
    for i in (0..input.len()).step_by(3) {
        let c = compare(&[input[i].clone(), input[i+1].clone()]);
        sum += if c == 1 {p_num} else {0};
        p_num += 1;
    }
    println!("{}", sum);
}

pub fn solve_b() {
    let input = fileio::input("src/day13.txt");
    let mut packets: Vec<String> = Vec::new();
    packets.push(String::from("[[2]]"));
    packets.push(String::from("[[6]]"));
    for i in (0..input.len()).step_by(3) {
        packets.push(input[i].clone());
        packets.push(input[i+1].clone())
    }
    packets.sort_unstable_by(|a, b| compare(&[a.clone(), b.clone()]).cmp(&0).reverse());
    let mut decoder_key = 1;
    for i in 0..packets.len() {
        if packets[i] == "[[2]]" || packets[i] == "[[6]]" {
            decoder_key *= (i+1);
        }
    }
    println!("{}", decoder_key);
}