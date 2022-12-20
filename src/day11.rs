use std::collections::VecDeque;
use crate::fileio;

mod monkey {
    use std::collections::VecDeque;

    pub struct Monkey {
        items: VecDeque<u64>,
        op_val: u64,
        op_mul: bool,
        test_div: u64,
        pass: (usize, usize),
    }

    impl Monkey {
        pub fn new(starting_items: VecDeque<u64>, op: (u64, bool), test_div: u64, pass: (usize, usize)) -> Monkey {
            Monkey {items: starting_items, op_val: op.0, op_mul: op.1, test_div, pass}
        }

        pub fn items(&mut self) -> &VecDeque<u64> {
            &self.items
        }

        pub fn items_mut(&mut self) -> &mut VecDeque<u64> {
            &mut self.items
        }

        pub fn do_op(&self, val: u64) -> u64 {
            let op_val = if self.op_val == 0 {val} else {self.op_val};
            if self.op_mul {val * op_val} else {val + op_val}
        }

        pub fn pass_target(&self, val: u64) -> usize {
            if val % self.test_div == 0 {self.pass.0} else {self.pass.1}
        }

        pub fn test_div(&self) -> u64 {
            self.test_div
        }
    }
}

use monkey::*;

pub fn make_monkeys(input: &Vec<String>) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = Vec::new();
    for i in (0..input.len()).step_by(7) {
        let lines = &input[i..i+6];
        monkeys.push(Monkey::new({
            let mut starting_items: VecDeque<u64> = VecDeque::new();
            let si_parts: Vec<&str> = lines[1][18..].split(", ").collect();
            for si in si_parts {
                starting_items.push_back(si.parse::<u64>().unwrap());
            }
            starting_items
        }, {
            let op_parts: Vec<&str> = lines[2][19..].split(" ").collect();
            (if op_parts[2] == "old" {0} else {op_parts[2].parse::<u64>().unwrap()}, op_parts[1] == "*")
        }, lines[3][21..].parse::<u64>().unwrap(),
            (lines[4][29..].parse::<usize>().unwrap(), lines[5][30..].parse::<usize>().unwrap())));
    }
    return monkeys;
}

pub fn solve_a() {
    let input = fileio::input("src/day11.txt");
    let mut monkeys = make_monkeys(&input);
    let mut inspects: Vec<usize> = vec!(0; monkeys.len());
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let len = monkeys[i].items().len();
            inspects[i] += len;
            for _ in 0..len {
                let worry = {
                    let w = monkeys[i].items_mut().pop_front().unwrap();
                    monkeys[i].do_op(w) / 3
                };
                let pass_target = monkeys[i].pass_target(worry);
                monkeys[pass_target].items_mut().push_back(worry);
            }
        }
    }
    inspects.sort_unstable();
    inspects.reverse();
    println!("{}", inspects[0] * inspects[1]);
}

pub fn solve_b() {
    let input = fileio::input("src/day11.txt");
    let mut monkeys = make_monkeys(&input);
    let div_val = {
        let mut v = 1;
        for m in &monkeys {
            v *= m.test_div();
        }
        v
    };
    let mut inspects: Vec<usize> = vec!(0; monkeys.len());
    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            let len = monkeys[i].items().len();
            inspects[i] += len;
            for _ in 0..len {
                let worry = {
                    let w = monkeys[i].items_mut().pop_front().unwrap();
                    monkeys[i].do_op(w) % div_val
                };
                let pass_target = monkeys[i].pass_target(worry);
                monkeys[pass_target].items_mut().push_back(worry);
            }
        }
    }
    inspects.sort_unstable();
    inspects.reverse();
    println!("{}", inspects[0] * inspects[1]);
}