use std::cmp::{max, min};
use crate::fileio;

struct SensorReach {
    sensor: (i32, i32),
    beacon: (i32, i32),
    radius: i32,
}

fn create_sr_vec(input: &Vec<String>) -> Vec<SensorReach> {
    let mut sr_vec: Vec<SensorReach> = Vec::new();
    for ln in input {
        let parts: Vec<&str> = ln.split(" ").collect();
        let sensor = (
            parts[2][2..parts[2].len()-1].parse::<i32>().unwrap(),
            parts[3][2..parts[3].len()-1].parse::<i32>().unwrap()
        );
        let beacon = (
            parts[8][2..parts[8].len()-1].parse::<i32>().unwrap(),
            parts[9][2..parts[9].len()].parse::<i32>().unwrap()
        );
        sr_vec.push(SensorReach {
            sensor,
            beacon,
            radius: (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs()
        });
    }
    return sr_vec;
}

pub fn solve_a() {
    let input = fileio::input("src/day15.txt");
    let sr_vec = create_sr_vec(&input);
    const ROW_Y: i32 = 2000000;
    enum Mark {
        RangeStart,
        RangeEnd,
        Beacon,
    }
    let mut marks: Vec<(i32, Mark)> = Vec::new();
    for sr in &sr_vec {
        let d = sr.radius - (ROW_Y - sr.sensor.1).abs();
        if d < 0 {continue};
        marks.push((sr.sensor.0 - d, Mark::RangeStart));
        marks.push((sr.sensor.0 + d + 1, Mark::RangeEnd));
        if sr.beacon.1 == ROW_Y {
            marks.push((sr.beacon.0, Mark::Beacon));
        }
    }
    marks.sort_by(|a, b| {
        a.0.cmp(&b.0)
    });
    let mut sum = 0;
    let mut wind = 0;
    let mut start = 0;
    let mut last_beacon_x = i32::MIN;
    for m in marks {
        match m.1 {
            Mark::RangeStart => {
                if wind == 0 {
                    start = m.0;
                }
                wind += 1;
            }
            Mark::RangeEnd => {
                wind -= 1;
                if wind == 0 {
                    sum += m.0 - start;
                }
            }
            Mark::Beacon => {
                if wind != 0 && last_beacon_x != m.0 {
                    sum -= 1;
                }
                last_beacon_x = m.0;
            }
        }
    }
    println!("{}", sum);
}

// after checking reddit - faster solution: rotate by 45 degrees and calc intersections to find hole
pub fn solve_b() {
    let input = fileio::input("src/day15.txt");
    let sr_vec = create_sr_vec(&input);
    enum Mark {
        RangeStart,
        RangeEnd,
    }
    const POS_CAP: usize = 4000000;
    let mut marks: Vec<(i32, i32, Mark)> = Vec::new();
    for sr in &sr_vec {
        let y_range: (i32, i32) = (max(0, sr.sensor.1 - sr.radius), min(POS_CAP as i32, sr.sensor.1 + sr.radius));
        if y_range.0 > y_range.1 { continue }
        for y in y_range.0..=y_range.1 {
            let d = sr.radius - (sr.sensor.1 - y).abs();
            let x_range: (i32, i32) = (max(0, sr.sensor.0 - d), min(POS_CAP as i32, sr.sensor.0 + d));
            marks.push((y, x_range.0, Mark::RangeStart));
            marks.push((y, x_range.1 + 1, Mark::RangeEnd));
        }
    }
    println!("marked");
    marks.sort_by(|a, b| {
        if a.0 == b.0 {a.1.cmp(&b.1)} else {a.0.cmp(&b.0)}
    });
    println!("sorted");
    let mut wind = 0;
    let mut start = 0;
    let mut y = 0;
    let mut x = 0;
    if marks[0].0 != 0 || marks[0].1 != 0 {
        println!("{}", 0);
        return;
    }
    for m in marks {
        /*
        println!("{},{},{}", m.0, m.1, match m.2 {
            Mark::RangeStart => {"S"}
            Mark::RangeEnd => {"E"}
        });
         */
        match m.2 {
            Mark::RangeStart => {
                if y != m.0 {
                    y = m.0;
                    wind = 0;
                }
                if wind == 0 {
                    start = m.0;
                    if x < m.1 {
                        print!("{}", (m.1 as u64 - 1) * 4000000 + m.0 as u64);
                        break;
                    }
                }
                wind += 1;
            }
            Mark::RangeEnd => {
                wind -= 1;
            }
        }
        x = m.1;
    }
}