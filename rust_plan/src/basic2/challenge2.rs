use std::any::Any;
use crate::basic2::shape;
use crate::basic2::shape::Graphic;
use crate::basic2::challenge;

struct ShapeObject {
  name: String,
  graphic: Graphic,
}

impl shape::Shape for ShapeObject {
  fn area(&self) -> f64 {
    self.graphic.area()
  }

  fn draw(&self) {
    println!("Drawing shape '{}'", self.name);
    self.graphic.draw();
  }

  fn as_any(&self) -> &dyn Any {
    self
  }
}

impl ShapeObject {
  pub fn is_circle(&self) -> Option<f64> {
    if let Graphic::Circle(r) = self.graphic {
      Some(r)
    } else {
      None
    }
  }
}

fn parse_named_shape(input: &str) -> Option<ShapeObject> {
  let mut parts = input.split(':');
  let name = parts.next()?;
  let shape_str = parts.next()?;

  let graphic = challenge::parse_graphic(shape_str)?;
  Some(ShapeObject {
    name: name.to_string(),
    graphic,
  })
}

pub fn c1() {
  let inputs = vec![
    "circle1:circle,8",
    "rectA:rectangle,5,7",
    "tri01:triangle,6,4",
    "bad1:unknown,2",
  ];

  let shapes: Vec<Box<dyn shape::Shape>> = inputs
    .iter()
    .filter_map(|line| parse_named_shape(line))
    .map(|obj| Box::new(obj) as Box<dyn  shape::Shape>)
    .collect();

  for (i, shape) in shapes.iter().enumerate() {
    println!("Shape {}:", i + 1);
    shape.draw();
    println!("Area: {}\n", shape.area());
  }
}

