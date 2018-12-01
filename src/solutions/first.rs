use super::super::utils::file::reader;
use std::num::ParseIntError;

pub fn solve() {
  let changes_in_frequency: String = reader("src/solutions/data.txt");

  let changes: Vec<&str> = changes_in_frequency.split("\n").collect();

  let new_values: Vec<(&Fn(i64, i64) -> i64, Result<i64, ParseIntError>)> = changes
    .into_iter()
    .map(parse_calculation_and_number)
    .collect();

  let result = new_values.into_iter().fold(0, |acc: i64, v| {
    let (function, value) = v;
    match value {
      Ok(number) => function(acc, number),
      Err(e) => panic!("{}", e),
    }
  });

  println!("{}", result)
}

fn parse_calculation_and_number(s: &str) -> (&Fn(i64, i64) -> i64, Result<i64, ParseIntError>) {
  if s.starts_with("+") {
    let string_collection: Vec<&str> = s.split("+").collect();
    let number = string_collection[1].parse::<i64>();
    return (&add, number);
  } else {
    let string_collection: Vec<&str> = s.split("-").collect();
    let number = string_collection[1].parse::<i64>();
    return (&subtract, number);
  }
}

fn add(first: i64, second: i64) -> i64 {
  first + second
}

fn subtract(first: i64, second: i64) -> i64 {
  first - second
}
