extern crate aoc2021;
use crate::utils::read_lines::read_file;
use aoc2021::utils;
use std::collections::HashSet;
use std::env;
use std::{collections::HashMap, usize};

fn main() {
    let args: Vec<String> = env::args().collect();
    let src = &args[1];
    let one = d4_p1(src);
    let two = d4_p2(src);

    print!("Day 4");
    println!("{}", one);
    println!("{}", two);
}

trait Score {
    fn apply_move(&mut self, m: i32);
    fn check_win(&self) -> bool;
    fn check_score(&self) -> i32;
}

#[derive(Debug)]
struct Game {
    pub moves: Vec<i32>,
}

#[derive(Debug)]
struct Board {
    pub nums: HashMap<i32, (usize, usize, usize)>,
    pub row_marks: HashMap<i32, usize>,
    pub col_marks: HashMap<i32, usize>,
}

impl Score for Board {
    fn apply_move(&mut self, m: i32) {
        if let Some((i, j, _)) = self.nums.get_mut(&m) {
            *self.row_marks.entry(*i as i32).or_insert(0) += 1;
            *self.col_marks.entry(*j as i32).or_insert(0) += 1;
            self.nums.entry(m).and_modify(|e| {
                *e = (e.0, e.1, e.2 + 1);
            });
        }
    }
    fn check_win(&self) -> bool {
        self.row_marks.values().any(|val| *val >= 5) || self.col_marks.values().any(|val| *val >= 5)
    }
    fn check_score(&self) -> i32 {
        self.nums
            .iter()
            .filter(|(_, (_, _, count))| *count <= 0)
            .map(|(key, _)| *key)
            .sum()
    }
}

fn parse_input(source: &str) -> (Game, Vec<Board>) {
    let mut game: Game = Game { moves: Vec::new() };
    let file = read_file(source).unwrap();
    let file_split: Vec<_> = file.split_terminator("\n\n").collect();

    game.moves = file_split[0]
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let mut boards: Vec<Board> = Vec::with_capacity(file_split.len() - 1);
    for group in &file_split[1..] {
        let board = Board {
            col_marks: HashMap::new(),
            row_marks: HashMap::new(),
            nums: group
                .lines()
                .enumerate()
                .flat_map(|(i, entry)| {
                    entry.split_whitespace().enumerate().map(move |(j, se)| {
                        (
                            se.parse::<i32>().unwrap(),
                            (i, j, usize::try_from(0).unwrap()),
                        )
                    })
                })
                .collect::<HashMap<i32, (usize, usize, usize)>>(),
        };
        boards.push(board);
    }

    (game, boards)
}

#[allow(dead_code)]
pub fn d4_p1(source: &str) -> i32 {
    let (game, mut boards) = parse_input(source);

    let result = game.moves.iter().find_map(|m| {
        boards.iter_mut().find_map(|board| {
            board.apply_move(*m);
            if board.check_win() {
                Some(board.check_score() * *m)
            } else {
                None
            }
        })
    });

    result.unwrap_or(0)
}

#[test]
fn d4_p1_example() {
    assert_eq!(d4_p1(&"./src/bin/d4/e.txt"), 4512);
}

#[test]
fn d4_p1_test() {
    assert_eq!(
        d4_p1(&"./src/bin/d4/i.txt"),
        dotenv::var("d4_p1").unwrap().parse::<i32>().unwrap()
    )
}

#[allow(dead_code)]
pub fn d4_p2(source: &str) -> i32 {
    let (game, mut boards) = parse_input(source);
    let bl = boards.len();
    let mut result = None;
    let mut winners: HashSet<usize> = HashSet::new();

    for m in &game.moves {
        for (i, board) in boards.iter_mut().enumerate() {
            board.apply_move(*m);
            if board.check_win() && !winners.contains(&i) {
                let res = board.check_score() * *m;
                result = Some(res);
                winners.insert(i);
            }
            if winners.len() == bl {
                break;
            }
        }
    }

    result.unwrap_or(0)
}

#[test]
fn d4_p2_example() {
    assert_eq!(d4_p2(&"./src/bin/d4/e.txt"), 1924);
}

#[test]
fn d4_p2_test() {
    assert_eq!(
        d4_p2(&"./src/bin/d4/i.txt"),
        dotenv::var("d4_p2").unwrap().parse::<i32>().unwrap()
    )
}
