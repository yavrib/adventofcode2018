use super::super::utils::file::reader;

pub fn solve() {
  let changes_in_frequency: String = reader("src/solutions/data.txt");

  let changes: Vec<&str> = changes_in_frequency.split("\n").collect();

  println!("{:?}", changes)
}
