
struct Sheep{}
struct Cow{}

trait Animal {
  fn noise(&self) -> &'static str;
}

// implement the animal trait for sheep
impl Animal for Sheep {
  fn noise(&self) -> &'static str {
    "baaaaaaaah!"
  }
}

// implement the animal trait for cow
impl Animal for Cow {
  fn noise(&self) -> &'static str {
    "mooooooooo!"
  }
}

// returns some struct that implements animal, but we dont know which one at compile time.
fn random_animal(random_number: f64) -> Box<dyn Animal> {
  if random_number < 0.5 {
    Box::new(Sheep {})
  } else {
    Box::new(Cow {})
  }
}

pub fn comm() {
  let random_number = 0.234;
  let animal = random_animal(random_number);
  println!("you randomly chaosen an animal, and it says {}", animal.noise());

}






