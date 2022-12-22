use std::time::{Duration, Instant};

mod fileio;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;

fn main() {
    {
        let start = Instant::now();
        day18::solve_a();
        let duration = start.elapsed();
        println!("part a: {:?}", duration);
    }
    {
        let start = Instant::now();
        day18::solve_b();
        let duration = start.elapsed();
        println!("part b: {:?}", duration);
    }

}
