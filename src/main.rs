use std::time::{Duration, Instant};
use year2022::day22;

mod fileio;
mod year2022;

fn main() {
    {
        let start = Instant::now();
        day22::solve_a();
        let duration = start.elapsed();
        println!("part a: {:?}", duration);
    }
    {
        let start = Instant::now();
        day22::solve_b();
        let duration = start.elapsed();
        println!("part b: {:?}", duration);
    }

}
