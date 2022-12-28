use crate::fileio;

fn solve(days: u32) {
    let input = fileio::input("src/year2021/input/day6.txt");
    let mut counts: [i64;9] = [0;9];
    for ln in input[0].split(",").collect::<Vec<&str>>() {
        counts[ln.parse::<usize>().unwrap()] += 1;
    }
    for _ in 0..days {
        let mut diffs: [i64;9] = [0;9];
        diffs[8] += counts[0];
        diffs[6] += counts[0];
        diffs[0] -= counts[0];
        for i in 1..9 {
            diffs[i-1] += counts[i];
            diffs[i] -= counts[i];
        }
        for i in 0..9 {
            counts[i] += diffs[i];
        }
    }
    let mut sum = 0;
    for c in &counts {
        sum += *c;
    }
    println!("{}", sum);
}

pub fn solve_a() {
    solve(80);
}

pub fn solve_b() {
    solve(256);
}