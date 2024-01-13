extern crate aoc2021;

use crate::utils::read_lines::read_lines;
use aoc2021::utils;
use std::{collections::HashMap, env, i64};

fn main() {
    let args: Vec<String> = env::args().collect();
    let src = &args[1];
    let s = &args[2];
    let steps = s.parse::<i32>().unwrap_or(4);

    let (p1, p2) = d_14(src, steps);

    println!("{}, {}", p1, p2);
}

fn d_14(source: &str, steps: i32) -> (i64, i64) {
    let mut template: String = "".to_string();
    let mut rules: HashMap<String, String> = HashMap::new();

    if let Ok(lines) = read_lines(source) {
        lines.flatten().enumerate().for_each(|(i, line)| {
            if i == 0 {
                template = line.to_string();
            } else if line.is_empty() != true {
                let f: Vec<_> = line.split(" -> ").collect();
                rules.insert(f[0].to_string(), f[1].to_string());
            }
        });
    }

    let mut pair_count: HashMap<String, i64> = HashMap::new();
    let mut char_count: HashMap<String, i64> = HashMap::new();

    for i in 0..template.chars().count() {
        let cc = char_count
            .entry(template.chars().nth(i).unwrap().to_string())
            .or_insert(0);
        *cc += 1;

        if i == 0 {
            continue;
        }

        let c = format!(
            "{}{}",
            template.chars().nth(i - 1).unwrap().to_string(),
            template.chars().nth(i).unwrap().to_string()
        );

        let pc = pair_count.entry(c).or_insert(0);
        *pc += 1;
    }

    for _ in 0..steps {
        let mut new_pair_count: HashMap<String, i64> = HashMap::new();
        for (s, v) in pair_count.clone() {
            let rule = rules.get(&s);
            match rule {
                Some(c) => {
                    let new_a = format!("{}{}", s.chars().nth(0).unwrap(), c);
                    let new_b = format!("{}{}", c, s.chars().nth(1).unwrap());
                    let a = new_pair_count.entry(new_a).or_insert(0);
                    *a += v;
                    let b = new_pair_count.entry(new_b).or_insert(0);
                    *b += v;
                    let c = char_count.entry(c.to_string()).or_insert(0);
                    *c += v;
                }
                _ => {}
            }
        }

        pair_count = new_pair_count;
    }

    let mut ac: Vec<i64> = char_count.iter_mut().map(|i| *i.1).collect::<Vec<i64>>();
    ac.sort();
    (ac[ac.len() - 1] - ac[0], 0)
}
