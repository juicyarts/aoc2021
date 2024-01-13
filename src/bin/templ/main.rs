extern crate aoc2021;
use std::{env, i32};

use crate::utils::read_lines::read_lines;
use aoc2021::utils;

fn main() {
    let args: Vec<String> = env::args().collect();
    let src = &args[1];

    let (p1, p2) = dX(src);

    println!("{}, {}", p1, p2);
}

fn dX(source: &str) -> (i32, i32) {
    if let Ok(lines) = read_lines(source) {
        lines.flatten().for_each(|line| {});
    }

    (0, 0)
}
