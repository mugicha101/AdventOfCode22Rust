use crate::fileio;

fn read_input() -> (Vec<String>, usize, Vec<Vec<u8>>) {
    let mut input = fileio::input("src/day5.txt");
    let mut i = 0;
    while i < input.len() && input[i].len() != 0 {
        i += 1;
    }
    let j = i-1;
    let mut stacks: Vec<Vec<u8>> = Vec::new();
    {
        let mut prev_blank = true;
        for c in input[j].bytes() {
            let blank = c == ' ' as u8;
            if blank != prev_blank {
                if (!blank) {
                    stacks.push(Vec::new());
                }
                prev_blank = blank;
            }
        }
    }
    i -= 1;
    while i > 0 {
        i -= 1;
        let ln = &input[i].as_bytes();
        let mut si = 0;
        for li in (1..ln.len()).step_by(4) {
            if ln[li] != ' ' as u8 {
                stacks[si].push(ln[li]);
            }
            si += 1;
        }
    }
    return (input, j+2, stacks);
}

pub fn solve_a() {
    let (input, start_index, mut stacks) = read_input();
    for i in start_index..input.len() {
        let ln = input[i].as_bytes();
        let (mut amount, mut source, mut dest) = (0 as u8, 0 as usize, 0 as usize);
        let mut li = 0;
        let mut state: u8 = 0;
        while li < ln.len() {
            if state == 0 {
                if ln[li] == 'm' as u8 {
                    li += 5;
                    state = 1;
                } else if ln[li] == 'f' as u8 {
                    li += 5;
                    state = 2;
                } else if ln[li] == 't' as u8 {
                    li += 3;
                    state = 3;
                }
            } else if ln[li] == ' ' as u8 {
                state = 0;
                li += 1
            } else {
                let val = ln[li] - '0' as u8;
                if state == 1 {
                    amount = amount * 10 + val;
                } else if state == 2 {
                    source = source * 10 + val as usize;
                } else {
                    dest = dest * 10 + val as usize;
                }
                li += 1
            }
        }
        source -= 1;
        dest -= 1;
        for a in 0..amount {
            let v = stacks[source].pop().unwrap();
            stacks[dest].push(v);
        }
    }
    for mut stack in stacks {
        print!("{}", if stack.len() == 0 {' '} else {stack.pop().unwrap() as char});
    }
    println!();
}

pub fn solve_b() {
    let (input, start_index, mut stacks) = read_input();
    for i in start_index..input.len() {
        let ln = input[i].as_bytes();
        let (mut amount, mut source, mut dest) = (0 as u8, 0 as usize, 0 as usize);
        let mut li = 0;
        let mut state: u8 = 0;
        while li < ln.len() {
            if state == 0 {
                if ln[li] == 'm' as u8 {
                    li += 5;
                    state = 1;
                } else if ln[li] == 'f' as u8 {
                    li += 5;
                    state = 2;
                } else if ln[li] == 't' as u8 {
                    li += 3;
                    state = 3;
                }
            } else if ln[li] == ' ' as u8 {
                state = 0;
                li += 1
            } else {
                let val = ln[li] - '0' as u8;
                if state == 1 {
                    amount = amount * 10 + val;
                } else if state == 2 {
                    source = source * 10 + val as usize;
                } else {
                    dest = dest * 10 + val as usize;
                }
                li += 1
            }
        }
        source -= 1;
        dest -= 1;
        let src_len = stacks[source].len();
        for a in (1..=amount).rev() {
            let v = stacks[source][src_len - a as usize];
            stacks[dest].push(v);
        }
        stacks[source].truncate(src_len - amount as usize);
    }
    for mut stack in stacks {
        print!("{}", if stack.len() == 0 {' '} else {stack.pop().unwrap() as char});
    }
    println!();
}