pub fn measure_increase(input: &[i32]) -> i32 {
  let output = input.iter().enumerate();
  output.fold(0, |accum, (index, item)| {
    if index == 0 {
      accum
    } else if &input[index - 1] < item {
      accum + 1
    } else {
      accum
    }
  })
}

#[test]
fn measure_increase_test() {
  let input = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
  assert_eq!(measure_increase(&input), 7);
}

#[test]
fn measure_increase_test_two() {
  let input = [199, 199, 208, 208, 200, 207, 240, 269, 260, 263];
  assert_eq!(measure_increase(&input), 5);
}
