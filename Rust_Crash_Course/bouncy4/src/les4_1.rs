
struct Fibs {
  x:    u32,
  y:    u32,
}

fn fibs() -> Fibs {
  Fibs {
    x:    0,
    y:    1,
  }
}

impl Iterator for Fibs {
  type Item = u32;

  // fn next(&mut self) -> Option<u32> {
  //   let orig_x = self.x;
  //   let orig_y = self.y;
  //
  //   self.x = orig_y;
  //   self.y = orig_x + orig_y;
  //
  //   Some(orig_x)
  // }

  fn next(&mut self) -> Option<u32> {
    let orig_x = self.x;
    let orig_y = self.y;

    match orig_x.checked_add(orig_y) {
      // overflow
      None => None,

      // not overflow
      Some(new_y) => {
        self.x = orig_y;
        self.y = new_y;

        Some(orig_x)
      }
    }
  }
}

pub fn comm() {
  for i in fibs().take(47) {
    println!("{}", i);
  }
}