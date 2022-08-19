


#[derive(Debug)]
enum UiObject {
  BUtton,
  SelectBox,
}

pub fn comm() {
  let objects = [
    UiObject::BUtton,
    UiObject::SelectBox,
  ];

  for o in objects {
    draw(o);
  }
}

fn draw(o: UiObject) {
  println!("{:?}", o);
}