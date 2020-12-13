
struct OneToTen(u32);

fn one_to_ten() -> OneToTen {
  OneToTen(1)
}

impl Iterator for OneToTen {
  type Item = u32;

  fn next(&mut self) -> Option<u32> {
    if self.0 > 10 {
      None
    } else {
      let res = Some(self.0);
      self.0 += 1;
      res
    }
  }
}

pub fn comm() {
  for i in one_to_ten() {
    println!("i: {}", i);
  }

  for i in (1..11).map(|x| x * 2) {
    println!("i*2= {}", i);
  }
}

