use crate::fileio;

pub fn solve_a() {
    let input = fileio::input("src/year2021/input/day2.txt");
    let mut depth: u32 = 0;
    let mut h_pos: u32 = 0;
    for ln in &input {
        let parts: Vec<&str> = ln.split(" ").collect();
        let amount = parts[1].parse::<u32>().unwrap();
        match parts[0] {
            "forward" => {
                h_pos += amount;
            }
            "down" => {
                depth += amount;
            }
            "up" => {
                depth -= amount;
            }
            _ => {
                panic!("invalid instruction");
            }
        }
    }
    println!("{}", h_pos * depth);
}

pub fn solve_b() {
    let input = fileio::input("src/year2021/input/day2.txt");
    let mut depth: i32 = 0;
    let mut h_pos: i32 = 0;
    let mut aim: i32 = 0;
    for ln in &input {
        let parts: Vec<&str> = ln.split(" ").collect();
        let amount = parts[1].parse::<i32>().unwrap();
        match parts[0] {
            "forward" => {
                h_pos += amount;
                depth += amount * aim;
            }
            "down" => {
                aim += amount;
            }
            "up" => {
                aim -= amount;
            }
            _ => {
                panic!("invalid instruction");
            }
        }
    }
    println!("{}", h_pos * depth);

}