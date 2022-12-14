use crate::fileio;

pub fn solve_a() -> i32 {
    let mut input = fileio::input("src/day2.txt");
    let mut score: i32 = 0;
    for ln in input {
        let (mut a, mut b): (i32, i32);
        a = (ln.as_bytes()[0] - 'A' as u8) as i32;
        b = (ln.as_bytes()[2] - 'X' as u8) as i32;
        let d: i32 = (b - a + 4) % 3;
        score += d * 3 + b + 1;
    }
    return score;
}

pub fn solve_b() -> i32 {
    let mut input = fileio::input("src/day2.txt");
    let mut score: i32 = 0;
    for ln in input {
        let (mut a, mut o): (i32, i32);
        a = (ln.as_bytes()[0] - 'A' as u8) as i32;
        o = (ln.as_bytes()[2] - 'X' as u8) as i32;
        let b: i32 = (a + o + 2) % 3;
        score += o * 3 + b + 1;
    }
    return score;
}