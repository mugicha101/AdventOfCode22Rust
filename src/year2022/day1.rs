use crate::fileio;

pub fn solve_a() {
    let mut input = fileio::input("src/year2022/input/day1.txt");
    input.push("".to_string());
    let mut max = 0;
    let mut curr = 0;
    for ln in input {
        if ln == "" {
            if curr > max {
                max = curr;
            }
            curr = 0;
        } else {
            let val: i32 = ln.parse::<i32>()
                .expect("parse failed");
            curr += val;
        }
    }
    println!("{}", max);
}

pub fn solve_b() {
    let mut input = fileio::input("src/year2022/input/day1.txt");
    input.push("".to_string());
    let mut max: [i32; 3] = [0; 3];
    let mut mi = 0;
    let mut curr = 0;
    for ln in input {
        if ln == "" {
            if curr > max[mi] {
                max[mi] = curr;
                mi = 0;
                for (i, el) in max.iter().enumerate() {
                    if el < &max[mi] {
                        mi = i;
                    }
                }
            }
            curr = 0;
        } else {
            let val: i32 = ln.parse::<i32>()
                .expect("parse failed");
            curr += val;
        }
    }
    let mut sum_max = 0;
    for el in max {
        sum_max += el;
    }
    println!("{}", sum_max);
}