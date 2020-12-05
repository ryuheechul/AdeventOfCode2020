mod sumer;

use sumer::Sumer;

fn main() -> Result<(), ()> {
  let input = read_from_file("./input.txt")?;
  let desired_sum = 2020;

  let mut s = Sumer::new(input.as_slice());

  if let Some(m) = s.find_product(desired_sum) {
    println!("the answer is {}!", m);
  } else {
    eprintln!("sorry there are no multiples found");
  }

  Ok(())
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
