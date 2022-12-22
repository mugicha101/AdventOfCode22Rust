use std::collections::VecDeque;
use crate::fileio;
use grid_state::*;

mod grid_state {
    use std::collections::VecDeque;
    use crate::day17::grid_state::Push::*;
    pub const WIDTH: usize = 7;

    enum Push {
        Left,
        Right,
        Down,
    }

    pub struct GridState {
        grid: VecDeque<u8>,
        drop_count: u64,
        // lines below row 0 of grid
        p_i: usize,
        // piece index
        w_i: usize, // input wind index
        input: Vec<u8>,
        pieces: [Vec<(i32, i32)>; 5],
        base: usize,
    }

    impl PartialEq for GridState {
        fn eq(&self, other: &Self) -> bool {
            return self.w_i == other.w_i && self.p_i == other.p_i && self.grid == other.grid;
        }
    }

    impl GridState {
        pub fn new(input: &[u8]) -> GridState {
            GridState {
                grid: VecDeque::from(vec![127, 0]),
                drop_count: 0,
                p_i: 0,
                w_i: 0,
                input: Vec::from(input),
                pieces: [
                    vec![(0,0), (1,0), (2,0), (3,0)],
                    vec![(1,0), (0,1), (1,1), (2,1), (1,2)],
                    vec![(0,0), (1,0), (2,0), (2,1), (2,2)],
                    vec![(0,0), (0,1), (0,2), (0,3)],
                    vec![(0,0), (1,0), (0,1), (1,1)]
                ],
                base: 0,
            }
        }

        pub fn drop_count(&self) -> u64 {
            return self.drop_count;
        }

        pub fn get_max_y(&self) -> usize {
            return self.get_max_y_relative() + self.base;
        }

        fn get_max_y_relative(&self) -> usize {
            let mut y = self.grid.len()-1;
            while self.grid[y] == 0 {
                y -= 1;
            }
            return y;
        }

        #[inline(always)]
        fn grid(&self, x: usize, y: usize) -> bool {
            self.grid[y] & (1 << x) != 0
        }

        #[inline(always)]
        fn set_grid(&mut self, x: usize, y: usize, value: bool) {
            self.grid[y] = (!(1 << x) & self.grid[y]) | (if value {1} else {0} << x);
        }

        pub fn step(&mut self) {
            // get max y
            let max_y = self.get_max_y_relative();

            // spawn piece
            {
                while self.grid.len() < max_y + 9 {
                    self.grid.push_back(0);
                }
            }
            let piece = self.pieces[self.p_i].clone();
            let mut piece_offset = (2, max_y as i32 + 4);
            self.p_i = if self.p_i == self.pieces.len() - 1 {0} else {self.p_i + 1};

            // move pieces
            loop {
                let mut move_piece = |push: Push| -> bool {
                    let offset: (i32, i32) = match push {
                        Left => { (-1,0) }
                        Right => { (1,0) }
                        Down => { (0,-1) }
                    };
                    for p in &piece {
                        let dest = (p.0 + piece_offset.0 + offset.0, p.1+ piece_offset.1 + offset.1);
                        if dest.0 < 0 || dest.1 < 0 || dest.0 >= WIDTH as i32 || (self.grid[dest.1 as usize] & (1 << dest.0 as usize) != 0) {
                            return false;
                        }
                    }
                    piece_offset.0 += offset.0;
                    piece_offset.1 += offset.1;
                    return true;
                };
                let push = if self.input[self.w_i] == '<' as u8 {Left} else {Right};
                self.w_i = if self.w_i == self.input.len() - 1 {0} else {self.w_i + 1};
                move_piece(push);
                if !move_piece(Down) {
                    break;
                }
            }

            // place piece
            for p in &piece {
                let dest = (p.0 + piece_offset.0, p.1 + piece_offset.1);
                self.set_grid(dest.0 as usize, dest.1 as usize, true);
            }

            // cull empty lines from top
            while self.grid[self.grid.len() - 1] == 0 {
                self.grid.pop_back();
            }

            // cull lines from bottom (flood fill with no upwards movement)
            let mut y = self.grid.len()-1;
            let mut open = true;
            let mut last_row: u8 = !self.grid[y];
            while y != 0 && open {
                open = false;
                let mut row: u8 = 0;
                y -= 1;
                for x in 0..WIDTH {
                    if (1 << x) & (row | !last_row | self.grid[y]) != 0 {
                        continue;
                    }
                    open = true;
                    row |= 1 << x;
                    for n_x in (0..x).rev() {
                        row |= ((row & (1 << (n_x + 1))) >> 1) & (!self.grid[y] & (1 << n_x));
                    }
                    for n_x in (x + 1)..WIDTH {
                        row |= ((row & (1 << (n_x - 1))) << 1) & (!self.grid[y] & (1 << n_x));
                    }
                }
                last_row = row;
            }
            self.base += y;
            self.grid.drain(0..y);

            self.drop_count += 1;
        }

        pub fn print_grid(&self) {
            for y in (0..self.grid.len()).rev() {
                for x in 0..WIDTH {
                    print!("{}", if self.grid(x, y) {'#'} else {'.'});
                }
                println!();
            }
        }
    }
}

pub fn solve_a() {
    let input_raw = fileio::input("src/day17.txt");
    let input = input_raw[0].as_bytes();
    let mut g = GridState::new(input);
    const CYCLES: u64 = 2022;
    while g.drop_count() != CYCLES {
        g.step();
    }
    println!("{}", g.get_max_y());
}

pub fn solve_b() {
    let input_raw = fileio::input("src/day17.txt");
    let input = input_raw[0].as_bytes();
    let mut g = GridState::new(input);
    let mut g_slow = GridState::new(input);
    const CYCLES: u64 = 1000000000000;
    g.step();
    while g.drop_count() != CYCLES && g != g_slow {
        g.step();
        g.step();
        g_slow.step();
    }
    if g.drop_count() == CYCLES {
        println!("{}", g.get_max_y());
    }
    let drop_count_diff = g.drop_count() - g_slow.drop_count();
    let y_diff = g.get_max_y() - g_slow.get_max_y();
    let periods = (CYCLES - g.drop_count()) / drop_count_diff;
    let goal = CYCLES - periods * drop_count_diff;
    while g.drop_count() != goal {
        g.step();
    }
    println!("{}", g.get_max_y() as u64 + (y_diff as u64) * periods);
}