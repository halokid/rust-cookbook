// todo: when the same time, more owner want own one varible, if they are the
// todo: same thread, use Rc, different thread, use Arc

use std::rc::Rc;

pub fn comm() {
  // todo: wrong usage
  let five = 5;
  // let five2 = five.clone();
  // println!("five2 -->>> {}", five2);
  // let five3 = five.clone();
  // println!("five3 -->>> {}", five3);

  let mut b = Rc::new(1);
  // *b = 2;
  let c = *b + 1;
  println!("b -->>> {}", b);
  println!("c -->>> {}", c);

  for _ in 0..3 {
    let ff = five.clone();
    println!("ff -->>> {}", ff);
  }

  #[derive(Debug, Clone)]
  struct FileName {
    name: String,
    ext: String,
  }

  fn ref_counter() {
    let name = String::from("main");
    let ext = String::from("rs");

    for _ in 0..3 {
      let f = FileName {
        name: name.clone(),
        ext: ext.clone(),
      };
      println!("fx -->>> {:?}", f);
    }
  }

  ref_counter();
}