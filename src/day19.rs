use std::cell::RefCell;
use std::cmp::{max, min};
use std::collections::{HashSet, LinkedList};
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use crate::fileio;

struct BluePrint {
    ore_robot: u32, // ore cost
    clay_robot: u32, // ore cost
    obsidian_robot: (u32, u32), // (ore, clay) cost
    geode_robot: (u32, u32) // (ore, obsidian) cost
}

fn solve(input: &Vec<String>, time: u32) -> Vec<u32> {
    let mut id = 0;
    let mut bp_max_geodes: Vec<u32> = Vec::new();
    for ln in input {
        id += 1;
        // read blueprint from input
        let parts: Vec<&str> = ln.splitn(32, " ").collect();
        let bp = BluePrint {
            ore_robot: parts[6].parse().unwrap(),
            clay_robot: parts[12].parse().unwrap(),
            obsidian_robot: (
                parts[18].parse().unwrap(),
                parts[21].parse().unwrap()
            ),
            geode_robot: (
                parts[27].parse().unwrap(),
                parts[30].parse().unwrap()
            )
        };

        // dfs
        struct State {
            time: u32,
            ore_robots: u32,
            clay_robots: u32,
            obsidian_robots: u32,
            geode_robots: u32,
            ore: u32,
            clay: u32,
            obsidian: u32,
            geodes: u32,
        }

        impl Copy for State {}

        impl Clone for State {
            fn clone(&self) -> Self {
                State {
                    time: self.time,
                    ore_robots: self.ore_robots,
                    clay_robots: self.clay_robots,
                    obsidian_robots: self.obsidian_robots,
                    geode_robots: self.geode_robots,
                    ore: self.ore,
                    clay: self.clay,
                    obsidian: self.obsidian,
                    geodes: self.geodes,
                }
            }
        }

        impl Eq for State {}

        impl PartialEq for State {
            fn eq(&self, other: &Self) -> bool {
                self.time == other.time &&
                    self.ore_robots == other.ore_robots &&
                    self.clay_robots == other.clay_robots &&
                    self.obsidian_robots == other.obsidian_robots &&
                    self.geode_robots == other.geode_robots &&
                    self.ore == other.ore &&
                    self.clay == other.clay &&
                    self.obsidian == other.obsidian &&
                    self.geodes == other.geodes
            }
        }

        impl Hash for State {
            fn hash<H: Hasher>(&self, state: &mut H) {
                self.time.hash(state);
                self.ore_robots.hash(state);
                self.clay_robots.hash(state);
                self.obsidian_robots.hash(state);
                self.geode_robots.hash(state);
                self.ore.hash(state);
                self.clay.hash(state);
                self.obsidian.hash(state);
                self.geodes.hash(state);
            }
        }

        impl State {
            fn pass_time(&mut self, time: u32) {
                self.time -= time;
                self.ore += self.ore_robots * time;
                self.clay += self.clay_robots * time;
                self.obsidian += self.obsidian_robots * time;
                self.geodes += self.geode_robots * time;
            }

            fn build_ore_robot(&self, bp: &BluePrint) -> Option<State> {
                let mut new_state = self.clone();
                let ore_required = if self.ore >= bp.ore_robot {0} else {bp.ore_robot - self.ore};
                let mut time_required = 1 + (self.ore_robots + ore_required - 1) / self.ore_robots;
                if time_required > self.time {
                    return None;
                }
                new_state.pass_time(time_required);
                new_state.ore_robots += 1;
                new_state.ore -= bp.ore_robot;
                return Some(new_state);
            }

            fn build_clay_robot(&self, bp: &BluePrint) -> Option<State> {
                let mut new_state = self.clone();
                let ore_required = if self.ore >= bp.clay_robot {0} else {bp.clay_robot - self.ore};
                let mut time_required = 1 + (self.ore_robots + ore_required - 1) / self.ore_robots;
                if time_required > self.time {
                    return None;
                }
                new_state.pass_time(time_required);
                new_state.clay_robots += 1;
                new_state.ore -= bp.clay_robot;
                return Some(new_state);
            }

            fn build_obsidian_robot(&self, bp: &BluePrint) -> Option<State> {
                let mut new_state = self.clone();
                let ore_required = if self.ore >= bp.obsidian_robot.0 {0} else {bp.obsidian_robot.0 - self.ore};
                let clay_required = if self.clay >= bp.obsidian_robot.1 {0} else {bp.obsidian_robot.1 - self.clay};
                let mut time_required = 1 + max(
                    (self.ore_robots + ore_required - 1) / self.ore_robots,
                    (self.clay_robots + clay_required - 1) / self.clay_robots
                );
                if time_required > self.time {
                    return None;
                }
                new_state.pass_time(time_required);
                new_state.obsidian_robots += 1;
                new_state.ore -= bp.obsidian_robot.0;
                new_state.clay -= bp.obsidian_robot.1;
                return Some(new_state);
            }

            fn build_geode_robot(&self, bp: &BluePrint) -> Option<State> {
                let ore_required = if self.ore >= bp.geode_robot.0 {0} else {bp.geode_robot.0 - self.ore};
                let obsidian_required = if self.obsidian >= bp.geode_robot.1 {0} else {bp.geode_robot.1 - self.obsidian};
                let mut time_required = 1 + max(
                    (self.ore_robots + ore_required - 1) / self.ore_robots,
                    (self.obsidian_robots + obsidian_required - 1) / self.obsidian_robots
                );
                if time_required > self.time {
                    return None;
                }
                let mut new_state = self.clone();
                new_state.pass_time(time_required);
                new_state.geode_robots += 1;
                new_state.ore -= bp.geode_robot.0;
                new_state.obsidian -= bp.geode_robot.1;
                return Some(new_state);
            }
        }
        let mut max_geodes: u32 = 0;
        let mut earliest_geode: u32 = 0;
        let mut q: LinkedList<State> = LinkedList::new();
        q.push_back(State {
            time: time - 1,
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
            ore: 1,
            clay: 0,
            obsidian: 0,
            geodes: 0
        });

        let max_ore_robots_needed = bp.clay_robot + bp.obsidian_robot.0 + bp.geode_robot.0;
        while !q.is_empty() {
            let s = q.pop_front().unwrap();
            let potential = s.geodes + if s.time == 0 {s.geode_robots} else {s.time * (2 * s.geode_robots + s.time) / 2};
            if potential <= max_geodes {
                continue;
            }
            let geodes = s.geodes + s.geode_robots * s.time;
            max_geodes = max(max_geodes, geodes);

            if s.obsidian_robots > 0 {
                let new_state = s.build_geode_robot(&bp);
                if new_state.is_some() {
                    earliest_geode = max(earliest_geode, new_state.unwrap().time);
                    q.push_front(new_state.unwrap());
                }
            }
            if s.time >= 4 && s.clay_robots > 0 && s.obsidian_robots < bp.geode_robot.1 {
                let new_state = s.build_obsidian_robot(&bp);
                if new_state.is_some() {
                    q.push_front(new_state.unwrap());
                }
            }
            if s.time >= 7 && s.clay_robots < bp.obsidian_robot.1 {
                let new_state = s.build_clay_robot(&bp);
                if new_state.is_some() {
                    q.push_front(new_state.unwrap());
                }
            }
            if s.time >= 16 && s.ore_robots < max_ore_robots_needed {
                let new_state = s.build_ore_robot(&bp);
                if new_state.is_some() {
                    q.push_front(new_state.unwrap());
                }
            }
        }
        bp_max_geodes.push(max_geodes);
    }
    return bp_max_geodes;
}

pub fn solve_a() {
    let input = fileio::input("src/day19.txt");
    let bp_max_geodes = solve(&input, 24);
    let mut quality_sum = 0;
    for id in 0..bp_max_geodes.len() {
        quality_sum += ((id + 1) * bp_max_geodes[id] as usize) as u32;
    }
    println!("{}", quality_sum);
}

pub fn solve_b() {
    let mut input = fileio::input("src/day19.txt");
    input = input[0..3].to_owned();
    let bp_max_geodes = solve(&input, 32);
    let mut prod: u32 = 1;
    for mg in bp_max_geodes {
        prod *= mg;
    }
    println!("{}", prod);
}