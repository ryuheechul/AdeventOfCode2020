#[macro_use]
extern crate vec_box;

mod field;
mod passport;

use passport::{PartFlag, Passport};

fn main() -> Result<(), ()> {
  let (p1, p2) = parse_passports_from_file("./input.txt")?;

  passport_count(p1, 1);
  passport_count(p2, 2);

  Ok(())
}

fn passport_count(passports: Vec<Passport>, n: u8) {
  let count = passports.iter().filter(|p| p.is_valid()).count();

  println!("[part {}] the number of valid passports is {}", n, count);
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

fn parse_passport(v: &[String]) -> (Vec<Passport>, Vec<Passport>) {
  let mut passport_line = "".to_string();
  let mut passports_part1: Vec<Passport> = vec![];
  let mut passports_part2: Vec<Passport> = vec![];

  for line in v {
    match line.len() {
      0 => {
        passports_part1.push(Passport::new(&passport_line, PartFlag::Flag1));
        passports_part2.push(Passport::new(&passport_line, PartFlag::Flag2));
        passport_line = "".into();
      }
      _ => {
        passport_line.push_str(&line);
        passport_line.push_str(" ");
      }
    }
  }

  passports_part1.push(Passport::new(&passport_line, PartFlag::Flag1));
  passports_part2.push(Passport::new(&passport_line, PartFlag::Flag2));

  (passports_part1, passports_part2)
}

fn parse_passports_from_file(filename: &str) -> Result<(Vec<Passport>, Vec<Passport>), ()> {
  read_lines(filename)
    .map(|lines| {
      let lines: Vec<String> = lines.map(|l| l.unwrap()).collect();
      parse_passport(&lines)
    })
    .or(Err(()))
}
