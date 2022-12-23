use std::cell::RefCell;
use std::cmp::min;
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;
use priority_queue::PriorityQueue;
use crate::day21::monkey::*;
use crate::fileio;

mod monkey {
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::ops::{Add, Div};
    use std::rc::Rc;

    pub struct Monkey {
        equation: Box<dyn Equation>,
        children: Option<(usize, usize)>,
        value: i64,
        id: u8,
        marked: bool,
    }

    impl Monkey {
        pub fn new() -> Monkey {
            return Monkey {
                equation: Box::new(Addition {}),
                children: None,
                value: 0,
                marked: false,
                id: 0,
            }
        }

        pub fn set_data(&mut self, parts: &Vec<&str>, symbol_table: &HashMap<String, usize>) {
            if parts.len() == 2 {
                self.value = parts[1].parse().unwrap();
                self.id = 0;
            } else if parts[2] == "*" {
                self.equation = Box::new(Multiplication {});
                self.id = 3;
            } else if parts[2] == "/" {
                self.equation = Box::new(Division {});
                self.id = 4;
            } else if parts[2] == "+" {
                self.equation = Box::new(Addition {});
                self.id = 1;
            } else if parts[2] == "-" {
                self.equation = Box::new(Subtraction {});
                self.id = 2;
            }
            if parts.len() != 2 {
                self.children = Some((symbol_table[parts[1]], symbol_table[parts[3]]));
            }
        }

        pub fn set_id(&mut self, id: u8) {
            self.id = id;
        }

        pub fn id(&self) -> u8 {
            return self.id;
        }

        pub fn get_marked_mut(&mut self) -> &mut bool {
            return &mut self.marked;
        }

        pub fn get_value_mut(&mut self) -> &mut i64 {
            return &mut self.value;
        }

        pub fn get_children(&self) -> &Option<(usize, usize)> {
            &self.children
        }

        pub fn get_equation(&self) -> &Box<dyn Equation> {
             &self.equation
        }

        pub fn get_equation_mut(&mut self) -> &mut Box<dyn Equation> {
            &mut self.equation
        }
    }

    pub trait Equation {
        fn run(&self, a: i64, b: i64) -> i64;
    }

    pub struct Multiplication;
    impl Equation for Multiplication {
        fn run(&self, a: i64, b: i64) -> i64 {
            return a * b;
        }
    }

    pub struct Division;
    impl Equation for Division {
        fn run(&self, a: i64, b: i64) -> i64 {
            return a / b;
        }
    }

    pub struct Addition;
    impl Equation for Addition {
        fn run(&self, a: i64, b: i64) -> i64 {
            return a + b;
        }
    }

    pub struct Subtraction;
    impl Equation for Subtraction {
        fn run(&self, a: i64, b: i64) -> i64 {
            return a - b;
        }
    }
}

fn create_tree(input: &Vec<String>) -> (Rc<RefCell<Vec<Monkey>>>, usize, usize) {
    let mut symbol_table: HashMap<String, usize> = HashMap::new();
    let monkeys: Rc<RefCell<Vec<Monkey>>> = Rc::new(RefCell::new(Vec::new()));
    for ln in input {
        let parts: Vec<&str> = ln.splitn(2, " ").collect();
        symbol_table.insert(parts[0][0..parts[0].len() - 1].parse().unwrap(), monkeys.borrow_mut().len());
        let new_monkey = Monkey::new();
        monkeys.borrow_mut().push(new_monkey);
    }
    let mut i: usize = 0;
    for ln in input {
        let parts: Vec<&str> = ln.split(" ").collect();
        monkeys.borrow_mut()[i].set_data(&parts, &symbol_table);
        i += 1;
    }
    return (monkeys, symbol_table["root"], symbol_table["humn"]);
}

fn run(monkeys: Rc<RefCell<Vec<Monkey>>>, index: usize) -> i64 {
    fn dfs(monkeys: Rc<RefCell<Vec<Monkey>>>, index: usize) -> i64 {
        if monkeys.borrow_mut()[index].get_children().is_none() {
            return *monkeys.borrow_mut()[index].get_value_mut();
        }
        let m1 = monkeys.borrow_mut()[index].get_children().unwrap().0;
        let m2 = monkeys.borrow_mut()[index].get_children().unwrap().1;
        let a = dfs(monkeys.clone(), m1);
        let b = dfs(monkeys.clone(), m2);
        monkeys.borrow_mut()[index].get_equation().run(a, b)
    }
    return dfs(monkeys.clone(), index);
}

pub fn solve_a() {
    let input = fileio::input("src/day21.txt");
    let (monkeys, root_index, _) = create_tree(&input);
    println!("{}", run(monkeys.clone(), root_index));
}

pub fn solve_b() {
    let input = fileio::input("src/day21.txt");
    let (mut monkeys, root_index, human_index) = create_tree(&input);
    *monkeys.borrow_mut()[root_index].get_equation_mut() = Box::new(Subtraction{});
    monkeys.borrow_mut()[root_index].set_id(2);
    fn mark_dependants(monkeys: Rc<RefCell<Vec<Monkey>>>, human_index: usize, index: usize) -> bool {
        if index == human_index {
            *monkeys.borrow_mut()[index].get_marked_mut() = true;
            return true;
        }
        let children = *monkeys.borrow_mut()[index].get_children();
        if children.is_none() {
            return false;
        }
        if mark_dependants(monkeys.clone(), human_index, children.unwrap().0) || mark_dependants(monkeys.clone(), human_index, children.unwrap().1) {
            *monkeys.borrow_mut()[index].get_marked_mut() = true;
            return true;
        }
        return false;
    }
    mark_dependants(monkeys.clone(), human_index, root_index);
    let mut q: VecDeque<(usize, i64)> = VecDeque::new();
    q.push_back((root_index, 0));
    while !q.is_empty() {
        let (i, expected) = q.pop_front().unwrap();
        if i == human_index {
            println!("{}", expected);
            break;
        }
        let mut children = monkeys.borrow_mut()[i].get_children().unwrap();
        let mut marks = (false, false);
        marks.0 = *monkeys.borrow_mut()[children.0].get_marked_mut();
        marks.1 = *monkeys.borrow_mut()[children.1].get_marked_mut();
        if marks.0 && marks.1 {
            panic!("too many branches");
        }
        children = if marks.1 {children} else {(children.1, children.0)}; // 0 is always known
        let known = run(monkeys.clone(), children.0);
        let c_expected = match monkeys.borrow_mut()[i].id() {
            1 => {
                // addition
                expected - known
            },
            2 => {
                // subtraction
                if marks.0 {expected + known} else {known - expected}
            },
            3 => {
                // multiplication
                expected / known
            },
            4 => {
                // division
                if marks.0 {known * expected} else {known / expected}
            },
            _ => {
                // unknown
                panic!("unknown equation id");
            }
        };
        q.push_back((children.1, c_expected));
    }
    /*
    let input = fileio::input("src/day21.txt");
    let (monkeys, root_index, human_index) = create_tree(&input);
    *monkeys.borrow_mut()[root_index].get_equation_mut() = Box::new(Subtraction{});
    let mut q: PriorityQueue<i64, i64> = PriorityQueue::new();
    let mut min_diff = i64::MAX >> 1;
    let mut dx = 1 << 51;
    q.push(1 << 52, -min_diff);
    let mut solution = i64::MAX;
    while dx != 0 {
        let amount = q.len();
        for _ in 0..amount {
            let (x, _) = q.pop().unwrap();
            *monkeys.borrow_mut()[human_index].get_value_mut() = x;
            let diff = run(monkeys.clone(), root_index).abs();
            if diff == 0 {
                println!("{}", x);
                solution = min(solution, x);
            }
            q.push( x - dx, -diff);
            q.push( x + dx, -diff);
            min_diff = min(diff, min_diff);
        }
        dx >>= 1;
    }
    *monkeys.borrow_mut()[human_index].get_value_mut() = solution;
    println!("{}", run(monkeys.clone(), root_index));
     */
}