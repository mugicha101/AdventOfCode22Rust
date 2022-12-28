use crate::fileio;

#[inline(always)]
fn char_index(c: u8) -> usize {
    (if c & 32 != 0 {c - 'a' as u8} else {26 + c - 'A' as u8}) as usize
}

pub fn solve_a() {
    let mut input = fileio::input("src/year2022/input/day3.txt");
    let mut psum: i32 = 0;
    for ln in input {
        let mut m: [bool; 52] = [false; 52];
        let h = ln.len() >> 1;
        for i in 0..h {
            let c = ln.as_bytes()[i];
            m[char_index(c)] = true;
        }
        let mut found = 0;
        for i in h..ln.len() {
            let c = ln.as_bytes()[i];
            let i = char_index(c);
            if m[i] {
                found = i + 1;
                break;
            }
        }
        psum += found as i32;
    }
    println!("{}", psum);
}

pub fn solve_b() {
    let mut input = fileio::input("src/year2022/input/day3.txt");
    let mut psum: i32 = 0;
    for i in (0..input.len()).step_by(3) {
        let ln1 = input[i].bytes();
        let ln2 = input[i+1].bytes();
        let ln3 = input[i+2].bytes();
        let mut m: [u8; 52] = [0; 52];
        for i in ln1 {
            m[char_index(i)] = 1;
        }
        for i in ln2 {
            m[char_index(i)] |= 2;
        }
        let mut found: i32 = 0;
        for i in ln3 {
            let ci = char_index(i);
            if m[ci] == 3 {
                found = (ci + 1) as i32;
                break;
            }
        }
        psum += found;
    }
    println!("{}", psum);
}