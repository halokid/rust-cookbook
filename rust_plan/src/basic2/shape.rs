use std::any::Any;

pub trait Shape: Any  {
  fn area(&self) -> f64;
  fn draw(&self);

  fn as_any(&self) -> &dyn Any;
}

pub enum Graphic {
  Circle(f64),
  Rectangle(f64, f64),
  Triangle(f64, f64),
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
      Graphic::Triangle(b, h) => {
        0.5 * b * h
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
      Graphic::Triangle(b, h) => {
        println!("Drawing a Triangle of base: {} and height: {}", b, h);
      }
    }
  }

  fn as_any(&self) -> &dyn Any {
    self
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
    Graphic::Triangle(_, _) => {}
  }
}

pub fn c2() {
  let shapes: Vec<Box<dyn Shape>> = vec![
    Box::new(Graphic::Circle(8.0)),
    Box::new(Graphic::Rectangle(5.0, 7.0)),
    Box::new(Graphic::Triangle(6.0, 4.0)),
  ];

  for (i, shape) in shapes.iter().enumerate() {
    println!("Shape {}:", i + 1);
    shape.draw();
    println!("Area: {}\n", shape.area());
  }
}










