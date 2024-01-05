use crate::utils::read_lines::read_lines;

fn parse_input(source: &str) -> Vec<(String, i32)> {
    let mut output: Vec<(String, i32)> = Vec::new();

    if let Ok(lines) = read_lines(source) {
        for line in lines.flatten() {
            let v: Vec<String> = line.split(" ").map(|s| s.to_string()).collect();
            let num: i32 = v[1].parse::<i32>().unwrap();
            let direction = v[0].clone();
            output.push((direction, num));
        }
    }

    output
}

#[allow(dead_code)]
pub fn day2_p1(source: &str) -> i32 {
    let output = parse_input(source);

    let (pz, px) = output.iter().fold(
        (0, 0),
        |(pos_z, pos_x), (direction, amount)| match direction.as_str() {
            "forward" => (pos_z, pos_x + amount),
            "down" => (pos_z + amount, pos_x),
            "up" => (pos_z - amount, pos_x),
            _ => (pos_z, pos_x),
        },
    );
    pz * px
}

#[test]
fn day2_p1_example() {
    assert_eq!(day2_p1(&"./src/day2/input_example.txt"), 150);
}

#[test]
fn day2_p1_test() {
    dotenv::dotenv().ok();
    assert_eq!(
        day2_p1(&"./src/day2/input.txt"),
        dotenv::var("d2_p1").unwrap().parse::<i32>().unwrap()
    );
}

#[allow(dead_code)]
pub fn day2_p2(source: &str) -> i32 {
    let output = parse_input(source);

    let (pz, px, _aim) =
        output.iter().fold(
            (0, 0, 0),
            |(pos_z, pos_x, a), (direction, amount)| match direction.as_str() {
                "forward" => (pos_z + (a * amount), pos_x + amount, a),
                "down" => (pos_z, pos_x, a + amount),
                "up" => (pos_z, pos_x, a - amount),
                _ => (pos_z, pos_x, a),
            },
        );
    pz * px
}

#[test]
fn day2_p2_example() {
    assert_eq!(day2_p2(&"./src/day2/input_example.txt"), 900);
}

#[test]
fn day2_p2_test() {
    dotenv::dotenv().ok();
    assert_eq!(
        day2_p2(&"./src/day2/input.txt"),
        dotenv::var("d2_p2").unwrap().parse::<i32>().unwrap()
    );
}
