use crate::fileio;

pub fn solve_a() {
    let mut input = fileio::input("src/day4.txt");
    let mut count = 0;
    for ln in input {
        let (mut l1, mut r1) = (0, 0);
        let (mut l2, mut r2) = (0, 0);
        for c in ln.bytes() {
            if c == '-' as u8 {
                l2 = r2;
                r2 = 0;
            } else if c == ',' as u8 {
                l1 = l2;
                r1 = r2;
                r2 = 0;
            } else {
                r2 = r2 * 10 + c - '0' as u8;
            }
        }
        count += ((l1 <= l2 && r1 >= r2) || (l1 >= l2 && r1 <= r2)) as i32;
    }
    println!("{}", count);
}

pub fn solve_b() {
    let mut input = fileio::input("src/day4.txt");
    let mut count = 0;
    for ln in input {
        let (mut l1, mut r1) = (0, 0);
        let (mut l2, mut r2) = (0, 0);
        for c in ln.bytes() {
            if c == '-' as u8 {
                l2 = r2;
                r2 = 0;
            } else if c == ',' as u8 {
                l1 = l2;
                r1 = r2;
                r2 = 0;
            } else {
                r2 = r2 * 10 + c - '0' as u8;
            }
        }
        count += !(l1 > r2 || r1 < l2) as i32;
    }
    println!("{}", count);
}