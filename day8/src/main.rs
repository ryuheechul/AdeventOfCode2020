fn main() -> Result<(), ()> {
  let insts = parse_instructions_from_file("./input.txt")?;

  match acc_just_before_limbo(vec![], 0, 0, &insts) {
    Some(acc) => println!("Immediately before the program would run an instruction a second time, the value in the accumulator is {:?}", acc),
    None => println!("I think pointer reached the end of the instructions or something went wrong."),
  }

  Ok(())
}

fn acc_just_before_limbo(
  been_there: Vec<u16>,
  acc: i16,
  pointer: isize,
  insts: &[Instruction],
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

  let pointer: u16 = p as u16;

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

fn parse_instructions(v: &[String]) -> Vec<Instruction> {
  v.iter().map(Instruction::from).collect()
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

fn parse_instructions_from_file(filename: &str) -> Result<Vec<Instruction>, ()> {
  read_lines(filename).map(|lines| {
    let lines: Vec<String> = lines.map(|l| l.unwrap()).collect();
    parse_instructions(&lines)
  })
    .or(Err(()))
}
