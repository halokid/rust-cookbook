use std::ops::Add;

/*
struct Counter{}

impl Iterator for Couter {
  type Item = u32;

  fn next(&mut self) -> Option<Self::Item> {
    todo!()
  }
}
 */

// -----------------------------------------------
// trait Add<RHS=Self> {
//   type Output;
//
//   fn add(self, rhs: RHS) -> Self::Output;
// }

#[derive(Debug, PartialEq)]
struct Point {
  x:  i32,
  y:  i32,
}

impl Add for Point {
  type Output = Point;

  fn add(self, other: Point) -> Point {
    Point {
      x: self.x + other.x,
      y: self.y + other.y,
    }
  }
}

// -------------------------------------------
trait Pilot {
  fn fly(&self);
}

trait Wizard {
  fn fly(&self);
}

struct Human;

impl Human {
  fn fly(&self) {
    println!("*waving arms furiously*");
  }
}

impl Pilot for Human {
  fn fly(&self) {
    println!("this is your captain speaking");
  }
}

impl Wizard for Human {
  fn fly(&self) {
    println!("up!");
  }
}


pub fn comm() {
  let p1 = Point {
    x: 1,
    y: 0
  };

  let p2 = Point {
    x: 2,
    y: 3
  };

  let p3 = Point {
    x: 3,
    y: 3,
  };

  println!("equal -->>> {}", (p1 + p2 == p3));
  assert_eq!(Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
               Point { x: 3, y: 3 });

  // -----------------------------------------------------
  let person = Human;
  person.fly();

  let person = Human;
  Pilot::fly(&person);
  Wizard::fly(&person);
}







