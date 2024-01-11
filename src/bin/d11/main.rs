extern crate aoc2021;
use std::{env, i32};

#[rustfmt::skip]
use crate::utils::read_lines::read_lines;
use aoc2021::utils;
use colored::*;

const DIRS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

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

fn d11_p1(source: &str, steps: i32) -> (usize, i64) {
    let mut l: Vec<Vec<i32>> = Vec::new();
    let mut rc = 0;
    let mut cc = 0;

    if let Ok(lines) = read_lines(source) {
        lines.flatten().for_each(|row| {
            rc += 1;
            let r = row
                .chars()
                .map(|c| {
                    if rc == 1 {
                        cc += 1;
                    }
                    return c as i32 - '0' as i32;
                })
                .collect();
            l.push(r);
        });
    }
    let count = sim(l, steps, rc, cc);

    (count, 0)
}

fn sim(mut m: Vec<Vec<i32>>, steps: i32, r: i32, c: i32) -> usize {
    (0..steps).fold(0, |acc, _| {
        m.iter_mut()
            .for_each(|row| row.iter_mut().for_each(|cell| *cell += 1));
        let ac = acc
            + (0..r)
                .flat_map(|y| (0..c).map(move |x| (x, y)))
                .fold(0, |acc, (x, y)| {
                    acc + (m[y as usize][x as usize] > 9)
                        .then(|| flash(&mut m, (x as usize, y as usize)))
                        .unwrap_or(0)
                });

        print_map(&m);

        ac
    })
}

fn flash(map: &mut Vec<Vec<i32>>, p: (usize, usize)) -> usize {
    map[p.1][p.0] = 0;
    DIRS.iter()
        .map(|(nx, ny)| ((p.0 as isize + nx) as usize, (p.1 as isize + ny) as usize))
        .fold(1, |acc, (x, y)| {
            match map.get_mut(y).and_then(|l| l.get_mut(x)) {
                Some(col) if *col > 0 => {
                    *col += 1;
                    acc + (*col > 9).then(|| flash(map, (x, y))).unwrap_or(0)
                }
                _ => acc,
            }
        })
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
