use std::cmp::max;
use std::collections::{HashMap, HashSet, VecDeque};
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

const F: usize = 0; // front
const R: usize = 1; // right
const A: usize = 2; // back
const L: usize = 3; // left
const B: usize = 4; // bottom
const T: usize = 5; // top

// face, dir (right,down,left,up) -> next face, ccw rots to match next face
const FACE_TRANS: [[(usize, usize);4];6] = [
    [(R,0),(B,0),(L,0),(T,0)], // F
    [(A,0),(B,1),(F,0),(T,3)], // R
    [(L,0),(B,2),(R,0),(T,2)], // A
    [(F,0),(B,3),(A,0),(T,1)], // L
    [(R,3),(A,2),(L,1),(F,0)], // B
    [(R,1),(F,0),(L,3),(A,2)]  // T
];

pub fn solve_b() {
    let input = fileio::input("src/day22.txt");
    // find w
    let mut tile_count = 0;
    for l in 0..(input.len()-2) {
        let mut i = 0;
        for c in input[l].chars() {
            if c != ' ' {
                break;
            }
            i += 1;
        }
        tile_count += input[l].len() - i;
    }
    tile_count /= 6;
    let mut w = 0;
    while w * w < tile_count {
        w += 1;
    }

    // map input to faces
    // pos (row, col), face, ccw rots from input
    let mut face_map: [((usize,usize),usize);6] = [((0,0),5);6]; // face -> (pos in input space, ccw rots from input orientation)
    let mut faces: Vec<Vec<Vec<bool>>> = vec![vec![vec![false;w];w];6]; // face -> grid on face (true = wall)
    let mut q: VecDeque<((usize,usize),usize,usize)> = VecDeque::new();
    q.push_back(((0,{
        let mut i: usize = 0;
        let line = input[0].as_bytes();
        while line[i * w] == ' ' as u8 {
            i += 1;
        }
        i
    }),0,0));
    while !q.is_empty() {
        let ((r,c),face,rots) = q.pop_front().unwrap();
        face_map[face] = ((r,c),rots);
        let offset = (r*w, c*w);
        match rots {
            0 => {
                for i in 0..w {
                    let line = input[offset.0+i].as_bytes();
                    for j in 0..w {
                        faces[face][i][j] = line[offset.1+j] == '#' as u8;
                    }
                }
            }
            1 => {
                for i in 0..w {
                    let line = input[offset.0+i].as_bytes();
                    for j in 0..w {
                        faces[face][j][w-1-i] = line[offset.1+j] == '#' as u8;
                    }
                }
            }
            2 => {
                for i in 0..w {
                    let line = input[offset.0+i].as_bytes();
                    for j in 0..w {
                        faces[face][w-1-i][w-1-j] = line[offset.1+j] == '#' as u8;
                    }
                }
            }
            3 => {
                for i in 0..w {
                    let line = input[offset.0+i].as_bytes();
                    for j in 0..w {
                        faces[face][w-1-j][i] = line[offset.1+j] == '#' as u8;
                    }
                }
            }
            _ => {panic!();}
        }
        let mut add_face = |r: usize, c: usize, dir: usize| {
            if r*w >= input.len() || c*w >= input[r*w].len() || input[r*w].as_bytes()[c*w] == ' ' as u8 {
                return;
            }
            let rel_dir = (rots+dir) & 3;
            let next_face = FACE_TRANS[face][rel_dir].0;
            if face_map[next_face].1 != 5 {
                return;
            }
            q.push_back(((r,c),next_face,(rots + FACE_TRANS[face][rel_dir].1) & 3));
        };
        add_face(r,c+1,0);
        add_face(r+1,c,1);
        if c != 0 {
            add_face(r,c-1,2);
        }
        if r != 0 {
            add_face(r-1,c,3);
        }
    }

    // traverse
    let mut pos: (usize, usize) = (0,0);
    let mut face: usize = 0;
    let mut dir: usize = 0;
    let mut instructs: Vec<(bool,usize)> = Vec::new(); // (type (rotation=true, movement=false), amount (ccw rots for rotation, steps for movement)
    {
        let mut v = 0;
        for c in input[input.len() - 1].chars() {
            if c == 'R' || c == 'L' {
                if v != 0 {
                    instructs.push((false,v));
                    v = 0;
                }
                instructs.push((true, if c == 'R' {1} else {3}));
            } else {
                v = v * 10 + (c as u8 - '0' as u8) as usize;
            }
        }
        if v != 0 {
            instructs.push((false,v));
        }
    }
    for instr in &instructs {
        if instr.0 {
            dir = (dir + instr.1) & 3;
            continue;
        }
        for _ in 0..instr.1 {
            let (mut r,mut c) = pos;
            let mut d = dir;
            let mut f = face;
            let mut trans = false;
            match dir {
                0 => { // move right
                    if c == w-1 {
                        c = 0;
                        trans = true;
                    } else {
                        c += 1;
                    }
                }
                1 => { // move down
                    if r == w-1 {
                        r = 0;
                        trans = true;
                    } else {
                        r += 1;
                    }
                }
                2 => { // move left
                    if c == 0 {
                        c = w-1;
                        trans = true;
                    } else {
                        c -= 1;
                    }
                }
                3 => { // move up
                    if r == 0 {
                        r = w-1;
                        trans = true;
                    } else {
                        r -= 1;
                    }
                }
                _ => {panic!();}
            }
            if trans {
                f = FACE_TRANS[face][dir].0;
                d = (dir + FACE_TRANS[face][dir].1) & 3;
                for _ in 0..FACE_TRANS[face][dir].1 {
                    (r,c ) = (c,w-1-r);
                }
            }
            if !faces[f][r][c] {
                pos = (r,c);
                dir = d;
                face = f;
            }
        }
    }
    for _ in 0..face_map[face].1 {
        pos = (w-1-pos.1,pos.0);
    }
    dir = (dir + 4 - face_map[face].1) & 3;
    println!("{}", (face_map[face].0.0 * w + pos.0 + 1) * 1000 + (face_map[face].0.1 * w + pos.1 + 1) * 4 + dir);
    // = 4578: row=3, col=143, dir=2
}