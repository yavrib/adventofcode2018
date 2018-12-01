use std::fs::File;
use std::io::prelude::*;

pub mod solution;

fn reader(path: &str) -> String {
  let mut file = File::open(path).expect("file not found");

  let mut contents = String::new();

  file.read_to_string(&mut contents).expect("could not read");

  contents
}

pub fn solve() {
  let changes_in_frequency = reader("first_problem/data");

  changes_in_frequency.split("\n");

  println!("{:?}", changes_in_frequency)
}
