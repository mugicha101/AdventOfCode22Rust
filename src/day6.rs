use crate::fileio;

fn solve(l: usize) {
    let binding = fileio::input("src/day6.txt");
    let input = binding[0].as_bytes();
    let mut chars: [u8; 26] = [0; 26];
    let mut dups = 0;
    let mut num = 0;
    for i in 0..input.len() {
        let mut c = (input[i] - 'a' as u8) as usize;
        chars[c] += 1;
        dups += if chars[c] == 2 {1} else {0};
        if i >= l {
            c = (input[i - l] - 'a' as u8) as usize;
            chars[c] -= 1;
            dups -= if chars[c] == 1 { 1 } else { 0 };
        }
        if i >= l - 1 && dups == 0 {
            num = i + 1;
            break;
        }
    }
    println!("{}", num);
}

pub fn solve_a() {
    solve(4);
}

pub fn solve_b() {
    solve(14);
}