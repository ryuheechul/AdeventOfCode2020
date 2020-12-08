pub struct Sumer {
  head: usize,
  tail: usize,
  list: Vec<u32>,
}

impl Sumer {
  pub fn new(list: &[u32]) -> Self {
    let mut l: Vec<_> = list.iter().copied().collect();
    let tail = list.len() - 1;
    l.sort();

    Sumer {
      head: 0,
      tail,
      list: l,
    }
  }

  fn advance_head(&mut self) {
    self.head += 1;
  }

  fn advance_tail(&mut self) {
    self.tail -= 1;
  }

  fn sum(&self) -> u32 {
    let h = self.value_at(self.head);
    let t = self.value_at(self.tail);

    h + t
  }

  fn product(&self) -> u32 {
    let h = self.value_at(self.head);
    let t = self.value_at(self.tail);

    h * t
  }

  fn value_at(&self, idx: usize) -> u32 {
    self.list[idx]
  }

  fn check_if_possible_to_advance(&self) -> bool {
    if self.head >= self.tail {
      false
    } else {
      true
    }
  }

  pub fn find_product(&mut self, target_sum: u32) -> Option<u32> {
    let sum_of_h_t = self.sum();

    if sum_of_h_t == target_sum {
      return Some(self.product());
    }

    if !self.check_if_possible_to_advance() {
      return None;
    }

    if sum_of_h_t > target_sum {
      self.advance_tail();
    } else {
      self.advance_head();
    }

    self.find_product(target_sum)
  }
}

pub struct Sumer3 {
  list: Vec<u32>,
}

impl Sumer3 {
  pub fn new(list: &[u32]) -> Self {
    let mut l: Vec<_> = list.iter().copied().collect();
    l.sort();

    Sumer3 { list: l }
  }

  pub fn find_product(&mut self, target_sum: u32) -> Option<u32> {
    for n in &self.list {
      let mut s = Sumer::new(&self.list[..]);

      if let Some(m) = s.find_product(target_sum - n) {
        return Some(m * n);
      }
    }

    None
  }
}
