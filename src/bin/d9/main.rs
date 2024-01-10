extern crate aoc2021;
use std::{collections::HashMap, env, i32};

use colored::*;

use crate::utils::read_lines::read_lines;
use aoc2021::utils;

fn main() {
    let args: Vec<String> = env::args().collect();
    let src = &args[1];

    let (p1, p2) = d9_p1(src);

    println!("Day 8");
    println!("Part 1 {}", p1);
    println!("Part 2 {}", p2);
}

fn d9_p1(source: &str) -> (i32, i32) {
    let mut l: Vec<Vec<i32>> = Vec::new();

    if let Ok(lines) = read_lines(source) {
        lines.flatten().for_each(|row| {
            let r = row.chars().map(|c| c as i32 - '0' as i32).collect();
            l.push(r);
        });
    }

    let mut pt1 = 0;
    let mut pt2 = Vec::new();

    let dirs: Vec<(i32, i32)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
    l.iter().enumerate().for_each(|(r_i, row)| {
        row.iter().enumerate().for_each(|(c_i, col)| {
            let mut lnb = 0;

            dirs.iter().for_each(|i| {
                let next_r = r_i as i32 + i.0;
                let next_c = c_i as i32 + i.1;

                let isoob = next_r < 0
                    || next_r >= (l.len() as i32)
                    || next_c < 0
                    || next_c >= (row.len() as i32);

                if !isoob {
                    let nbr = l[next_r as usize][next_c as usize];
                    if nbr <= *col {
                        lnb += 1;
                    }
                }
            });

            if lnb == 0 {
                pt1 += 1 + col;
                let (c, visited) = collect(&l, &vec![r_i as i32, c_i as i32], HashMap::new(), 0);
                if visited.len() > 2 {
                    pt2.push(c);
                }
            }
        });
    });

    pt2.sort_by(|a, b| b.cmp(a));
    (pt1, pt2[0] as i32 * pt2[1] as i32 * pt2[2] as i32)
}

fn collect(
    m: &Vec<Vec<i32>>,
    start: &Vec<i32>,
    mut visited: HashMap<(i32, i32), i32>,
    mut count: i32,
) -> (i32, HashMap<(i32, i32), i32>) {
    let entry = visited.get(&(start[0], start[1]));
    match entry {
        Some(c) => {
            if *c > 0 {
                return (count, visited);
            }
        }
        None => {
            let foo = visited.entry((start[0], start[1])).or_insert(1);
            *foo = 1;
            count += 1;
        }
    }

    let dirs: Vec<(i32, i32)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];

    dirs.iter().for_each(|i| {
        let next = vec![start[0] as i32 + i.0, start[1] as i32 + i.1];

        let isoob = next[0] < 0
            || next[0] >= (m.len() as i32)
            || next[1] < 0
            || next[1] >= (m[next[0] as usize].len() as i32);

        if !isoob {
            let nc = m[next[0] as usize][next[1] as usize];
            if nc != 9 {
                let (c, vis) = collect(m, &vec![next[0], next[1]], visited.clone(), 0);
                count += c;
                visited = vis;
            }
        }
    });

    (count, visited)
}
