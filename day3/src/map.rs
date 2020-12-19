use std::ops::Add;

#[derive(Copy, Clone)]
pub struct Slope {
  h: usize,
  v: usize,
}

impl Slope {
  pub fn new(h: usize, v: usize) -> Self {
    Slope { h, v }
  }
}

impl Add for Slope {
  type Output = Slope;

  fn add(self, other: Slope) -> Slope {
    Slope {
      v: self.v + other.v,
      h: self.h + other.h,
    }
  }
}

pub struct Row {
  line: String,
}

impl From<&str> for Row {
  fn from(l: &str) -> Self {
    Row { line: l.into() }
  }
}

impl Row {
  fn step(&self, index: usize) -> char {
    let len = self.line.len();
    let mut index = index;
    while index >= len {
      index -= len;
    }

    self.line.chars().nth(index).unwrap()
  }
}

pub struct Map {
  curr: Slope,
  next: Slope,
  rows: Vec<Row>,
  slope: Slope,
}

impl Iterator for Map {
  type Item = char;

  fn next(&mut self) -> Option<char> {
    self.curr = self.curr + self.slope;

    let Slope { h, v } = self.curr;
    let v = v;

    let rows = &self.rows;

    if v >= rows.len() {
      return None;
    }

    let row = &rows[v];
    let ch = row.step(h);
    Some(ch)
  }
}

impl Map {
  pub fn new(rows: Vec<Row>) -> Self {
    let unit = Slope { v: 0, h: 0 };

    Map {
      curr: unit,
      next: unit,
      rows,
      slope: unit,
    }
  }

  pub fn count_trees(&mut self, s: Slope) -> usize {
    self.init(s);

    self.filter(|&c| c == '#').count()
  }

  fn init(&mut self, s: Slope) {
    self.slope = s;
    self.curr = Slope { v: 0, h: 0 };
    self.next = Slope { v: 0, h: 0 };
  }
}
