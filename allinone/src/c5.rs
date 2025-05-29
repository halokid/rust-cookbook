trait Animal {
  fn make_sound(&self) -> String;
}

struct Dog;

struct Cat;

impl Animal for Dog {
  fn make_sound(&self) -> String {
    "Woof".to_string()
  }
}

impl Animal for Cat {
  fn make_sound(&self) -> String {
    "Meow".to_string()
  }
}

fn main() {
  let dog = Dog;
  let cat = Cat;

  // Create a vector of boxed trait objects
  let mut animals: Vec<Box<dyn Animal>> = Vec::new();
  // animals.push(Box::new(dog));
  // animals.push(Box::new(cat));
  //
  // for animal in animals.iter() {
  //   println!("{}", animal.make_sound());
  // }
}

