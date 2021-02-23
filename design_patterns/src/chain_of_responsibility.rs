/*
责任链模式
 */


trait Cor {
  fn process_request(&self, &mut Request);
}

trait Request {
  fn get_level(&self) -> Level;
  fn get_something(&self) -> usize;
}

struct RequestX {
  level:    Level,
  v:        usize,
}

impl RequestX {
  fn new(l:  Level,  v: usize) -> RequestX {
    RequestX {
      level:    l,
      v:        v,
    }
  }
}

impl Request for RequestX {
  fn get_level(&self) -> Level {
    self.level
  }

  fn get_something(&self) -> usize {
    self.v
  }
}

#[derive(Debug, Copy, Clone, PartialEq)]

