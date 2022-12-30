use std::cmp::{max, min};
use crate::fileio;

fn parse_input(input: &Vec<String>) -> Vec<bool> {
    let line = input[0].as_bytes();
    let mut binary: Vec<bool> = vec![false; line.len() << 2];
    for i in 0..line.len() {
        let offset = i << 2;
        let val = if line[i] >= 'A' as u8 && line[i] <= 'F' as u8 {line[i] - 'A' as u8 + 10} else {line[i] - '0' as u8};
        for j in 0..4 {
            binary[offset+j] = (val & (1 << (3-j))) != 0;
        }
    }
    return binary;
}

fn get_val(binary: &Vec<bool>, offset: usize, length: usize) -> u64 {
    let mut val: u64 = 0;
    for i in 0..length {
        val = (val << 1) + binary[offset+i] as u64;
    }
    return val;
}

pub fn solve_a() {
    let input = fileio::input("src/year2021/input/day16.txt");
    let binary = parse_input(&input);
    let mut i = 0;
    let mut ver_sum = 0;
    while i+6 < binary.len() {
        let ver = get_val(&binary, i, 3);
        ver_sum += ver;
        let id = get_val(&binary, i + 3, 3);
        i += 6;
        if id == 4 {
            while binary[i] {
                i += 5;
            }
            i += 5;
            continue;
        }
        if binary[i] {
            i += 12;
        } else {
            i += 16;
        }
    }
    println!("{}", ver_sum);
}

pub fn solve_b() {

    let input = fileio::input("src/year2021/input/day16.txt");
    let binary = parse_input(&input);
    fn parse_packet(binary: &Vec<bool>, mut i: usize) -> (usize, u64) {
        let id = get_val(&binary, i + 3, 3);
        i += 6;
        if id == 4 {
            let mut val = 0;
            while binary[i] {
                val = (val << 4) + get_val(binary, i+1, 4);
                i += 5;
            }
            val = (val << 4) + get_val(binary, i+1, 4);
            return (i + 5, val);
        }
        let len_type = binary[i];
        i += 1;
        let mut vals: Vec<u64> = Vec::new();
        if len_type {
            let packets = get_val(binary, i, 11);
            i += 11;
            for _ in 0..packets {
                let res = parse_packet(binary, i);
                i = res.0;
                vals.push(res.1);
            }
        } else {
            let end = i + 15 + get_val(binary, i, 15) as usize;
            i += 15;
            while i < end {
                let res = parse_packet(binary, i);
                i = res.0;
                vals.push(res.1);
            }
        }
        return (i, match id {
            0 => {
                let mut out = 0;
                for v in vals {
                    out += v;
                }
                out
            }
            1 => {
                let mut out = 1;
                for v in vals {
                    out *= v;
                }
                out
            }
            2 => {
                let mut out = u64::MAX;
                for v in vals {
                    out = min(out, v);
                }
                out
            }
            3 => {
                let mut out = u64::MIN;
                for v in vals {
                    out = max(out, v);
                }
                out
            }
            5 => { (vals[0] > vals[1]) as u64 }
            6 => { (vals[0] < vals[1]) as u64 }
            7 => { (vals[0] == vals[1]) as u64 }
            _ => { panic!(); }
        });
    }
    let res = parse_packet(&binary, 0);
    println!("{}", res.1);
}