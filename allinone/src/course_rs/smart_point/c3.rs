use std::rc::Rc;

pub fn comm() {
  let s = String::from("hello, world");
  // todo: s move to `a`
  // let a = Box::new(s);
  // todo: error! continue move s to `b`
  // let b = Box::new(s);

  // -------------------------------------------------
  // todo: use `Rc` to fix above problem
  let a = Rc::new(String::from("hello, world"));
  let b = Rc::clone(&a);

  println!("Rc count {}", Rc::strong_count(&a));
  println!("Rc count a {}", Rc::strong_count(&a));
  println!("Rc count b {}", Rc::strong_count(&b));

  // ---------------------------------------------------
  let a = Rc::new(String::from("test ref counting"));
  println!("count after creating a 1 = {}", Rc::strong_count(&a));

  let b = Rc::clone(&a);
  println!("count after creating a 2 = {}", Rc::strong_count(&a));
  println!("count after creating b = {}", Rc::strong_count(&b));

  {
    let c = Rc::clone(&a);
    println!("count after creating c = {}", Rc::strong_count(&c));
  }

  println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}



