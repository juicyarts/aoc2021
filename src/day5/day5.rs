use crate::utils::read_lines::read_lines;
use colored::*;
use std::{cmp::max, collections::HashMap};

fn parse_input(source: &str) -> (HashMap<(i32, i32), i32>, HashMap<(i32, i32), i32>, i32, i32) {
    let mut non_diagonal = HashMap::new();
    let mut all = HashMap::new();
    let mut max_x = 0;
    let mut max_y = 0;

    if let Ok(lines) = read_lines(source) {
        for line in lines.flatten() {
            if let Ok(l) = line.parse::<String>() {
                let root: Vec<_> = l.split(" -> ").collect();
                let a: Vec<_> = root[0].split(",").collect();
                let (x1, y1) = (
                    a[0].parse::<i32>().unwrap().abs(),
                    a[1].parse::<i32>().unwrap().abs(),
                );
                let b: Vec<_> = root[1].split(",").collect();
                let (x2, y2) = (
                    b[0].parse::<i32>().unwrap().abs(),
                    b[1].parse::<i32>().unwrap().abs(),
                );

                max_x = max_x.max(max(x1, x2));
                max_y = max_y.max(max(y1, y2));

                let dx = x2 - x1;
                let dy = y2 - y1;

                for i in 0..(1 + max(dx.abs(), dy.abs())) {
                    let x = x1
                        + (if dx > 0 {
                            1
                        } else {
                            if dx < 0 {
                                -1
                            } else {
                                0
                            }
                        }) * i;

                    let y = y1
                        + (if dy > 0 {
                            1
                        } else {
                            if dy < 0 {
                                -1
                            } else {
                                0
                            }
                        }) * i;

                    if dx == 0 || dy == 0 {
                        let entry = non_diagonal.entry((x, y)).or_insert(0);
                        *entry += 1;
                    }

                    let ntry = all.entry((x, y)).or_insert(0);
                    *ntry += 1;
                }
            }
        }
    }
    (non_diagonal, all, max_x, max_y)
}

fn print_map(lines: &HashMap<(i32, i32), i32>, max_x: i32, max_y: i32) {
    println!("Printing Map: {:?}*{:?} \n", max_x, max_y);
    for x in 0..=max_x {
        print!("\n");
        for y in 0..=max_y {
            let entry = lines.get(&(y, x));

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

#[allow(dead_code)]
pub fn day5_p1(source: &str) -> usize {
    let (one, _, max_x, max_y) = parse_input(source);
    let res = one.iter().filter(|x| x.1 > &1).count();
    print_map(&one, max_x, max_y);
    res
}

#[test]
pub fn day5_ex_p1() {
    assert_eq!(day5_p1(&"./src/day5/input_example.txt"), 5);
}

#[test]
fn day5_test_p1() {
    assert_eq!(
        day5_p1(&"./src/day5/input.txt"),
        dotenv::var("d5_p1").unwrap().parse::<usize>().unwrap()
    )
}

#[allow(dead_code)]
pub fn day5_p2(source: &str) -> usize {
    let (_, two, max_x, max_y) = parse_input(source);
    let res = two.iter().filter(|x| x.1 > &1).count();
    print_map(&two, max_x, max_y);
    res
}

#[test]
pub fn day5_ex_p2() {
    assert_eq!(day5_p2(&"./src/day5/input_example.txt"), 12);
}

#[test]
fn day5_test_p2() {
    assert_eq!(
        day5_p2(&"./src/day5/input.txt"),
        dotenv::var("d5_p2").unwrap().parse::<usize>().unwrap()
    )
}
