use crate::oop::circle::Circle;
use crate::oop::comm::HasArea;
use crate::oop::square::Square;

pub fn comm1() {
  let c = Circle{
    x: 0.0,
    y: 0.0,
    radius: 12.0
  };
  let carea = c.area();
  println!("carea: {}", carea);

  let s = Square {
    x: 0.0,
    y: 0.0,
    side: 9.0
  };
  let sarea = s.area();
  println!("sarea: {}", sarea);
}

