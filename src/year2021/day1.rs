use crate::fileio;

pub fn solve_a() {
    let input = fileio::input("src/year2021/input/day1.txt");
    let mut prev = u32::MAX;
    let mut count = 0;
    for ln in input {
        let v = ln.parse::<u32>().unwrap();
        if v > prev {
            count += 1;
        }
        prev = v;
    }
    println!("{}", count);
}

pub fn solve_b() {
    let input = fileio::input("src/year2021/input/day1.txt");
    let mut ints: Vec<u32> = vec![0;input.len()];
    for i in 0..input.len() {
        ints[i] = input[i].parse::<u32>().unwrap();
    }
    let mut prev = input[0].parse::<u32>().unwrap();
    let mut count = 0;
    for i in 3..input.len() {
        count += if ints[i] > prev {1} else {0};
        prev = ints[i-2];
    }
    println!("{}", count);
}