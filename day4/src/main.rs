mod passport;

use passport::Passport;

fn main() -> Result<(), ()> {
  let passports = parse_passports_from_file("./input.txt")?;

  part1(passports);
  Ok(())
}

fn part1(passports: Vec<Passport>) {
  let count = passports.iter().filter(|p| p.is_valid()).count();

  println!("[part 1] the number of valid passports is {}", count);
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

fn parse_passport(v: &[String]) -> Vec<Passport> {
  let mut passport_line = "".to_string();
  let mut passports: Vec<Passport> = vec![];

  for line in v {
    match line.len() {
      0 => {
        passports.push(Passport::new(&passport_line));
        passport_line = "".into();
      }
      _ => {
        passport_line.push_str(&line);
        passport_line.push_str(" ");
      }
    }
  }

  passports.push(Passport::new(&passport_line));

  passports
}

fn parse_passports_from_file(filename: &str) -> Result<Vec<Passport>, ()> {
  read_lines(filename)
    .map(|lines| {
      let lines: Vec<String> = lines.map(|l| l.unwrap()).collect();
      parse_passport(&lines)
    })
    .or(Err(()))
}
