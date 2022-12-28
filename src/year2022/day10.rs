use crate::fileio;

pub fn solve_a() {
    let input = fileio::input("src/year2022/input/day10.txt");
    let mut cycle = 0;
    let mut x: i32 = 1;
    let mut sum_sig_vals: i32 = 0;
    let mut nmc: u32 = 20; // next measured cycle
    const MMC: u32 = 220; // max measured cycle
    for ln in input {
        let parts: Vec<&str> = ln.splitn(2, " ").collect();
        let is_add = parts[0] == "addx";
        cycle += if is_add {2} else {1};
        if cycle >= nmc {
            sum_sig_vals += x * nmc as i32;
            nmc = if nmc == MMC {u32::MAX} else {nmc + 40};
        }
        if is_add {
            x += parts[1].parse::<i32>().unwrap();
        }
    }
    while nmc != u32::MAX {
        sum_sig_vals += x * nmc as i32;
        nmc = if nmc == MMC {u32::MAX} else {nmc + 40};
    }
    println!("{}", sum_sig_vals);
}

pub fn solve_b() {
    let input = fileio::input("src/year2022/input/day10.txt");
    let mut pos = 0;
    let mut x: i32 = 1;
    for ln in input {
        let parts: Vec<&str> = ln.splitn(2, " ").collect();
        let is_add = parts[0] == "addx";
        for _ in 0..(if is_add {2} else {1}) {
            print!("{}", if (pos - x).abs() <= 1 {'#'} else {'.'});
            pos += 1;
            if pos == 40 {
                println!();
                pos = 0;
            }
        }
        if is_add {
            x += parts[1].parse::<i32>().unwrap();
        }
    }
}