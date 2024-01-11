extern crate aoc2021;
use std::{collections::HashMap, env, i32};

use crate::utils::read_lines::read_lines;
use aoc2021::utils;
use colored::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let src = &args[1];
    let s = &args[2];
    let steps = s.parse::<i32>().unwrap();

    let (p1, p2) = d11_p1(src, steps);

    println!("Day 11");
    println!("Part 1 {}", p1);
    println!("Part 2 {}", p2);
}

fn d11_p1(source: &str, steps: i32) -> (i32, i64) {
    let mut l: Vec<Vec<i32>> = Vec::new();

    if let Ok(lines) = read_lines(source) {
        lines.flatten().for_each(|row| {
            let r = row.chars().map(|c| c as i32 - '0' as i32).collect();
            l.push(r);
        });
    }

    let count = sim(l, steps);

    (count, 0)
}

fn sim(m: Vec<Vec<i32>>, steps: i32) -> i32 {
    let mut flash_count = 0;
    let mut fin: Vec<Vec<i32>> = m;

    println!("BEFORE -----------------");
    print_map(&fin);
    println!("-------------");

    for s in 0..steps {
        let mut nm: Vec<Vec<i32>> = fin
            .iter()
            .map(|row| row.iter().map(|c| *c + 1).collect::<Vec<_>>())
            .collect();
        for (i, row) in nm.clone().iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                if *col >= 10 {
                    let (c, nmm) = flash(&vec![i as i32, j as i32], nm);
                    nm = nmm;
                    flash_count += c;
                }
            }
        }

        fin = nm
            .iter()
            .map(|row| {
                row.iter()
                    .map(|c| if *c == -1 { 0 } else { *c })
                    .collect::<Vec<i32>>()
            })
            .collect();

        println!();
        println!("AFTER STEP {:?}", s);
        print_map(&fin);
    }

    flash_count
}

fn flash(p: &Vec<i32>, mut m: Vec<Vec<i32>>) -> (i32, Vec<Vec<i32>>) {
    m[p[0] as usize][p[1] as usize] = -1;
    let mut count = 1;

    let dirs: Vec<(i32, i32)> = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 0),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for dir in dirs {
        let next = vec![p[0] + dir.0, p[1] + dir.1];
        let isib = (next[0] >= 0
            && next[0] < (m.len() as i32)
            && next[1] >= 0
            && next[1] < (m[p[0] as usize].len() as i32)
            && m[next[0] as usize][next[1] as usize] != -1);

        if !isib {
            continue;
        }

        m[next[0] as usize][next[1] as usize] += 1;

        if m[next[0] as usize][next[1] as usize] >= 10 {
            let (c, nm) = flash(&next, m);
            count += c;
            m = nm;
        }
    }

    //    println!("Round trip ended with {:?}", count);
    (count, m)
}

fn print_map(m: &Vec<Vec<i32>>) {
    for row in m {
        println!();
        for col in row {
            if *col == 0 {
                print!("{}", 0.to_string().red());
            } else {
                print!("{}", col.to_string().blue());
            }
        }
    }

    println!()
}
