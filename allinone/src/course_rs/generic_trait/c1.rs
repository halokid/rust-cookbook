// /*
fn add<T: std::ops::Add<Output=T>>(a: T, b: T) -> T {
  a + b
}
// */

/*
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> T {
  let mut largest = list[0];

  for &item in list.iter() {
    if item > largest {
      largest = item;
    }
  }
  largest
}
 */

struct Point<T> {
  x: T,
  y: T,
}

impl<T> Point<T> {
  fn x(&self) -> &T {
    &self.x
  }
}

impl Point<f32> {
  fn distance_from_origin(&self) -> f32 {
    (self.x.powi(2) + self.y.powi(2).sqrt())
  }
}

// ----------------------------------------
struct Pointx<T, U> {
  x: T,
  y: U,
}

impl<T, U> Pointx<T, U> {
  fn mixup<V, W>(self, other: Pointx<V, W>) -> Pointx<T, W> {
    Pointx {
      x: self.x,
      y: other.y,
    }
  }
}

// ----------------------------------------
enum Optionx<T> {
  Some(T),
  None,
}

enum Resultx<T, E> {
  Ok(T),
  Err(E),
}

// -----------------------------------------------
fn display_array(arr: &[i32]) {
  println!("{:?}", arr);
}

fn display_array_all<T: std::fmt::Debug>(arr: &[T]) {
  println!("{:?}", arr);
}

fn display_array_all_x<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
    println!("{:?}", arr);
}

pub fn comm() {
  // println!("add i8: {}", add(2i8, 3i8));
  // println!("add i32: {}", add(20i32, 30i32));
  // println!("add f64: {}", add(1.23, 2.67));

  let integer = Point {
    x: 5,
    y: 10,
  };
  let float = Point {
    x: 1.0,
    y: 4.0,
  };

  let p = Pointx {
    x: 1,
    y: 3.4,
  };

  // -----------------------------------------------
  let arr: [i32; 3] = [1, 2, 3];
  display_array(&arr);

  let arr_1: [&str; 3] = ["a", "b", "c"];
  display_array_all(&arr);
  display_array_all(&arr_1);

  // -----------------------------------------------
  // let arr: [i32; 3] = [1, 2, 3];
  display_array_all_x(arr);
  display_array_all_x(arr_1);
}





