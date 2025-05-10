
trait Shape {
  fn area(&self) -> f64;
  fn draw(&self);
}

enum Graphic {
  Circle(f64),
  Rectangle(f64, f64),
}

impl Shape for Graphic {
  fn area(&self) -> f64 {
    match self {
      Graphic::Circle(radius) => {
        std::f64::consts::PI * radius * radius
      }
      Graphic::Rectangle(w, h) => {
        w * h
      }
    }
  }

  fn draw(&self) {
    match self {
      Graphic::Circle(radius) => {
        println!("Drawing a circle with radius: {}", radius);
      }
      Graphic::Rectangle(w, h) => {
        println!("Drawing a rectangle of width: {} and height: {}", w, h);
      }
    }
  }
}

fn print_shape_info(shape: &impl Shape) {
  shape.draw();
  println!("Area is: {}\n", shape.area());
}

pub fn c1() {
  let s1 = Graphic::Circle(10.0);
  let s2 = Graphic::Rectangle(4.0, 6.0);

  print_shape_info(&s1);
  print_shape_info(&s2);

  match s1 {
    Graphic::Circle(r) if r > 5.0 => {
      println!("s1 is a Big circle");
    }
    Graphic::Circle(_) => {
      println!("s1 is a Small Circle");
    }
    Graphic::Rectangle(w, h) => {
      println!("s1 is a Rectangle: {} x {}", w, h);
    }
  }
}



