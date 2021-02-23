/*
享元模式
 */

use std::collections::HashMap;

#[derive(Debug)]
struct Object(String, usize);

struct Flyweight {
  pool:       HashMap<String, Object>,
  counter:    usize,
}

impl Flyweight {
  fn new() -> Flyweight {
    Flyweight {
      pool:     HashMap::new(),
      counter:  0,
    }
  }

  fn obtain_object(&mut self, key: String) -> &mut Object {
    if self.pool.contains_key(&key) {
      return self.pool.get_mut(&key).unwrap()
    }

    self.pool.insert(key.clone(), Object(key.clone(), self.counter));
    self.counter += 1;
    self.obtain_object(key)
  }
}

fn main() {
  let mut flyweight = Flyweight::new();
  {
    let ol = flyweight.obtain_object(("halokid".to_string()));
    println!("{:?}", ol);
  }
}


