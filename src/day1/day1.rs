use crate::utils::read_lines::read_lines;

#[allow(dead_code)]
pub fn day1(source: &str) -> i32 {
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

// #[test]
// fn day1_example() {
//     assert_eq!(day1(&"./src/day1/input_example.txt"), 7);
// }

// #[test]
// fn day1_p1() {
//     dotenv::dotenv().ok();
//     assert_eq!(
//         day1(&"./src/day1/input.txt"),
//         dotenv::var("day1_part2").unwrap().parse::<i32>().unwrap()
//     );
// }

#[test]
fn day1_p2_example() {
    assert_eq!(day1(&"./src/day1/input_example.txt"), 5);
}

#[test]
fn day1_p2() {
    assert_eq!(
        day1(&"./src/day1/input.txt"),
        dotenv::var("day1_part2").unwrap().parse::<i32>().unwrap()
    )
}
