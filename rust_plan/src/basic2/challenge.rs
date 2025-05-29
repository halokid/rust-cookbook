use crate::basic2::shape;
use crate::basic2::shape::Graphic;

pub fn parse_graphic(input: &str) -> Option<shape::Graphic> {
  let parts: Vec<&str> = input.split(",").collect();

  match parts.as_slice() {
    ["circle", r] => {
      r.parse().ok().map(Graphic::Circle)
    }

    ["rectangle", w, h] => {
      if let (Ok(wx), Ok(hx)) = (w.parse(), h.parse()) {
        Some(Graphic::Rectangle(wx, hx))
      } else {
        None
      }
    }

    ["triangle", b, h] => {
      if let (Ok(bx), Ok(hx)) = (b.parse(), h.parse()) {
        Some(Graphic::Triangle(bx, hx))
      } else {
        None
      }
    }

    _ => None,
  }
}

pub fn c1() {
  let inputs = vec!["circle,8", "rectangle,5,7", "triangle,6,4", "unknown,2"];

  let shapes: Vec<Box<dyn shape::Shape>> = inputs
    .iter()
    .filter_map(|line| parse_graphic(line))
    .map(|g| Box::new(g) as Box<dyn shape::Shape>)
    .collect();

  for (i, shape) in shapes.iter().enumerate() {
    println!("Shape {}:", i + 1);
    shape.draw();
    println!("Area: {}", shape.area());
  }
}





