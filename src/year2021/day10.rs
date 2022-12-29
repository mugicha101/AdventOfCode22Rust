use crate::fileio;

pub fn solve_a() {
    let input = fileio::input("src/year2021/input/day10.txt");
    let mut score: u32 = 0;
    for ln in input {
        let mut stack: Vec<char> = Vec::new();
        for c in ln.chars() {
            if c == '(' || c == '[' || c == '{' || c == '<' {
                stack.push(c);
                continue;
            }
            if c == ')' {
                if stack.len() == 0 || stack.pop().unwrap() != '(' {
                    score += 3;
                    break;
                }
            } else if c == ']' {
                if stack.len() == 0 || stack.pop().unwrap() != '[' {
                    score += 57;
                    break;
                }
            } else if c == '}' {
                if stack.len() == 0 || stack.pop().unwrap() != '{' {
                    score += 1197;
                    break;
                }
            } else if c == '>' {
                if stack.len() == 0 || stack.pop().unwrap() != '<' {
                    score += 25137;
                    break;
                }
            } else {
                panic!();
            }
        }
    }
    println!("{}", score);
}

pub fn solve_b() {
    let input = fileio::input("src/year2021/input/day10.txt");
    let mut scores: Vec<u64> = Vec::new();
    for ln in input {
        let mut stack: Vec<char> = Vec::new();
        let mut corrupt = false;
        for c in ln.chars() {
            if c == '(' || c == '[' || c == '{' || c == '<' {
                stack.push(c);
                continue;
            }
            if stack.len() == 0 {
                corrupt = true;
                break;
            }
            let v = stack.pop().unwrap();
            if (c == ')' && v != '(')
                || (c == ']' && v != '[')
                || (c == '}' && v != '{')
                || (c == '>' && v != '<') {
                corrupt = true;
                break;
            }
        }
        if corrupt || stack.len() == 0 {
            continue;
        }
        let mut score: u64 = 0;
        while stack.len() != 0 {
            score = score * 5 + match stack.pop().unwrap() {
                '(' => {1},
                '[' => {2},
                '{' => {3},
                '<' => {4},
                _ => {panic!();}
            }
        }
        scores.push(score);
    }
    scores.sort();
    println!("{}", scores[scores.len() >> 1]);
}