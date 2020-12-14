mod sumer;

use sumer::Sumer;
use sumer::Sumer3;

fn main() -> Result<(), ()> {
  let input = read_from_file("./input.txt")?;
  let input = input.as_slice();
  let desired_sum = 2020;

  part1(input, desired_sum);
  part2(input, desired_sum);

  Ok(())
}

fn part1(input: &[u32], desired_sum: u32) {
  let mut s = Sumer::new(input);

  if let Some(m) = s.find_product(desired_sum) {
    println!("[part 1] the answer is {}!", m);
  } else {
    eprintln!("[part 1] sorry there are no multiples found");
  }
}

fn part2(input: &[u32], desired_sum: u32) {
  let mut s = Sumer3::new(input);

  if let Some(m) = s.find_product(desired_sum) {
    println!("[part 2] the answer is {}!", m);
  } else {
    eprintln!("[part 2] sorry there are no multiples found");
  }
}

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_from_file(filename: &str) -> Result<Vec<u32>, ()> {
  read_lines(filename)
    .map(|lines| {
      lines
        .map(|line| line.unwrap())
        .map(|line| line.parse::<u32>().unwrap())
        .collect()
    })
    .or(Err(()))
}

// copied and pasted from https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
  P: AsRef<Path>,
{
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}
