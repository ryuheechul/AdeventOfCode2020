#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Op {
  Nop,
  Acc,
  Jmp,
}

#[derive(Debug, Clone)]
pub struct Instruction {
  pub op: Op,
  pub operand: i16,
}

impl Instruction {
  pub fn values(&self) -> (Op, i16) {
    (self.op, self.operand)
  }
}

use std::convert::From;

impl From<&String> for Instruction {
  fn from(l: &String) -> Self {
    let tokens: Vec<&str> = l.split(' ').collect(); // assume it's always one spacebar for now

    let op = tokens[0];
    let operand = tokens[1];

    let sign = operand.chars().next().unwrap();

    let number: String = operand.chars().skip(1).collect();
    let number = number.parse::<i16>().unwrap();

    let operand = match sign {
      '+' => number,
      _ => number * -1, // assuming it's always '-'
    };

    let op = match op {
      "acc" => Op::Acc,
      "jmp" => Op::Jmp,
      _ => Op::Nop, // assuming it is always "nop"
    };

    Instruction { op, operand }
  }
}
