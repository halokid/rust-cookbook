

// todo: 通过引用，我们可以“借用”一些值，而无需拥有它们。

pub fn comm() {
  let greeting = String::from("hello");
  let greeting_reference = &greeting; // We borrow `greeting` but the string data is still owned by `greeting`
  println!("Greeting: {}", greeting); // We can still use `greeting`

  print_greeting(&greeting); // `print_greeting` takes a `&String` not an owned `String` so we borrow `greeting` with `&`
  print_greeting(&greeting); // Since `greeting` didn't move into `print_greeting` we can use it again

  let mut change_ref = String::from("xxx");
  change(&mut change_ref);

  let mut val = String::from("hello");

  let ref1 = &val;
  let ref2 = &mut val;
  println!("{}, {}", ref1, ref2);
}

fn print_greeting(message: &String) {
  println!("Greeting: {}", message);
}

// fn change(message: &String) {   // todo: 错误
fn change(message:  &mut String) {
  message.push_str("!"); // We try to add a "!" to the end of our message
}



