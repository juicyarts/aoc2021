extern crate aoc2021;
use std::{cmp::Ordering, collections::BinaryHeap, env, i32};

use crate::utils::read_lines::read_lines;
use aoc2021::utils;

const DIRS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn main() {
    let args: Vec<String> = env::args().collect();
    let src = &args[1];

    let (p1, p2) = d_15(src);

    println!("{}, {}", p1, p2);
}

fn d_15(source: &str) -> (i32, i32) {
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
        [map.len() as i32 - 1, map[0].len() as i32 - 1],
        &map,
    );

    (c, 0)
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct State {
    risk: i32,
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

fn shortest_path(start: [i32; 2], end: [i32; 2], m: &Vec<Vec<u32>>) -> i32 {
    let mut heap = BinaryHeap::new();
    heap.push(State {
        risk: 0,
        position: start,
    });

    let mut visited = vec![vec![0; m[0].len()]; m.len()];

    while let Some(State { risk, position }) = heap.pop() {
        if position == end {
            return risk;
        }

        if visited[position[0] as usize][position[1] as usize] > 0 {
            continue;
        }

        visited[position[0] as usize][position[1] as usize] += risk;

        for dir in DIRS {
            let next: [i32; 2] = [
                (position[0] as isize + dir.0) as i32,
                (position[1] as isize + dir.1) as i32,
            ];

            let isoob = next[0] < 0
                || next[0] as isize >= (m.len() as isize)
                || next[1] < 0
                || next[1] as isize >= (m[next[0] as usize].len() as isize);

            if !isoob {
                let c = m[next[0] as usize][next[1] as usize];
                let n = State {
                    position: next,
                    risk: risk + c as i32,
                };
                heap.push(n);
            }
        }
    }

    0
}
