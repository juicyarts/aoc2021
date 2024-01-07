extern crate aoc2021;
use std::env;

use crate::utils::read_lines::read_lines;
use aoc2021::utils;

fn main() {
    let args: Vec<String> = env::args().collect();
    let src = &args[1];

    let (p1, p2) = d7_p1(src);

    println!("Day 7");
    println!("Part 1 {}", p1);
    println!("Part 2 {}", p2);
}

fn d7_p1(source: &str) -> (f64, f64) {
    let mut nums: Vec<i32> = Vec::new();
    let mut i_min = 10000000;
    let mut i_max = 0;
    let mut total: usize = 0;

    if let Ok(lines) = read_lines(source) {
        lines.flatten().for_each(|line| {
            line.split(",").for_each(|n| {
                let i = n.parse::<i32>().unwrap();
                nums.push(i);
                total += i as usize;
                i_min = i_min.min(i);
                i_max = i_max.max(i);
            })
        })
    }

    let mut min = f64::INFINITY;

    (i_min..i_max).for_each(|i| {
        let foo: i32 = nums.iter().map(|j| (j - i).abs()).sum();
        min = min.min(foo as f64);
    });

    let mut min_multi = f64::INFINITY;

    (i_min..i_max).for_each(|i| {
        let foo: i32 = nums
            .iter()
            .map(|j| (i - j).abs() * ((i - j).abs() + 1) / 2)
            .sum();
        min_multi = min_multi.min(foo as f64);
    });

    (min, min_multi)
}
