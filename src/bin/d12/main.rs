extern crate aoc2021;
use std::{collections::HashMap, env, i32};

use crate::utils::read_lines::read_lines;
use aoc2021::utils;

fn main() {
    let args: Vec<String> = env::args().collect();
    let src = &args[1];

    let (p1, p2) = d12(src);

    println!("Day 12");
    println!("Part 1 {}", p1);
    println!("Part 2 {}", p2);
}

fn d12(source: &str) -> (i32, i32) {
    let mut connections: HashMap<String, Vec<String>> = HashMap::new();

    if let Ok(lines) = read_lines(source) {
        lines.flatten().for_each(|line| {
            let row: Vec<_> = line.split("-").collect();
            connections.entry(row[0].to_string()).or_insert(Vec::new());
            connections
                .get_mut(&row[0].to_string())
                .unwrap()
                .push(row[1].to_string());
            connections.entry(row[1].to_string()).or_insert(Vec::new());
            connections
                .get_mut(&row[1].to_string())
                .unwrap()
                .push(row[0].to_string());
        });
    }

    let mut cnt = 0;
    let mut cntd = 0;
    for c in &connections["start"] {
        cnt += travel(c, &connections, HashMap::new(), 0);
        cntd += travel(c, &connections, HashMap::new(), 1);
    }

    (cnt, cntd)
}

fn travel(
    p: &str,
    con: &HashMap<String, Vec<String>>,
    mut visited: HashMap<String, i32>,
    mut dbl: i32,
) -> i32 {
    let mut cnt = 0;

    if p == "end" {
        return 1;
    }

    if p == "start" {
        return 0;
    }

    if p != "end" && p.chars().all(char::is_lowercase) {
        let entry = visited.get(p);
        match entry {
            Some(cnt) => {
                if *cnt > 0 {
                    if dbl > 0 {
                        dbl -= 1;
                    } else {
                        return 0;
                    }
                }
            }
            None => {
                visited.entry(p.to_string()).or_insert(1);
            }
        }
    }

    let cs = &con[p];
    for c in cs {
        cnt += travel(&c.to_string(), &con, visited.clone(), dbl);
    }

    return cnt;
}
