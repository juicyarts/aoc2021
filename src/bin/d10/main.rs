extern crate aoc2021;
use std::{collections::HashMap, env, i32};

use crate::utils::read_lines::read_lines;
use aoc2021::utils;

fn main() {
    let args: Vec<String> = env::args().collect();
    let src = &args[1];

    let (p1, p2) = d10_p1(src);

    println!("Day 10");
    println!("Part 1 {}", p1);
    println!("Part 2 {}", p2);
}

fn d10_p1(source: &str) -> (i32, i32) {
    let o = "([{<";
    let c_map = HashMap::from([("]", "["), (")", "("), ("}", "{"), (">", "<")]);
    let o_map = HashMap::from([("[", "]"), ("(", ")"), ("{", "}"), ("<", ">")]);
    let s_map = HashMap::from([("]", 57), (")", 3), ("}", 1197), (">", 25137)]);
    let mut sum = 0;
    if let Ok(lines) = read_lines(source) {
        for line in lines.flatten() {
            let mut l: String = "".to_string();
            line.split("\n").for_each(|row| {
                println!("Analyzing: {:?}", row);
                for s in row.chars() {
                    let sf: &str = &s.to_string();

                    if o.contains(s) {
                        l.push_str(&s.to_string());
                    } else {
                        let expected: &str = &l.chars().last().unwrap().to_string();
                        if c_map[sf] == expected {
                            l.remove(l.len() - 1);
                        } else {
                            sum += s_map[sf];
                            break;
                        }
                    }
                }
            });
        }
    }

    (sum, 0)
}
