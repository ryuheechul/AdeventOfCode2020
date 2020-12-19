#[derive(Debug, PartialEq)]
struct PasswordPolicy {
  min: u8,
  max: u8,
  charactor: char,
}

#[derive(Debug, PartialEq)]
pub struct PasswordValidation {
  policy: PasswordPolicy,
  pw: String,
}

use std::convert::From;

impl PasswordPolicy {
  fn validate_for(&self, pw: &str) -> bool {
    let count = pw.chars().filter(|&c| c == self.charactor).count();

    let min: usize = self.min as usize;
    let max: usize = self.max as usize;

    min <= count && count <= max
  }

  fn validate_part2_for(&self, pw: &str) -> bool {
    let min: usize = self.min as usize - 1;
    let max: usize = self.max as usize - 1;

    let f = pw.chars().nth(min).unwrap();
    let s = pw.chars().nth(max).unwrap();

    let all: String = vec![f, s].iter().collect();
    let count = all.chars().filter(|&c| c == self.charactor).count();

    count == 1
  }
}

impl From<&str> for PasswordPolicy {
  fn from(policy_text: &str) -> Self {
    let tokens: Vec<&str> = policy_text.split(' ').collect(); // assume it's always one spacebar for now

    let range = tokens[0];
    let charactor = tokens[1].chars().next().unwrap();

    let tokens: Vec<&str> = range.split('-').collect(); // assume it's always one spacebar for now

    let min = tokens[0];
    let max = tokens[1];
    let min: u8 = min.parse().unwrap();
    let max: u8 = max.parse().unwrap();

    PasswordPolicy {
      min,
      max,
      charactor,
    }
  }
}
impl From<&str> for PasswordValidation {
  fn from(l: &str) -> Self {
    let tokens: Vec<&str> = l.split(':').collect(); // assume it's always one spacebar for now

    let p_token = tokens[0];
    let pw: String = tokens[1].trim().into();
    let policy = PasswordPolicy::from(p_token);

    PasswordValidation { policy, pw }
  }
}

impl PasswordValidation {
  pub fn is_valid(&self) -> bool {
    self.policy.validate_for(&self.pw)
  }

  pub fn is_valid_part2(&self) -> bool {
    self.policy.validate_part2_for(&self.pw)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  // 1-3 a: abcde
  #[test]
  fn first() {
    let policy = PasswordPolicy {
      min: 1,
      max: 3,
      charactor: 'a',
    };
    let validation = PasswordValidation {
      policy,
      pw: "abcde".into(),
    };
    let from = PasswordValidation::from("1-3 a: abcde");

    assert_eq!(from, validation);
    assert!(from.is_valid());
  }

  // 1-3 b: cdefg
  #[test]
  fn second() {
    let policy = PasswordPolicy {
      min: 1,
      max: 3,
      charactor: 'b',
    };
    let validation = PasswordValidation {
      policy,
      pw: "cdefg".into(),
    };
    let from = PasswordValidation::from("1-3 b: cdefg");

    assert_eq!(from, validation);
    assert!(!from.is_valid());
  }

  // 2-9 c: ccccccccc
  #[test]
  fn third() {
    let policy = PasswordPolicy {
      min: 2,
      max: 9,
      charactor: 'c',
    };
    let validation = PasswordValidation {
      policy,
      pw: "ccccccccc".into(),
    };
    let from = PasswordValidation::from("2-9 c: ccccccccc");

    assert_eq!(from, validation);
    assert!(from.is_valid());
  }
}
