pub struct Passport {
  byr: bool,
  iyr: bool,
  eyr: bool,
  hgt: bool,
  hcl: bool,
  ecl: bool,
  pid: bool,
  cid: bool,
}

impl Passport {
  pub fn new(line: &str) -> Self {
    // byr (Birth Year)
    // iyr (Issue Year)
    // eyr (Expiration Year)
    // hgt (Height)
    // hcl (Hair Color)
    // ecl (Eye Color)
    // pid (Passport ID)
    // cid (Country ID)
    Passport {
      byr: line.contains("byr:"),
      iyr: line.contains("iyr:"),
      eyr: line.contains("eyr:"),
      hgt: line.contains("hgt:"),
      hcl: line.contains("hcl:"),
      ecl: line.contains("ecl:"),
      pid: line.contains("pid:"),
      cid: line.contains("cid:"),
    }
  }
  pub fn is_valid(&self) -> bool {
    let Passport {
      byr,
      iyr,
      eyr,
      hgt,
      hcl,
      ecl,
      pid,
      cid: _cid,
    } = *self;

    byr && iyr && eyr && hgt && hcl && ecl && pid
  }
}
