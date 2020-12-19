mod password;

use password::PasswordValidation;

fn main() -> Result<(), ()> {
  let validations = parse_validation_from_file("./input.txt")?;

  let count = validations.iter().filter(|v| v.is_valid()).count();

  println!(
    "[part 1] the number of valid password from the input.txt is {}",
    count
  );

  let count = validations.iter().filter(|v| v.is_valid_part2()).count();

  println!(
    "[part 2] the number of valid password from the input.txt is {}",
    count
  );

  Ok(())
}

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// copied and pasted from https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
  P: AsRef<Path>,
{
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}

fn parse_validation(v: &[String]) -> Vec<PasswordValidation> {
  v.iter().map(|l| PasswordValidation::from(&l[..])).collect()
}

fn parse_validation_from_file(filename: &str) -> Result<Vec<PasswordValidation>, ()> {
  read_lines(filename)
    .map(|lines| {
      let lines: Vec<String> = lines.map(|l| l.unwrap()).collect();
      parse_validation(&lines)
    })
    .or(Err(()))
}
