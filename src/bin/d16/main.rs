extern crate aoc2021;

use std::{collections::HashMap, env, str};

use crate::utils::read_lines::read_lines;
use aoc2021::utils;

#[derive(Debug)]
struct Packet {
    version: u32,
    type_id: u32,
    groups: Vec<String>,
    value: isize,
    ptype: String,
    c_len: isize,
    c_b_len: isize,
}

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
        let _ = lines
            .flatten()
            .map(|line| {
                line.chars()
                    .map(|c| {
                        byte_string.push_str(byte_to_string_map[&c]);
                        byte_to_string_map[&c]
                    })
                    .collect::<Vec<&str>>()
            })
            .collect::<Vec<Vec<&str>>>();
    }
    let v_cnt = make_packet(&byte_string);
    (v_cnt, 0)
}

fn make_packet(input: &str) -> u32 {
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
        return 0;
    }

    let v: &str = &format!("{:0>4}", input.split_at(3).0).to_string();
    let version = string_to_byte_map[v].to_digit(10).unwrap();
    let t: &str = &format!("{:0>4}", input.split_at(3).1.split_at(3).0.to_string());
    let type_id = string_to_byte_map[t].to_digit(10).unwrap();
    let rest = input.split_at(3).1.split_at(3).1;

    let mut total_versions = version;
    let packet: Packet;

    println!("{}", input);

    // println!("Paket {} {} start", version, type_id);
    if type_id != 4 {
        let lt_id = rest.chars().nth(0).unwrap().to_string();
        let mut c_b_len = 0;
        let mut c_len = 0;

        if lt_id == "1" {
            let pkg_no = isize::from_str_radix(rest.split_at(1).1.split_at(11).0, 2).unwrap();
            let vss: u32 = make_packet(rest.split_at(1).1.split_at(11).1);

            c_len = pkg_no;
            total_versions += vss;
        } else {
            let pkg_l = isize::from_str_radix(rest.split_at(1).1.split_at(15).0, 2).unwrap();
            // println!("WW {} {} {} {} {} {} {}", rest, pkg_l, version, type_id, lt_id, rest.split_at(1).1.split_at(15).0,  rest.split_at(1).1.split_at(15).1.len());

            let m = rest.split_at(1).1.split_at(15).1;

            let mut vss: u32 = make_packet(m);
            //if m.1 != "" {
            //    vss += make_packet(rest.split_at(1).1.split_at(15).1.split_at(pkg_l as usize).1);
            //}

            c_b_len = pkg_l;
            total_versions += vss;
        }
        packet = Packet {
            version,
            type_id,
            groups: Vec::new(),
            value: 0,
            ptype: "Operator".to_string(),
            c_b_len,
            c_len,
        }
    } else {
        let r = rest
            .as_bytes()
            .chunks(5)
            .map(|chunk| std::str::from_utf8(chunk).map_err(|_| ()))
            .collect::<Result<Vec<&str>, _>>()
            .unwrap();

        let mut rest_rest = "".to_string();

        let (va, _, nr) = r.iter().fold(
            ("".to_string(), 0, vec![]),
            |mut acc: (String, i32, Vec<String>), s| {
                if s.split_at(1).1.len() < 3 || acc.1 == 1 {
                    return acc;
                }

                acc.0.push_str(&s.split_at(1).1.chars().collect::<String>());
                acc.2.push(s.split_at(1).1.chars().collect::<String>());

                if s.split_at(1).0 == "0" {
                    rest_rest = rest
                        .split_once(&s.split_at(1).1.to_string())
                        .unwrap()
                        .1
                        .to_string();
                    acc.1 = 1;
                }
                acc
            },
        );

        let val = isize::from_str_radix(&va, 2).unwrap();

        packet = Packet {
            version,
            type_id,
            groups: nr.clone(),
            value: val,
            ptype: "Literal".to_string(),
            c_len: nr.len() as isize,
            c_b_len: 0,
        };

        if rest_rest.len() > 6 {
            total_versions += make_packet(&rest_rest);
        }
    }

    println!("Created {:?}", packet);
    // println!("Packet {} {} End", version, type_id);

    total_versions
}
