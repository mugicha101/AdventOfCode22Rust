use std::cmp::max;
use std::collections::{HashMap, HashSet};
use crate::fileio;

fn parse_board(input: &Vec<String>) -> (Vec<usize>, Vec<Vec<bool>>, &[u8]) {
    let mut left_margin: Vec<usize> = Vec::new();
    let mut board: Vec<Vec<bool>> = vec![Vec::new(); input.len()-2];
    for row in 0..input.len()-2 {
        let line = input[row].as_bytes();
        let mut l = 0;
        while l < line.len() && line[l] == ' ' as u8 {
            l += 1;
        }
        if l == line.len() {
            break;
        }
        left_margin.push(l);
        for c in l..line.len() {
            board[row].push(line[c] == '#' as u8);
        }
    }
    return (left_margin, board, input[input.len()-1].as_bytes());
}

pub fn solve_a() {
    let input = fileio::input("src/day22.txt");
    let (left_margin, board, instructs) = parse_board(&input);
    let mut value: u32 = 0;
    let mut dir: u8 = 0;
    let mut pos: (usize, usize) = (left_margin[0], 0);
    let x_in_bounds = |x: usize, y: usize| -> bool {
        left_margin[y] <= x && left_margin[y] + board[y].len() > x
    };
    // let mut t = 0;
    let mut travel = |amount: u32, dir: u8| {
        for _ in 0..amount {
            let mut dest = (pos.0, pos.1);
            match dir {
                0 => { // move right
                    dest.0 = if pos.0 + 1 == left_margin[pos.1] + board[pos.1].len() {
                        left_margin[pos.1]
                    } else {
                        pos.0 + 1
                    };
                }
                1 => { // move down
                    if pos.1 + 1 == board.len() || !x_in_bounds(pos.0, pos.1+1) {
                        dest.1 -= 1;
                        while dest.1 != 0 && x_in_bounds(pos.0, dest.1) {
                            dest.1 -= 1;
                        }
                        if dest.1 != 0 || !x_in_bounds(pos.0, dest.1) {
                            dest.1 += 1;
                        }
                    } else {
                        dest.1 += 1;
                    }
                }
                2 => { // move left
                    dest.0 = if pos.0 == left_margin[pos.1] {
                        left_margin[pos.1] + board[pos.1].len() - 1
                    } else {
                        pos.0 - 1
                    };
                }
                3 => { // move up
                    if pos.1 == 0 || !x_in_bounds(pos.0, pos.1-1) {
                        dest.1 += 1;
                        while dest.1 != board.len()-1 && x_in_bounds(pos.0, dest.1) {
                            dest.1 += 1;
                        }
                        if dest.1 != board.len()-1 || !x_in_bounds(pos.0, dest.1) {
                            dest.1 -= 1;
                        }
                    } else {
                        dest.1 -= 1;
                    }
                }
                _ => {
                    panic!("invalid instruction");
                }
            };
            if board[dest.1][dest.0 - left_margin[dest.1]] {
                break;
            }
            pos = dest;
        }
    };
    for instr in instructs {
        if *instr == 'R' as u8 {
            travel(value, dir);
            value = 0;
            dir = (dir + 1) & 3;
        } else if *instr == 'L' as u8 {
            travel(value, dir);
            value = 0;
            dir = (dir + 3) & 3;
        } else {
            value = value * 10 + (*instr - '0' as u8) as u32;
        }
    }
    travel(value, dir);
    println!("{}", (pos.1 + 1) * 1000 + (pos.0 + 1) * 4 + dir as usize);
}

pub fn solve_b() {
    let input = fileio::input("src/day22.txt");
    let check = fileio::input("src/check.txt");
    let (left_margin, board, instructs) = parse_board(&input);
    let mut value: u32 = 0;
    let mut dir: u8 = 0;
    let mut pos: (i32, i32) = (left_margin[0] as i32, 0);
    let mut t = 0;
    // hard code warps
    let mut warps: HashMap<(i32, i32), (i32, i32, u8)> = HashMap::new(); // (row, col) -> (row, col, dir change)
    for i in 0..50 {
        // top of F -> bottom of T
        warps.insert((-1,50+i), (150+i,0,1));
        warps.insert((150+i,-1), (0,50+i,3));

        // top of R -> right of T
        warps.insert((-1,i+100), (199,i,0));
        warps.insert((200,i), (0,i+100,0));

        // right of R -> left of A
        warps.insert((i,150), (149-i,99,2));
        warps.insert((149-i,100), (i,149,2));

        // bottom of R -> right of B
        warps.insert((50,100+i), (50+i,99,1));
        warps.insert((50+i,100), (49,100+i,3));

        // top of A -> top of T
        warps.insert((150,50+i), (150+i,49,1));
        warps.insert((150+i,50), (149,50+i,3));

        // right of L -> left of F
        warps.insert((100+i,-1), (49-i,50,2));
        warps.insert((49-i,49), (100+i,0,2));

        // bottom of L -> left of B
        warps.insert((99,i), (50+i,50,1));
        warps.insert((50+i,49), (100,i,3));
    }
    let mut travel = |amount: u32, mut dir: u8| {
        for _ in 0..amount {
            let mut new_dir = dir;
            let mut dest = (pos.0, pos.1);
            match dir {
                0 => {
                    dest.0 += 1;
                }
                1 => {
                    dest.1 += 1;
                }
                2 => {
                    dest.0 -= 1;
                }
                3 => {
                    dest.1 -= 1;
                }
                _ => {
                    panic!("invalid direction");
                }
            }
            if dest.1 < 0 || dest.1 >= board.len() as i32 || dest.0 < left_margin[dest.1 as usize] as i32 || dest.0 >= (left_margin[dest.1 as usize] + board[dest.1 as usize].len()) as i32 {
                let mut w = warps[&(dest.1, dest.0)];
                if dest == (49,99) {
                    w = if pos.1 == 100 {(99,50,0)} else {(100,49,0)};
                    new_dir = if pos.1 == 100 {0} else {1};
                }
                if dest == (100,100) {
                    w = if pos.1 == 99 {(99,100,0)} else {(100,99,0)};
                    new_dir = if pos.1 == 99 {2} else {3};
                }
                // one more corner case but idk
                // println!("({},{},{}) out", dest.0, dest.1, dir);
                dest = (w.1, w.0);
                assert!(!(dest.1 < 0 || dest.1 >= board.len() as i32 || dest.0 < left_margin[dest.1 as usize] as i32 || dest.0 >= (left_margin[dest.1 as usize] + board[dest.1 as usize].len()) as i32));
                new_dir = (new_dir + w.2) & 3;
            }
            if board[dest.1 as usize][dest.0 as usize - left_margin[dest.1 as usize]] {
                break;
            }
            dir = new_dir;
            pos = (dest.0, dest.1);
            // println!("({},{},{})", pos.0, pos.1, dir);
            /*
            let cl: Vec<&str> = check[t].split(" ").collect();
            let c = (cl[1].parse::<i32>().unwrap(), cl[0].parse::<i32>().unwrap());
            if c.0 != pos.0 || c.1 != pos.1 {
                println!("t:{} ({},{}) != ({},{})", t, pos.0, pos.1, c.0, c.1);
                panic!("check doesnt match");
            }
            t += 1;

             */
        }
        return dir;
    };
    for instr in instructs {
        if *instr == 'R' as u8 {
            dir = travel(value, dir);
            value = 0;
            dir = (dir + 1) & 3;
        } else if *instr == 'L' as u8 {
            dir = travel(value, dir);
            value = 0;
            dir = (dir + 3) & 3;
        } else {
            value = value * 10 + (*instr - '0' as u8) as u32;
        }
    }
    travel(value, dir);
    println!("{}", (pos.1 as i32 + 1) * 1000 + (pos.0 as i32 + 1) * 4 + dir as i32);
}