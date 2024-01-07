extern crate aoc2021;
use std::env;

use crate::utils::read_lines::read_lines;
use aoc2021::utils;

fn main() {
    let args: Vec<String> = env::args().collect();
    let src = &args[1];
    let one = d1_p1(src);
    let two = d1_p2(src);
    println!("{}", one);
    println!("{}", two);
}

#[allow(dead_code)]
pub fn d1_p1(source: &str) -> i32 {
    let mut output: Vec<i32> = Vec::new();

    if let Ok(lines) = read_lines(source) {
        for line in lines.flatten() {
            if let Ok(num) = line.parse::<i32>() {
                output.push(num);
            }
        }
    }

    output.iter().enumerate().fold(0, |accum, (index, item)| {
        if index == 0 {
            return accum;
        }

        if output[index - 1] < *item {
            accum + 1
        } else {
            accum
        }
    })
}

#[allow(dead_code)]
pub fn d1_p2(source: &str) -> i32 {
    let mut output: Vec<i32> = Vec::new();

    if let Ok(lines) = read_lines(source) {
        for line in lines.flatten() {
            if let Ok(num) = line.parse::<i32>() {
                output.push(num);
            }
        }
    }

    output.iter().enumerate().fold(0, |accum, (index, item)| {
        if index < 3 {
            accum
        } else {
            if item + output[index - 1] + output[index - 2]
                > output[index - 1] + output[index - 2] + output[index - 3]
            {
                accum + 1
            } else {
                accum
            }
        }
    })
}

#[test]
fn d1_test_example() {
    assert_eq!(d1_p1(&"./src/bin/d1/e.txt"), 7);
}

#[test]
fn d1_p1_test() {
    dotenv::dotenv().ok();
    assert_eq!(
        d1_p1(&"./src/bin/d1/i.txt"),
        dotenv::var("d1_p1").unwrap().parse::<i32>().unwrap()
    );
}

#[test]
fn d1_p2_test_example() {
    assert_eq!(d1_p2(&"./src/bin/d1/e.txt"), 5);
}

#[test]
fn d1_p2_test() {
    assert_eq!(
        d1_p2(&"./src/bin/d1/i.txt"),
        dotenv::var("d1_p2").unwrap().parse::<i32>().unwrap()
    )
}
