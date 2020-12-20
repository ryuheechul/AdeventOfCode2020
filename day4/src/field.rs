// inspired by this a lot https://github.com/csixteen/AdventOfCode/blob/master/2020/Rust/Day4/src/main.rs
use regex::Regex;
use std::str::FromStr;

#[derive(Copy, Clone)]
pub struct Byr {
  value: u16,
}

#[derive(Copy, Clone)]
pub struct Iyr {
  value: u16,
}

#[derive(Copy, Clone)]
pub struct Eyr {
  value: u16,
}

#[derive(Copy, Clone)]
enum Height {
  Cm(u16),
  In(u16),
}

#[derive(Copy, Clone)]
pub struct Hgt {
  value: Height,
}

pub struct Hcl {
  value: Option<String>,
}

pub struct Ecl {
  value: Option<String>,
}

pub struct Pid {
  value: Option<String>,
}

fn extract_number_from_regex(rule: &str, source: &str) -> u16 {
  let regex = Regex::new(rule).unwrap();

  regex
    .captures(source)
    .and_then(|c| {
      let n = c.get(1).unwrap().as_str();
      u16::from_str(n).ok()
    })
    .unwrap_or(0)
}

fn extract_str_from_regex(rule: &str, source: &str) -> Option<String> {
  let regex = Regex::new(rule).unwrap();

  regex
    .captures(source)
    .and_then(|c| c.get(1).map(|s| s.as_str().into()))
}

pub trait Validate {
  fn is_valid(&self) -> bool;
}

impl Validate for Byr {
  fn is_valid(&self) -> bool {
    self.value >= 1920 && self.value <= 2002
  }
}

impl Validate for Iyr {
  fn is_valid(&self) -> bool {
    self.value >= 2010 && self.value <= 2020
  }
}

impl Validate for Eyr {
  fn is_valid(&self) -> bool {
    self.value >= 2020 && self.value <= 2030
  }
}

impl Validate for Hgt {
  fn is_valid(&self) -> bool {
    match self.value {
      Height::In(n) => n >= 59 && n <= 76,
      Height::Cm(n) => n >= 150 && n <= 193,
    }
  }
}

impl Validate for Hcl {
  fn is_valid(&self) -> bool {
    self.value.is_some()
  }
}

impl Validate for Ecl {
  fn is_valid(&self) -> bool {
    self.value.is_some()
  }
}

impl Validate for Pid {
  fn is_valid(&self) -> bool {
    self.value.is_some()
  }
}

impl Byr {
  pub fn new(line: &str) -> Self {
    Byr {
      value: extract_number_from_regex(r"byr:([0-9]{4})[ |]", line),
    }
  }
}

impl Iyr {
  pub fn new(line: &str) -> Self {
    Iyr {
      value: extract_number_from_regex(r"iyr:([0-9]{4})[ |]", line),
    }
  }
}

impl Eyr {
  pub fn new(line: &str) -> Self {
    Eyr {
      value: extract_number_from_regex(r"eyr:([0-9]{4})[ |]", line),
    }
  }
}

fn extract_height_from_regex(rule: &str, source: &str) -> Height {
  let regex = Regex::new(rule).unwrap();

  // println!("{:?}", regex);

  let cap = regex.captures(source);

  // println!("{:?}, {:?}", cap, source);

  // regex
  //   .captures(source)
  cap
    .map(|c| {
      let n = c.get(1).unwrap().as_str();
      let n = u16::from_str(n).unwrap_or(0);

      match c.get(2).unwrap().as_str() {
        "in" => Height::In(n),
        _ => Height::Cm(n),
      }
    })
    .unwrap_or(Height::In(0))
}

impl Hgt {
  pub fn new(line: &str) -> Self {
    Hgt {
      value: extract_height_from_regex(r"hgt:([0-9]+)(in|cm)[ |]", line),
    }
  }
}

impl Hcl {
  pub fn new(line: &str) -> Self {
    Hcl {
      value: extract_str_from_regex(r"hcl:(#[0-9a-f]{6})[ |]", line),
    }
  }
}

impl Ecl {
  pub fn new(line: &str) -> Self {
    Ecl {
      value: extract_str_from_regex(r"ecl:(amb|blu|brn|gry|grn|hzl|oth)[ |]", line),
    }
  }
}

impl Pid {
  pub fn new(line: &str) -> Self {
    Pid {
      value: extract_str_from_regex(r"pid:([0-9]{9})[ |]", line),
    }
  }
}
