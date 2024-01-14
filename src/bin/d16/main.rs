extern crate aoc2021;

use std::{collections::HashMap, env, isize, str};

use crate::utils::read_lines::read_lines;
use aoc2021::utils;

fn main() {
    let args: Vec<String> = env::args().collect();
    let src = &args[1];
    let (p1, p2) = d_16(src);

    println!("{}, {}", p1, p2);
}

fn d_16(source: &str) -> (u32, i32) {
    let byte_to_string_map: HashMap<char, &str> = HashMap::from([
        ('0', "0000"),
        ('1', "0001"),
        ('2', "0010"),
        ('3', "0011"),
        ('4', "0100"),
        ('5', "0101"),
        ('6', "0110"),
        ('7', "0111"),
        ('8', "1000"),
        ('9', "1001"),
        ('A', "1010"),
        ('B', "1011"),
        ('C', "1100"),
        ('D', "1101"),
        ('E', "1110"),
        ('F', "1111"),
    ]);

    let mut byte_string = "".to_string();

    if let Ok(lines) = read_lines(source) {
        lines.flatten().for_each(|line| {
            line.chars().for_each(|c| {
                byte_string.push_str(byte_to_string_map[&c]);
            });
        });
    }

    let (v_cnt, _) = make_packet(&byte_string);
    (v_cnt, 0)
}

fn make_packet(input: &str) -> (u32, &str) {
    let string_to_byte_map: HashMap<&str, char> = HashMap::from([
        ("0000", '0'),
        ("0001", '1'),
        ("0010", '2'),
        ("0011", '3'),
        ("0100", '4'),
        ("0101", '5'),
        ("0110", '6'),
        ("0111", '7'),
        ("1000", '8'),
        ("1001", '9'),
        ("1010", 'A'),
        ("1011", 'B'),
        ("1100", 'C'),
        ("1101", 'D'),
        ("1110", 'E'),
        ("1111", 'F'),
    ]);

    if input.len() < 11 {
        return (0, input);
    }

    let v: &str = &format!("{:0>4}", input.split_at(3).0).to_string();
    let version = string_to_byte_map[v].to_digit(10).unwrap();
    let t: &str = &format!("{:0>4}", input.split_at(3).1.split_at(3).0.to_string());
    let type_id = string_to_byte_map[t].to_digit(10).unwrap();
    let mut rest = input.split_at(6).1;

    let mut total_versions = version;

    if type_id != 4 {
        let lt_id = rest.split_at(1).0.parse::<i32>().unwrap();
        rest = rest.split_at(1).1;

        if lt_id == 1 {
            if rest.len() < 22 {
                return (total_versions, rest);
            }

            let (start, end) = rest.split_at(11);
            let pkg_no = isize::from_str_radix(start, 2).unwrap();
            rest = end;
            println!("11 BIT {} {} {} {}, ", version, type_id, pkg_no, start);

            for _ in 0..pkg_no {
                let (vss, r) = make_packet(rest);
                total_versions += vss;
                rest = r;
            }
        } else {
            if rest.len() < 26 {
                return (total_versions, rest);
            }

            let (start, end) = rest.split_at(15);
            let pkg_l = isize::from_str_radix(start, 2).unwrap();

            if pkg_l > end.len() as isize {
                return (total_versions, end);
            }

            println!("15 BIT {} {} {} {}, ", version, type_id, pkg_l, start);
            let mut sub = end.split_at(pkg_l as usize).0;

            while sub.is_empty() == false && sub.len() >= 11 {
                let (vss, s) = make_packet(sub);
                total_versions += vss;
                sub = s;
            }

            rest = end.split_at(pkg_l as usize).1;
        }
    } else {
        let r = rest
            .as_bytes()
            .chunks(5)
            .map(|chunk| std::str::from_utf8(chunk).map_err(|_| ()))
            .collect::<Result<Vec<&str>, _>>()
            .unwrap();

        let (_, _) = r
            .iter()
            .fold(("".to_string(), 0), |mut acc: (String, i32), s| {
                if acc.1 == 1 {
                    return acc;
                }

                if s.split_at(1).0 == "0" {
                    rest = rest.split_once(&s.to_string()).unwrap().1;
                    acc.1 = 1;
                }

                acc.0.push_str(&s.split_at(1).1.chars().collect::<String>());
                acc
            });
    }

    (total_versions, rest)
}
