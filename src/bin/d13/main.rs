extern crate aoc2021;
use std::{collections::HashMap, env, i32};

use colored::*;

use crate::utils::read_lines::read_lines;
use aoc2021::utils;

fn main() {
    let args: Vec<String> = env::args().collect();
    let src = &args[1];

    let (p1, _) = d13(src);

    println!("Day 13");
    println!("Part 1 {}", p1);
}

fn d13(source: &str) -> (i32, i32) {
    let mut points: HashMap<(i32, i32), i32> = HashMap::new();
    let mut folds: Vec<(String, i32)> = Vec::new();

    let mut max_x = 0;
    let mut max_y = 0;

    if let Ok(lines) = read_lines(source) {
        lines.flatten().for_each(|line| {
            if line.contains("fold") {
                let row: Vec<String> = line
                    .replace("fold along ", "")
                    .split("=")
                    .map(String::from)
                    .collect::<Vec<String>>();
                folds.push((row[0].to_owned(), row[1].parse::<i32>().unwrap()));
            } else if line.is_empty() == false {
                let row: Vec<_> = line.split(",").collect();
                let (x, y) = (
                    row[0].parse::<i32>().unwrap(),
                    row[1].parse::<i32>().unwrap(),
                );

                max_x = max_x.max(x);
                max_y = max_y.max(y);

                points.insert((x, y), 1);
            }
        });
    }

    let mut count = 0;
    for f in &folds {
        (count, points, max_x, max_y) = fold(&f.1, &f.0, points.clone(), &max_x, &max_y);
    }

    print_map(&points, &max_x, &max_y);
    (count, 0)
}

fn fold(
    at: &i32,
    dir: &str,
    m: HashMap<(i32, i32), i32>,
    max_x: &i32,
    max_y: &i32,
) -> (i32, HashMap<(i32, i32), i32>, i32, i32) {
    let mut nm: HashMap<(i32, i32), i32> = HashMap::new();
    let max = if dir == "x" { max_y } else { max_x };
    (
        (0..*at)
            .flat_map(|x| (0..*max + 1).map(move |y| if dir == "x" { (x, y) } else { (y, x) }))
            .fold(0, |ac, (x, y)| {
                let a = m.get(&(x, y)).unwrap_or(&0);
                let mut b = m.get(&(x, max_y - y)).unwrap_or(&0);

                if dir == "x" {
                    b = m.get(&(max_x - x, y)).unwrap_or(&0);
                }

                if a + b > 0 {
                    nm.insert((x, y), 1);
                    ac + 1
                } else {
                    ac
                }
            }),
        nm,
        if dir == "x" { at - 1 } else { *max_x },
        if dir == "x" { *max_y } else { at - 1 },
    )
}

fn print_map(vis: &HashMap<(i32, i32), i32>, max_x: &i32, max_y: &i32) {
    println!("Printing Map: {:?}*{:?} \n", max_x, max_y);
    for y in 0..=*max_y {
        print!("\n");
        for x in 0..=*max_x {
            let entry = vis.get(&(x, y));
            match entry {
                Some(count) => {
                    if count > &1 {
                        print!("{}", "#".red())
                    } else {
                        print!("{}", "#".green())
                    }
                }
                None => print!("{}", ".".blue()),
            }
        }
    }
    println!();
}
