pub fn dive(input: Vec<(&str, i32)>) -> i32 {
  let (pz, px) = input.iter().fold(
    (0, 0),
    |(pos_z, pos_x), (direction, amount)| match direction {
      &"forward" => (pos_z, pos_x + amount),
      &"down" => (pos_z + amount, pos_x),
      &"up" => (pos_z - amount, pos_x),
      _ => (pos_z, pos_x),
    },
  );
  pz * px
}

#[test]
fn dive_test() {
  let input = vec![
    ("forward", 5),
    ("down", 5),
    ("forward", 8),
    ("up", 3),
    ("down", 8),
    ("forward", 2),
  ];

  assert_eq!(dive(input), 150);
}
