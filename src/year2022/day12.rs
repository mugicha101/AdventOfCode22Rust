use std::cmp::min;
use std::collections::VecDeque;
use crate::fileio;

fn parse(input: Vec<String>) -> (Vec<Vec<u8>>, (usize, usize), (usize, usize), Vec<(usize, usize)>) {
    let height = input.len();
    let width = input[0].len();
    let mut h_map: Vec<Vec<u8>> = vec![vec![0; width]; height];
    let mut start: (usize, usize) = (0, 0);
    let mut end: (usize, usize) = (0, 0);
    let mut lowest: Vec<(usize, usize)> = Vec::new();
    for h in 0..height {
        let row = input[h].as_bytes();
        for w in 0..width {
            let mut tile = &mut h_map[h][w];
            if row[w] == 'S' as u8 {
                *tile = 0;
                start = (w, h);
                lowest.push((w, h));
            } else if row[w] == 'E' as u8 {
                *tile = 25;
                end = (w, h);
            } else {
                *tile = row[w] - 'a' as u8;
                if row[w] == 'a' as u8 {
                    lowest.push((w, h));
                }
            }
        }
    }
    (h_map, start, end, lowest)
}

fn shortest_path(width: usize, height: usize, h_map: &Vec<Vec<u8>>, start: (usize, usize), end: (usize, usize)) -> u32 {
    let mut min_dists: Vec<Vec<u32>> = vec![vec![u32::MAX; width]; height];
    min_dists[start.1][start.0] = 0;
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    queue.push_back(start);
    while queue.len() != 0 {
        let pos = queue.pop_front().unwrap();
        let h = h_map[pos.1][pos.0];
        let md = min_dists[pos.1][pos.0];
        let mut visit = |row: usize, col: usize| {
            let n_h = h_map[col][row];
            if n_h > h + 1 {
                return;
            }
            let n_md = &mut min_dists[col][row];
            if *n_md > md + 1 {
                *n_md = md + 1;
                queue.push_back((row, col));
            }
        };
        if pos.0 != 0 {
            visit(pos.0 - 1, pos.1);
        }
        if pos.0 != width - 1 {
            visit(pos.0 + 1, pos.1);
        }
        if pos.1 != 0 {
            visit(pos.0, pos.1 - 1);
        }
        if pos.1 != height - 1 {
            visit(pos.0, pos.1 + 1);
        }
    }
    return min_dists[end.1][end.0];
}

pub fn solve_a() {
    let input = fileio::input("src/year2022/input/day12.txt");
    let height = input.len();
    let width = input[0].len();
    let (h_map, start, end, _) = parse(input);
    println!("{}", shortest_path(width, height, &h_map, start, end));
}

pub fn solve_b() {
    let input = fileio::input("src/year2022/input/day12.txt");
    let height = input.len();
    let width = input[0].len();
    let (h_map, _, end, lowest) = parse(input);
    let mut md = u32::MAX;
    for start in &lowest {
        md = min(md, shortest_path(width, height, &h_map, *start, end));
    }
    println!("{}", md);
}