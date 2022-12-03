

struct HasDrop1;
struct HasDrop2;

impl Drop for HasDrop1 {
  fn drop(&mut self) {
    println!("Dropping HasDrop1");
  }
}

impl Drop for HasDrop2 {
  fn drop(&mut self) {
    println!("Dropping HasDrop2");
  }
}

struct HasTwoDrops {
  one: HasDrop1,
  two: HasDrop2,
}

impl Drop for HasTwoDrops {
  fn drop(&mut self) {
    println!("Dropping HasTwoDrops!");
  }
}

struct Foo;

impl Drop for Foo {
  fn drop(&mut self) {
    println!("Dropping Foo!");
  }
}

// todo: Drop is in the stack, first in last out!!! so the result is:
// ------------------------
// Running!
// Dropping Foo!
// Dropping HasTwoDrops!
// Dropping HasDrop1
// Dropping HasDrop2
// ------------------------

#[derive(Debug)]
struct Foox;

impl Drop for Foox {
  fn drop(&mut self) {
    println!("Dropping Foox");
  }
}

pub fn comm() {
  let _x = HasTwoDrops {
    one: HasDrop1,
    two: HasDrop2
  };

  let _foo = Foo;
  println!("Running!");

  // -------------------------------------
  let mut foox = Foox;
  // foox.drop();
  println!("Running {:?}", foox);
  drop(foox);
}







