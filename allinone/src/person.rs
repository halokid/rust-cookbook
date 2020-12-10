
struct Person {
  name:     String,
  age:      u32,
}

impl std::fmt::Display for Person {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
    write!(fmt, "{} {} years old", self.name, self.age)
  }
}

pub fn comm() {
  let alice = Person {
    name:       String::from("Alice"),
    age:        18,
  };
  println!("Person: {}", alice);
}
