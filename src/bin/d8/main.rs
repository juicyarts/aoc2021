extern crate aoc2021;
use std::env;

use crate::utils::read_lines::read_lines;
use aoc2021::utils;

fn main() {
    let args: Vec<String> = env::args().collect();
    let src = &args[1];

    let (p1, p2) = d8_p1(src);

    println!("Day 8");
    println!("Part 1 {}", p1);
    println!("Part 2 {}", p2);
}

fn d8_p1(source: &str) -> (f64, f64) {
    let mut known_digits = vec![0; 10];
    let mut total: i32 = 0;

    if let Ok(lines) = read_lines(source) {
        lines.flatten().for_each(|line| {
            let mut chars: Vec<String> = vec!["".to_string(); 10];

            let left = line.split("|").collect::<Vec<_>>()[0]
                .split_whitespace()
                .collect::<Vec<_>>();

            let secrets = line.split("|").collect::<Vec<_>>()[1]
                .split_whitespace()
                .map(|i| i.to_string())
                .collect::<Vec<_>>();

            // pt 1
            left.iter().for_each(|i| {
                if i.len() == 2 {
                    chars[1] = i.to_string();
                } else if i.len() == 3 {
                    chars[7] = i.to_string();
                } else if i.len() == 4 {
                    chars[4] = i.to_string();
                } else if i.len() == 7 {
                    chars[8] = i.to_string();
                }
                known_digits[i.len()] += 1;
            });

            let mut cnt = "".to_string();

            for secret in &secrets {
                if secret.len() == 2 {
                    cnt.push_str(&1.to_string())
                } else if secret.len() == 3 {
                    cnt.push_str(&7.to_string())
                } else if secret.len() == 4 {
                    cnt.push_str(&4.to_string())
                } else if secret.len() == 7 {
                    cnt.push_str(&8.to_string())
                } else {
                    let s_a = secret
                        .chars()
                        .filter(|c| chars[1].contains(*c))
                        .collect::<Vec<_>>();

                    if s_a.len() > 1 {
                        if (secret.len() - chars[7].len()) == 2 {
                            cnt.push_str(&3.to_string());
                        } else {
                            let s_b: Vec<_> = chars[4]
                                .chars()
                                .filter(|c| !chars[1].contains(*c))
                                .collect();
                            let s_c = secret
                                .chars()
                                .filter(|c| s_b.contains(c))
                                .collect::<Vec<_>>();
                            if s_c.len() > 1 {
                                cnt.push_str(&9.to_string());
                            } else {
                                cnt.push_str(&0.to_string());
                            }
                        }
                    } else {
                        if secret.len() == 6 {
                            cnt.push_str(&6.to_string());
                        } else {
                            let s_b: Vec<_> = chars[4]
                                .chars()
                                .filter(|c| !chars[1].contains(*c))
                                .collect();
                            let s_c = secret
                                .chars()
                                .filter(|c| s_b.contains(c))
                                .collect::<Vec<_>>();
                            if s_c.len() > 1 {
                                cnt.push_str(&5.to_string());
                            } else {
                                cnt.push_str(&2.to_string());
                            }
                        }
                    }
                }
            }

            total += cnt.parse::<i32>().unwrap();
        });
    }

    let count = known_digits
        .iter()
        .enumerate()
        .filter(|(i, _)| *i == 2 || *i == 3 || *i == 4 || *i == 7)
        .map(|(_, &count)| count)
        .sum::<usize>();

    // 2, 3, 4, 7
    println!("{:?} {:?}", count, total);
    (count as f64, total as f64)
}
