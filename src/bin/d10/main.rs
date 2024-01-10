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

fn d10_p1(source: &str) -> (i32, i64) {
    let o = "([{<";
    let c_map = HashMap::from([("]", "["), (")", "("), ("}", "{"), (">", "<")]);
    let o_map = HashMap::from([("[", "]"), ("(", ")"), ("{", "}"), ("<", ">")]);
    let s_map = HashMap::from([("]", 57), (")", 3), ("}", 1197), (">", 25137)]);
    let s2_map = HashMap::from([("[", 2), ("(", 1), ("{", 3), ("<", 4)]);
    let mut sum = 0;
    let mut rest: Vec<String> = Vec::new();

    if let Ok(lines) = read_lines(source) {
        for line in lines.flatten() {
            'outer: for row in line.split("\n") {
                let mut left: String = "".to_string();
                for s in row.chars() {
                    let sf: &str = &s.to_string();
                    if o.contains(s) {
                        left.push_str(&s.to_string());
                    } else {
                        let expected: &str = &left.chars().last().unwrap().to_string();
                        if c_map[sf] == expected {
                            left.remove(left.len() - 1);
                        } else {
                            sum += s_map[sf];
                            break 'outer;
                        }
                    }
                }
                rest.push(left);
            }
        }
    }

    let mut points: Vec<i64> = Vec::new();
    for l in rest {
        let mut sum2: i64 = 0;
        let mut right: String = "".to_string();
        for c in l.chars().rev() {
            let cs: &str = &c.to_string();
            right.push_str(o_map[cs]);
            sum2 = (sum2 * 5) + s2_map[&cs];
        }
        points.push(sum2);
    }

    points.sort();
    (sum, points[points.len() / 2])
}
