use std::cmp::max;
use crate::fileio;

pub fn solve_a() {
    let input = fileio::input("src/year2022/input/day8.txt");
    let (rows, cols) = (input.len(), input[0].len());
    let mut heights: Vec<Vec<u8>> = vec![vec![0; cols]; rows];
    let mut visible: Vec<Vec<bool>> = vec![vec![false; cols]; rows];
    for r in 0..rows {
        let ln = input[r].as_bytes();
        for c in 0..cols {
            heights[r][c] = 1 + ln[c] - '0' as u8;
        }
    }
    for r in 0..rows {
        // from left
        let mut max_h = 0;
        for c in 0..cols {
            if heights[r][c] > max_h {
                max_h = heights[r][c];
                visible[r][c] = true;
                if max_h == 10 { break }
            }
        }

        // from right
        max_h = 0;
        for c in (0..cols).rev() {
            if heights[r][c] > max_h {
                max_h = heights[r][c];
                visible[r][c] = true;
                if max_h == 10 { break }
            }
        }
    }
    for c in 0..cols {
        // from top
        let mut max_h = 0;
        for r in 0..rows {
            if heights[r][c] > max_h {
                max_h = heights[r][c];
                visible[r][c] = true;
                if max_h == 10 { break }
            }
        }

        // from bottom
        max_h = 0;
        for r in (0..rows).rev() {
            if heights[r][c] > max_h {
                max_h = heights[r][c];
                visible[r][c] = true;
                if max_h == 10 { break }
            }
        }
    }

    // count seen trees
    let mut total_visible = 0;
    for r in 0..rows {
        let ln = input[r].as_bytes();
        for c in 0..cols {
            total_visible += if visible[r][c] {1} else {0};
        }
    }
    println!("{}", total_visible);
}

pub fn solve_b() {
    let input = fileio::input("src/year2022/input/day8.txt");
    let (rows, cols) = (input.len(), input[0].len());
    let mut heights: Vec<Vec<u8>> = vec![vec![0; cols]; rows];
    let mut max_score = 0;
    for r in 0..rows {
        let ln = input[r].as_bytes();
        for c in 0..cols {
            heights[r][c] = ln[c] - '0' as u8;
        }
    }
    for r in 1..rows-1 {
        let ln = input[r].as_bytes();
        for c in 1..cols-1 {
            let h = heights[r][c];
            let mut score = 1;

            // look left
            if c != cols-1 {
                let mut v = c + 1;
                while v < cols - 1 && heights[r][v] < h { v += 1 }
                score *= v - c;
            }

            // look right
            if c != 0 {
                let mut v = c - 1;
                while v > 0 && heights[r][v] < h { v -= 1 }
                score *= c - v;
            }

            // look down
            if r != rows-1 {
                let mut v = r + 1;
                while v < rows - 1 && heights[v][c] < h { v += 1 }
                score *= v - r;
            }

            // look up
            if r != 0 {
                let mut v = r - 1;
                while v > 0 && heights[v][c] < h { v -= 1 }
                score *= r - v;
            }

            max_score = max(max_score, score);
        }
    }
    println!("{}", max_score);
}