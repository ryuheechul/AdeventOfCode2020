//TODO: use input file

fn main() {
  let dum = dummy();
  let insts = parse_instructions(dum);

  match acc_just_before_limbo(vec![], 0, 0, insts) {
    Some(acc) => println!("Immediately before the program would run an instruction a second time, the value in the accumulator is {:?}", acc),
    None => println!("I think pointer reached the end of the instructions or something went wrong."),
  }
}

fn acc_just_before_limbo(
  been_there: Vec<u8>,
  acc: i16,
  pointer: isize,
  insts: Vec<Instruction>,
) -> Option<i16> {
  if pointer < 0 {
    return None;
  }

  let pointer: usize = pointer as usize;

  let (op, operand) = (&insts[pointer]).values();

  let (a, p): (i16, isize) = match op {
    Op::Nop => (acc, pointer as isize + 1),
    Op::Acc => (acc + operand, pointer as isize + 1),
    Op::Jmp => (acc, pointer as isize + operand as isize),
  };

  if p >= insts.len() as isize - 1 || p < 0 {
    return None;
  }

  let pointer: u8 = p as u8;

  if been_there.contains(&pointer) {
    Some(acc)
  } else {
    let been_there = [been_there, vec![pointer]].concat();
    acc_just_before_limbo(been_there, a, p, insts)
  }
}

#[derive(Debug, Copy, Clone)]
enum Op {
  Nop,
  Acc,
  Jmp,
}

#[derive(Debug)]
struct Instruction {
  op: Op,
  operand: i16,
}

impl Instruction {
  fn values(&self) -> (Op, i16) {
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

fn parse_instructions(v: Vec<String>) -> Vec<Instruction> {
  v.iter().map(Instruction::from).collect()
}

fn dummy() -> Vec<String> {
  vec![
    "nop +0", "acc +1", "jmp +4", "acc +3", "jmp -3", "acc -99", "acc +1", "jmp -4", "acc +6",
  ]
  .iter()
  .map(|&s| s.into())
  .collect()
}
