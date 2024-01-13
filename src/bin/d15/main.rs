extern crate aoc2021;
use std::{cmp::Ordering, collections::BinaryHeap, env, i32};

use crate::utils::read_lines::read_lines;
use aoc2021::utils;

const DIRS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct State {
    risk: u32,
    position: [i32; 2],
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .risk
            .cmp(&self.risk)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let src = &args[1];
    let s = &args[2];
    let size = s.parse::<i32>().unwrap();
    let (p1, p2) = d_15(src, &size);

    println!("{}, {}", p1, p2);
}

fn d_15(source: &str, size: &i32) -> (u32, i32) {
    let mut map: Vec<Vec<u32>> = Vec::new();
    if let Ok(lines) = read_lines(source) {
        map = lines
            .flatten()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect::<Vec<u32>>()
            })
            .collect::<Vec<Vec<u32>>>();
    }

    let c = shortest_path(
        [0, 0],
        [
            ((map.len() as i32) * size) - 1,
            ((map[0].len() as i32) * size) - 1,
        ],
        &map,
        &size,
    ) - (map[0][0] as u32);

    (c, 0)
}

const MAXRISK: u32 = 9;

fn shortest_path(start: [i32; 2], end: [i32; 2], m: &Vec<Vec<u32>>, size: &i32) -> u32 {
    let mut heap = BinaryHeap::new();
    let mut visited = vec![vec![0; m[0].len() * *size as usize]; m.len() * *size as usize];

    heap.push(State {
        risk: 0,
        position: start,
    });

    while let Some(State { risk, position }) = heap.pop() {
        let mut norm_pos = position;
        let mut offset = 0;

        norm_pos[0] = (position[0] as usize % m.len()) as i32;
        offset += position[0].div_euclid(m.len() as i32);
        norm_pos[1] = (position[1] as usize % m[0].len()) as i32;
        offset += position[1].div_euclid(m[0].len() as i32);

        let c = m[norm_pos[0] as usize][norm_pos[1] as usize] + (offset as u32);
        let mut r = risk;

        if c > MAXRISK {
            r += c % MAXRISK;
        } else {
            r += c;
        }

        if visited[position[0] as usize][position[1] as usize] > 0 {
            continue;
        }

        visited[position[0] as usize][position[1] as usize] += r;

        if position == end {
            return r;
        }

        for dir in DIRS {
            let next = [
                (position[0] as isize + dir.0) as i32,
                (position[1] as isize + dir.1) as i32,
            ];

            let isoob = next[0] < 0
                || next[0] as isize > ((m.len() * *size as usize) - 1) as isize
                || next[1] < 0
                || next[1] as isize > ((m[0].len() * *size as usize) - 1) as isize;

            if !isoob {
                heap.push(State {
                    position: next,
                    risk: r,
                });
            }
        }
    }

    0
}
