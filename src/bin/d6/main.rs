extern crate aoc2021;
use std::{collections::VecDeque, env, u32};

use crate::utils::read_lines::read_lines;
use aoc2021::utils;

fn main() {
    let args: Vec<String> = env::args().collect();
    let src = &args[1];
    let d: &String = &args[2];
    let t: &String = &args[3];
    let days = d.parse::<u32>().unwrap();

    let p1 = d6_p1(src, days);
    let p2 = 0;

    println!("Day 6");
    println!("Part 1 {}", p1);
    println!("Part 2 {}", p2);
}

fn d6_p1(source: &str, days: u32) -> u64 {
    let mut counter = VecDeque::from(vec![0; 9]);
    if let Ok(lines) = read_lines(source) {
        lines.flatten().for_each(|line| {
            line.split(",").for_each(|s| {
                if s == "" {
                    return;
                }
                let c = s.parse::<usize>().unwrap();
                counter[c] += 1;
            });
        })
    }

    for _ in 0..days {
        let new_fish = counter.pop_front().unwrap();
        counter[6] += new_fish;
        counter.push_back(new_fish);
    }

    counter.into_iter().sum()
}

// intial idea but could not make it work.
// rather than iterating over the days,
// iterate over the fish, calculate how many times they will get new fish in given time
// (days - days left per fish) / time to evolve
// for those new fish run expo based on the days left when they were added
// (expo(days - (x*tte)))
// basic exponential growth calc
fn expo(time: &f64, rate: &f64, tte: &f64) -> f64 {
    let b = 2f64.powf(1.0 / tte);
    (rate * b.powf(*time)).floor()
}

#[test]
fn expo_test() {
    assert_eq!(expo(&0.0, &1.0, &8.0), 1.0);
    assert_eq!(expo(&1.0, &1.0, &7.0).abs(), 1.0);
    assert_eq!(expo(&3.0, &1000.0, &3.0).abs(), 2000.0);
    assert_eq!(expo(&27.0, &1000.0, &3.0).abs(), 512000.0);
}
