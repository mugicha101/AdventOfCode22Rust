use std::cmp::{max, min};
use std::str::from_utf8;
use crate::fileio;

fn create_grid(input: &Vec<String>, floor: bool) -> (Vec<Vec<bool>>, (usize, usize), (usize, usize))  {
    let mut rock_lines: Vec<Vec<(i32, i32)>> = Vec::new();
    for ln in input {
        let parts: Vec<&str> = ln.split(" -> ").collect();
        let mut rl: Vec<(i32, i32)> = Vec::new();
        for p in parts {
            let d = p.find(",").unwrap();
            let s = p.as_bytes();
            rl.push((
                from_utf8(&s[0..d]).unwrap().parse::<i32>().unwrap(),
                from_utf8(&s[d+1..s.len()]).unwrap().parse::<i32>().unwrap()
            ));
        }
        rock_lines.push(rl);
    }
    let mut limits: [i32; 4] = [500,0,500,0];
    for rl in &rock_lines {
        for p in rl {
            limits[0] = min(limits[0], p.0);
            limits[1] = min(limits[1], p.1);
            limits[2] = max(limits[2], p.0);
            limits[3] = max(limits[3], p.1);
        }
    }
    if floor {
        limits[3] += 1;
        limits[0] = min(limits[0], 500 - limits[3]);
        limits[2] = max(limits[2], 500 + limits[3]);
    }
    let offset = (limits[0] as usize, limits[1] as usize);
    let dim = ((limits[2] - limits[0] + 1) as usize, (limits[3] - limits[1] + 1) as usize);
    let mut grid: Vec<Vec<bool>> = vec![vec![false; dim.0]; dim.1];
    for rl in &rock_lines {
        for i in 0..rl.len()-1 {
            let apply_offset = |pos: &(i32, i32)| -> (i32, i32) {
                (pos.0 - offset.0 as i32, pos.1 - offset.1 as i32)
            };
            let mut pos = apply_offset(&rl[i]);
            let mut end = apply_offset(&rl[i+1]);
            let o = ((end.0 as i32 - pos.0 as i32).signum(), (end.1 as i32 - pos.1 as i32).signum());
            while pos.0 != end.0 || pos.1 != end.1 {
                grid[pos.1 as usize][pos.0 as usize] = true;
                pos.0 += o.0;
                pos.1 += o.1;
            }
            grid[pos.1 as usize][pos.0 as usize] = true;
        }
    }
    return (grid, offset, dim);
}

pub fn solve_a() {
    let input = fileio::input("src/year2022/input/day14.txt");
    let (mut grid, offset, dim) = create_grid(&input, false);
    let spawn_pos = (500 - offset.0, 0 - offset.1);
    let mut sand_count = 0;
    let mut finished = false;
    let mut path: Vec<(usize, usize)> = Vec::new();
    path.push(spawn_pos);
    while !finished {
        let mut sand_pos = path[path.len()-1];
        while grid[sand_pos.1][sand_pos.0] {
            path.pop();
            sand_pos = path[path.len()-1];
        }
        while sand_pos.1 != dim.1 - 1 {
            if grid[sand_pos.1 + 1][sand_pos.0] {
                if sand_pos.0 == 0 {
                    finished = true;
                    break;
                } else if !grid[sand_pos.1 + 1][sand_pos.0 - 1] {
                    sand_pos.0 -= 1;
                } else if sand_pos.0 == dim.0 - 1 {
                    finished = true;
                    break;
                }
                else if !grid[sand_pos.1 + 1][sand_pos.0 + 1] {
                    sand_pos.0 += 1;
                } else {
                    break;
                }
            }
            sand_pos.1 += 1;
            path.push(sand_pos);
        }
        if finished || sand_pos.1 == dim.1 - 1 {
            finished = true;
        } else {
            grid[sand_pos.1][sand_pos.0] = true;
            sand_count += 1;
        }
    }
    println!("{}", sand_count);
}

pub fn solve_b() {
    let input = fileio::input("src/year2022/input/day14.txt");
    let (mut grid, offset, dim) = create_grid(&input, true);
    // let origin_grid = grid.clone();
    let spawn_pos = (500 - offset.0, 0 - offset.1);
    let mut sand_count = 0;
    let mut path: Vec<(usize, usize)> = Vec::new();
    path.push(spawn_pos);
    while !grid[spawn_pos.1][spawn_pos.0] {
        let mut sand_pos = path[path.len()-1];
        while grid[sand_pos.1][sand_pos.0] {
            path.pop();
            sand_pos = path[path.len()-1];
        }
        while sand_pos.1 != dim.1 - 1 {
            if grid[sand_pos.1 + 1][sand_pos.0] {
                if sand_pos.0 == 0 {
                    break;
                } else if !grid[sand_pos.1 + 1][sand_pos.0 - 1] {
                    sand_pos.0 -= 1;
                } else if sand_pos.0 == dim.0 - 1 {
                    break;
                }
                else if !grid[sand_pos.1 + 1][sand_pos.0 + 1] {
                    sand_pos.0 += 1;
                } else {
                    break;
                }
            }
            sand_pos.1 += 1;
            path.push(sand_pos);
        }
        grid[sand_pos.1][sand_pos.0] = true;
        sand_count += 1;
    }
    /*
    for r in 0..dim.1 {
        for c in 0..dim.0 {
            print!("{}", match (origin_grid[r][c], grid[r][c]) {
                (true, _) => {
                    "#"
                }
                (false, true) => {
                    "S"
                }
                (false, false) => {
                    "."
                }
            });
        }
        println!();
    }
     */
    println!("{}", sand_count);
}