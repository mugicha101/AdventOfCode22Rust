use crate::fileio;

fn parse_input(input: &Vec<String>) -> Vec<([Vec<usize>; 10], [Vec<usize>; 4])> {
    let mut signals: Vec<([Vec<usize>;10], [Vec<usize>;4])> = Vec::new();
    let parse_part = |part: &str| -> Vec<usize> {
        let mut sig: Vec<usize> = Vec::new();
        for c in part.bytes() {
            sig.push((c - 'a' as u8) as usize);
        }
        return sig;
    };
    for ln in input {
        let halves: Vec<&str> = ln.split(" | ").collect();
        let mut row: ([Vec<usize>; 10], [Vec<usize>; 4]) = ([
                                                                Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new(),
                                                                Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new()
                                                            ], [Vec::new(),Vec::new(),Vec::new(),Vec::new()]);
        let mut parts: Vec<&str> = halves[0].split(" ").collect();
        for i in 0..10 {
            row.0[i] = parse_part(&parts[i]);
        }
        parts = halves[1].split(" ").collect();
        for i in 0..4 {
            row.1[i] = parse_part(&parts[i]);
        }
        signals.push(row);
    }
    return signals;
}

pub fn solve_a() {
    let input = fileio::input("src/year2021/input/day8.txt");
    let signals = parse_input(&input);
    let mut count: u32 = 0;
    const MAP: [u32;8] = [0,0,1,1,1,0,0,1];
    for row in &signals {
        for sig in &row.1 {
            count += MAP[sig.len()];
        }
    }
    println!("{}", count);
}

pub fn solve_b() {
    let input = fileio::input("src/year2021/input/day8.txt");
    let signals = parse_input(&input);
    let dig_segs: [u8;10] = [0b1110111, 0b0100100, 0b1011101, 0b1101101, 0b0101110, 0b1101011, 0b1111011, 0b0100101, 0b1111111, 0b1101111];
    let mut sum: u32 = 0;
    for row in &signals {
        let mut sym_counts: [u8;7] = [0;7];
        let mut i1 = 0;
        let mut i4 = 0;
        for i in 0..10 {
            let sig = &row.0[i];
            if sig.len() == 2 {
                i1 = i;
            } else if sig.len() == 4 {
                i4 = i;
            }
            for sym in sig {
                sym_counts[*sym] += 1;
            }
        }
        let mut sym_seg: [u8;7] = [0;7];
        for sym in 0..7 {
            sym_seg[sym] = match sym_counts[sym] {
                4 => {4},
                6 => {1},
                7 => {if row.0[i4].contains(&sym) {3} else {6}},
                8 => {if row.0[i1].contains(&sym) {2} else {0}},
                9 => {5},
                _ => {panic!();}
            }
        }
        let mut val: u32 = 0;
        for i in 0..4 {
            let sig = &row.1[i];
            let mut segs = 0b0000000;
            for sym in sig {
                segs |= 1 << sym_seg[*sym];
            }
            val = val * 10 + dig_segs.iter().position(|&r| r == segs).unwrap() as u32;
        }
        sum += val;
    }
    println!("{}", sum);
}