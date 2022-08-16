pub fn comm() {
  let x = 1;

  match x {
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    _ => println!("anything"),
  }

  // --------------------------------
  let x = Some(5);
  let y = 10;

  match x {
    Some(50) => println!("Got 50"),
    Some(y) => println!("Matched, y = {}", y),
    _ => println!("Default case, x = {:?}", x),
  }

  println!("at the end: x = {:?}, y = {:?}", x, y);

  let x = 'c';

  match x {
    'a'..='j' => println!("early ASCII letter"),
    'k'..='z' => println!("late ASCII letter"),
    _ => println!("something else"),
  }

  let x = 5;

  match x {
    1..=5 => println!("one through five"),
    _ => println!("something else"),
  }

  let x = 1;

  match x {
    1 | 2 => println!("one or two"),
    3 => println!("three"),
    _ => println!("anything"),
  }

  // -------------------------------------------
  let p = Point {
    x: 0,
    y: 7,
  };

  let Point { x: a, y: b } = p;
  println!("a -->>> {}", a);
  println!("b -->>> {}", b);

  // -------------------------------------------
  let p = Point { x: 0, y: 7 };

  match p {
    Point { x, y: 0 } => println!("On the x axis at {}", x),
    Point { x: 0, y } => println!("On the y axis at {}", y),
    Point { x, y } => println!("On neither axis: ({}, {})", x, y),
  }

  // ------------------------------------------
  let msg = Message::ChangeColor(0, 160, 255);
  match msg {
    Message::Quit => {
      println!("The Quit variant has no data to destruture");
    }
    Message::Move { x, y } => {
      println!("Move in the x direction {} and in the y direction {}", x, y);
    }
    Message::Write(text) => {
      println!("Text message: {}", text);
    }
    Message::ChangeColor(r, g, b) => {
      println!("Change the color to red {}, green {}, and blue {}",
               r, g, b);
    }
  }

  // --------------------------------------------
  let msg_plus = Message_plus::ChangeColor(Color::Hsv(0, 160, 255));
  match msg_plus {
    Message_plus::ChangeColor(Color::Rgb(r, g, b)) => {
      println!("Change the color to red {}, green {}, and blue {}", r, g, b);
    }
    Message_plus::ChangeColor(Color::Hsv(h, s, v)) => {
      println!("Change the colot to hue {}, saturation {}, and value {}", h, s, v);
    }
  }

  // ---------------------------------------------
  let numbers = (2, 4, 8, 16, 32);

  match numbers {
    (first, .., last) => {
      println!("Some numbers: {}, {}", first, last);
    }
  }

  // --------------------------------------------
  let num = Some(4);

  match num {
    Some(x) if x < 5 => println!("less than five: {}", x),
    Some(x) => println!("{}", x),
    None => (),
  }

  // ---------------------------------------------
  let x = 4;
  let y = false;

  match x {
    4 | 5 | 6 if y => println!("yes"),
    _ => println!("no"),
  }

  // ------------------------------------------------
  let msgx = Message_plusx::Hello { id: 5 };
  match msgx {
    Message::Hello { id: id_variable @ 3..=7 } => {
      println!("Found an id in range: {}", id_variable)
    }
    Message::Hello { id: 10..=12 } => {
      println!("Found an id in another range")
    }
    Message::Hello { id } => {
      println!("Found some other id: {}", id)
    }
  }

  // -----------------------------------------------
  let p @ Point { x: px, y: py } = Point { x: 10, y: 23 };
  println!("x: {}, y: {}", px, py);
  println!("{:?}", p);

  let point = Point { x: 10, y: 5 };
  if let p @ Point { x: 10, y } = point {
    println!("x is 10 and y is {} in {:?}", y, p);
  } else {
    println!("x was not 10 :(");
  }

  match 1 {
    num @ (1 | 2) => {
      println!("{}", num);
    }
    _ => {}
  }
}

enum Message {
  Quit,
  Move { x: i32, y: i32 },
  Write(String),
  ChangeColor(i32, i32, i32),
}

enum Message_plus {
  ChangeColor(Color)
}

enum Message_plusx {
  Hello { id: i32 }
}

enum Color {
  Rgb(i32, i32, i32),
  Hsv(i32, i32, i32),
}

#[derive(Debug)]
struct Point {
  x: i32,
  y: i32,
}


