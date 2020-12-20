// inspired by this a lot https://github.com/csixteen/AdventOfCode/blob/master/2020/Rust/Day4/src/main.rs
use crate::field::{Byr, Ecl, Eyr, Hcl, Hgt, Iyr, Pid, Validate};

// byr (Birth Year)
// iyr (Issue Year)
// eyr (Expiration Year)
// hgt (Height)
// hcl (Hair Color)
// ecl (Eye Color)
// pid (Passport ID)
// cid (Country ID)

struct Part1 {
  byr: bool,
  iyr: bool,
  eyr: bool,
  hgt: bool,
  hcl: bool,
  ecl: bool,
  pid: bool,
}

struct Part2 {
  byr: Byr,
  iyr: Iyr,
  eyr: Eyr,
  hgt: Hgt,
  hcl: Hcl,
  ecl: Ecl,
  pid: Pid,
}

pub enum PartFlag {
  Flag1,
  Flag2,
}

enum Part {
  Part1(Part1),
  Part2(Part2),
}

pub struct Passport {
  part: Part,
}

impl Passport {
  pub fn new(line: &str, part: PartFlag) -> Self {
    let part = match part {
      PartFlag::Flag1 => Part::Part1(Part1 {
        byr: line.contains("byr:"),
        iyr: line.contains("iyr:"),
        eyr: line.contains("eyr:"),
        hgt: line.contains("hgt:"),
        hcl: line.contains("hcl:"),
        ecl: line.contains("ecl:"),
        pid: line.contains("pid:"),
      }),
      PartFlag::Flag2 => Part::Part2(Part2 {
        byr: Byr::new(line),
        iyr: Iyr::new(line),
        eyr: Eyr::new(line),
        hgt: Hgt::new(line),
        hcl: Hcl::new(line),
        ecl: Ecl::new(line),
        pid: Pid::new(line),
      }),
    };

    Passport { part }
  }

  pub fn is_valid(&self) -> bool {
    match &self.part {
      Part::Part1(p) => {
        let Part1 {
          byr,
          iyr,
          eyr,
          hgt,
          hcl,
          ecl,
          pid,
        } = *p;

        byr && iyr && eyr && hgt && hcl && ecl && pid
      }
      Part::Part2(p) => {
        let Part2 {
          ref byr,
          ref iyr,
          ref eyr,
          ref hgt,
          ref hcl,
          ref ecl,
          ref pid,
          ..
        } = *p;

        let all: Vec<Box<&dyn Validate>> = vec_box![byr, iyr, eyr, hgt, hcl, ecl, pid];
        all.iter().all(|v| v.is_valid())
      }
    }
  }
}
