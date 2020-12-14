mod inst;

use inst::{Instruction, Op};
use std::mem;

fn main() -> Result<(), ()> {
  let insts = parse_instructions_from_file("./input.txt")?;

  part1(&insts);
  part2(&insts);

  Ok(())
}

fn part1(insts: &[Instruction]) {
  match calc_acc(vec![], 0, 0, insts) {
    Acc::JustBeforeLimbo(acc) => println!("[part1 - right] Immediately before the program would run an instruction a second time, the value in the accumulator is {:?}", acc),
    Acc::AfterDone(acc) => println!("[part1 - wrong] ran all the instructions, the value in the accumulator is {:?}", acc),
    Acc::WentWrong => println!("[part1 - wrong] I think pointer reached the end of the instructions or something went wrong."),
  };
}

fn part2(insts: &[Instruction]) {
  for (idx, inst) in insts.iter().enumerate() {
    let to_swap = match inst.op {
      Op::Nop => Op::Jmp,
      Op::Jmp => Op::Nop,
      Op::Acc => Op::Acc,
    };

    if to_swap == Op::Acc {
      continue;
    }

    let modified = replace_instruction(
      insts,
      idx,
      Instruction {
        op: to_swap,
        operand: inst.operand,
      },
    );

    match calc_acc(vec![], 0, 0, &modified) {
      Acc::JustBeforeLimbo(acc) => println!("[part2 - wrong] Immediately before the program would run an instruction a second time, the value in the accumulator is {:?}", acc),
      Acc::AfterDone(acc) => {
        println!("[part2 - right] ran all the instructions, the value in the accumulator is {:?}", acc);
        return;
      }
      Acc::WentWrong => println!("[part2 - wrong] I think pointer reached the end of the instructions or something went wrong."),
    };
  }
}

fn replace_instruction(insts: &[Instruction], index: usize, inst: Instruction) -> Vec<Instruction> {
  let mut copy = insts.to_vec();
  let item = &mut copy[index];

  let _ = mem::replace(item, inst);

  copy
}

enum Acc {
  JustBeforeLimbo(i16),
  AfterDone(i16),
  WentWrong,
}

fn calc_acc(been_there: Vec<u16>, acc: i16, pointer: isize, insts: &[Instruction]) -> Acc {
  if pointer < 0 {
    return Acc::WentWrong;
  }

  let pointer: usize = pointer as usize;

  let (op, operand) = (&insts[pointer]).values();

  let (a, p): (i16, isize) = match op {
    Op::Nop => (acc, pointer as isize + 1),
    Op::Acc => (acc + operand, pointer as isize + 1),
    Op::Jmp => (acc, pointer as isize + operand as isize),
  };

  if p < 0 {
    return Acc::WentWrong;
  }

  if p >= insts.len() as isize - 1 {
    return Acc::AfterDone(a as i16);
  }

  let pointer: u16 = p as u16;

  if been_there.contains(&pointer) {
    Acc::JustBeforeLimbo(a as i16)
  } else {
    let been_there = [been_there, vec![pointer]].concat();
    calc_acc(been_there, a, p, insts)
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
  read_lines(filename)
    .map(|lines| {
      let lines: Vec<String> = lines.map(|l| l.unwrap()).collect();
      parse_instructions(&lines)
    })
    .or(Err(()))
}
