use crate::fileio;

struct Board {
    sum: u32,
    grid: [[u8;5];5],
    marked: Vec<(usize,usize)>,
    rows: [u8;5],
    cols: [u8;5],
    bingo: bool,
}

impl Board {
    pub fn new() -> Self {
        return Board {
            sum: 0,
            grid: [[0;5];5],
            marked: Vec::new(),
            rows: [0;5],
            cols: [0;5],
            bingo: false,
        }
    }

    fn mark(&mut self, r: usize, c: usize) {
        self.rows[r] += 1;
        self.cols[c] += 1;
        self.marked.push((r,c));
        self.bingo = self.rows[r] == 5 || self.cols[c] == 5;
    }

    pub fn call(&mut self, val: u8) {
        for r in 0..5 {
            for c in 0..5 {
                if self.grid[r][c] == val {
                    self.mark(r,c);
                    return;
                }
            }
        }
        return;
    }
}

fn create_boards(input: &Vec<String>) -> Vec<Board> {
    let mut boards: Vec<Board> = Vec::new();
    for l in (2..input.len()).step_by(6) {
        boards.push(Board::new());
        let t = boards.len()-1;
        let mut board = &mut boards[t];
        for r in 0..5 {
            let line = input[l+r].as_bytes();
            for c in 0..5 {
                let i = c*3;
                board.grid[r][c] = if line[i] == ' ' as u8 {line[i+1] - '0' as u8} else {(line[i] - '0' as u8) * 10 + line[i+1] - '0' as u8};
                board.sum += board.grid[r][c] as u32;
            }
        }
    }
    return boards;
}

pub fn solve_a() {
    let input = fileio::input("src/year2021/input/day4.txt");
    let mut boards = create_boards(&input);
    let calls = input[0].split(",");
    let mut score: u32 = 0;
    for c in calls {
        let call_val = c.parse::<u8>().unwrap();
        for b in &mut boards {
            b.call(call_val);
            if b.bingo {
                score = b.sum;
                for m in &b.marked {
                    score -= b.grid[m.0][m.1] as u32;
                }
                score *= call_val as u32;
                break;
            }
        }
        if score != 0 {
            break;
        }
    }
    println!("{}", score);
}

pub fn solve_b() {
    let input = fileio::input("src/year2021/input/day4.txt");
    let mut boards = create_boards(&input);
    let calls = input[0].split(",");
    let mut score: u32 = 0;
    let mut unbingoed = boards.len();
    for c in calls {
        let call_val = c.parse::<u8>().unwrap();
        for b in &mut boards {
            if b.bingo {
                continue;
            }
            b.call(call_val);
            if !b.bingo {
                continue;
            }
            unbingoed -= 1;
            if unbingoed != 0 {
                continue;
            }
            score = b.sum;
            for m in &b.marked {
                score -= b.grid[m.0][m.1] as u32;
            }
            score *= call_val as u32;
            break;
        }
        if unbingoed == 0 {
            break;
        }
    }
    println!("{}", score);
}