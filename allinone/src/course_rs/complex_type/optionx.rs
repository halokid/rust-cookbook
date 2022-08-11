



fn plus_one(x: Option<i32>) -> Option<i32> {
  match x {
    None => None,
    Some(i) =>  Some(i + 1)
  }
}

pub fn comm() {
  let five = Some(5);
  let six = plus_one(five);
  dbg!(&six);
  let none = plus_one(None);
  dbg!(&none);
}



