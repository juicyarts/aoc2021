use crate::utils::read_lines::read_lines;
use std::cmp;

#[derive(Debug)]
struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug)]
struct Line {
    pub start: Point,
    pub end: Point,
}

fn parse_input(source: &str) -> (Vec<Line>, i32, i32) {
    let mut output: Vec<Line> = Vec::new();
    let mut max_x = 0;
    let mut max_y = 0;
    if let Ok(lines) = read_lines(source) {
        for line in lines.flatten() {
            if let Ok(l) = line.parse::<String>() {
                let root: Vec<_> = l.split(" -> ").collect();
                let a: Vec<_> = root[0].split(",").collect();
                let (x1, y1) = (a[0].parse::<i32>().unwrap(), a[1].parse::<i32>().unwrap());
                let b: Vec<_> = root[1].split(",").collect();
                let (x2, y2) = (b[0].parse::<i32>().unwrap(), b[1].parse::<i32>().unwrap());

                max_x = cmp::max(max_x, cmp::max(x1, x2));
                max_y = cmp::max(max_y, cmp::max(y1, y2));

                output.push(Line {
                    start: Point {
                        x: x1,
                        y: y1,
                    },
                    end: Point {
                        x: x2,
                        y: y2,
                    },
                });
            }
        }
    }
    (output, max_x, max_y)
}
#[allow(dead_code)]
pub fn day5_p1(source: &str) -> i32 {
    let (lines, max_x, max_y) = parse_input(source);
    print_map(&lines, max_x, max_y);

    for line in &lines {
        println!("{:?}", line)
    }
    println!("\n");
    0
}

fn print_map(lines: &Vec<Line>, max_x: i32, max_y: i32) {
    println!("Printing Map: {:?}*{:?} \n", max_x, max_y);
    for x in 0..=max_x {
        print!("\n");
        for y in 0..=max_y {
            let l = lines.iter().find(|line| {
                (line.start.x == x && line.start.y == y) || (line.end.x == x && line.end.y == y)
            });

            match l {
                Some(_) => print!("X"),
                None => print!("."),
            }
        }
    }
    println!("\n");
}

#[test]
pub fn day5_p1_example() {
    assert_eq!(day5_p1(&"./src/day5/input_example.txt"), 4512);
}
