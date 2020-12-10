

#[derive(Debug)]
struct Foobar(i32);

impl Drop for Foobar {
  fn drop(&mut self) {
    println!("Dropping a Foobar: {:?}", self);
  }
}

fn uses_foobar(foobar: Foobar) {
  println!("I consumed a Foobar: {:?}", foobar);
}

pub fn comm() {
  let x = Foobar(1);
  println!("Before uses_foobar");
  uses_foobar(x);
  // uses_foobar(x);
  println!("After uses_foobar");
}