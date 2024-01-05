use crate::utils::read_lines::read_lines;

fn parse_input(source: &str) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines(source) {
        for line in lines.flatten() {
            if let Ok(l) = line.parse::<String>() {
                output.push(l);
            }
        }
    }
    output
}

fn count_bits(input: &Vec<String>) -> Vec<[i32; 2]> {
    let mut bit_count: Vec<[i32; 2]> = vec![[0, 0]; input[0].len()];

    for line in input {
        for (i, c) in line.chars().enumerate() {
            if c == '0' {
                bit_count[i][0] += 1;
            } else {
                bit_count[i][1] += 1;
            }
        }
    }
    bit_count
}

fn find_rating(input: &Vec<String>, current_column: usize, sel: char, exc: char) -> String {
    if input.len() == 1 {
        return input[0].to_string();
    }

    let bit_count = count_bits(input);
    let mut filtered_input: Vec<String> = Vec::new();

    for line in input {
        let char = line.chars().collect::<Vec<_>>()[current_column];
        let b_c = bit_count[current_column];
        if b_c[0] == b_c[1] && char == sel
            || b_c[0] > b_c[1] && char == exc
            || b_c[0] < b_c[1] && char == sel
        {
            filtered_input.push(line.clone())
        }
    }

    find_rating(&filtered_input, current_column + 1, sel, exc)
}

fn binary_to_decimal(bin: &String) -> isize {
    isize::from_str_radix(bin, 2).unwrap()
}

#[allow(dead_code)]
pub fn day3_p1(source: &str) -> isize {
    let output = parse_input(source);
    // rust 101: str are generally immutable while String represents mutable strings
    // in the output above it could make sense to have str but since we borrow
    // values that way, lifetime of variables gets in the way
    let mut gamma_rating: String = "".to_string();
    let mut epsilon_rating: String = "".to_string();

    let bit_count = count_bits(&output);

    for bit in bit_count.iter() {
        if bit[0] > bit[1] {
            gamma_rating.push('0');
            epsilon_rating.push('1');
        } else {
            gamma_rating.push('1');
            epsilon_rating.push('0');
        }
    }

    binary_to_decimal(&gamma_rating) * binary_to_decimal(&epsilon_rating)
}

#[test]
fn day3_p1_example() {
    assert_eq!(day3_p1(&"./src/day3/input_example.txt"), 198);
}

#[test]
fn day3_p1_test() {
    dotenv::dotenv().ok();
    assert_eq!(
        day3_p1(&"./src/day3/input.txt"),
        dotenv::var("d3_p1").unwrap().parse::<isize>().unwrap()
    );
}

#[allow(dead_code)]
pub fn day3_p2(source: &str) -> isize {
    let output = parse_input(source);
    let oxygen_generator_rating: String = find_rating(&output, 0, '1', '0');
    let co2_scrubber_rating: String = find_rating(&output, 0, '0', '1');
    binary_to_decimal(&oxygen_generator_rating) * binary_to_decimal(&co2_scrubber_rating)
}

#[test]
fn day3_p2_example() {
    assert_eq!(day3_p2(&"./src/day3/input_example.txt"), 230);
}

#[test]
fn day3_p2_test() {
    dotenv::dotenv().ok();
    assert_eq!(
        day3_p2(&"./src/day3/input.txt"),
        dotenv::var("d3_p2").unwrap().parse::<isize>().unwrap()
    );
}
