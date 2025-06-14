use crate::basic2::shape;
use crate::basic2::shape::Graphic;
use crate::basic2::challenge2;
use std::any::Any;

pub fn c1() {
  let shapes: Vec<Box<dyn shape::Shape>> = vec![
    Box::new(Graphic::Circle(8.0)),
    Box::new(Graphic::Rectangle(5.0, 7.0)),
    Box::new(Graphic::Triangle(6.0, 4.0)),
  ];

  // let shapes: Vec<dyn shape::Shape> = vec![
  // let shapes: Vec<impl shape::Shape> = vec![
  //   Graphic::Circle(8.0),
  //   Graphic::Rectangle(5.0, 7.0),
  //   Graphic::Triangle(6.0, 4.0),
  // ];

  for shapex in shapes.iter() {
    if let Some(obj) = shapex.as_any().downcast_ref::<challenge2::ShapeObject>() {
      if let Some(radius) = obj.is_circle() {
        println!("Found Circle: name = {}, radius = {}", obj.name, radius);
      } else {
        println!("Not found Circle");
      }
    } else {
      println!("Not a ShapeObject");
    }
  }
}