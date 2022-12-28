use crate::fileio;

fn to_snafu(mut v: u64) -> Vec<char> {
    let mut output: Vec<char> = Vec::new();
    if v == 0 {
        return vec!['0'];
    }
    while v != 0 {
        v += 2;
        output.push(match v % 5 {
            0 => {'='},
            1 => {'-'},
            2 => {'0'},
            3 => {'1'},
            4 => {'2'},
            _ => {panic!()}
        });
        v /= 5;
    }
    output.reverse();
    return output;
}

fn to_deci(v: Vec<char>) -> u64 {
    let mut output: i64 = 0;
    for c in v {
        output = output * 5 + match c {
            '=' => {-2},
            '-' => {-1},
            '0' => {0},
            '1' => {1},
            '2' => {2}
            _ => {panic!();}
        };
    }
    return output as u64;
}

pub fn solve_a() {
    let input = fileio::input("src/year2022/input/day25.txt");
    let mut sum: u64 = 0;
    for ln in input {
        sum += to_deci(ln.chars().collect::<Vec<char>>());
    }
    let snafu_sum = to_snafu(sum);
    for i in snafu_sum {
        print!("{}", i);
    }
    println!();
}

pub fn solve_b() {
    println!("Pog");
}