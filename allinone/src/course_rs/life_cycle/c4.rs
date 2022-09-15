use std::fmt::{Debug, Display};

pub fn comm() {
  let mark_twain = "Samuel Clemens";
  print_author(mark_twain);

  print(&mark_twain);

  let i = 5;

  // print_it(&i);
  // print_it1(&i);
  print_it2(&i);
}

fn print_author(author: &'static str) {
  println!("{}", author);
}

fn print<T: Display + 'static>(message: &T) {
  println!("{}", message);
}

// --------------------------------------------
fn print_it<T: Debug + 'static>(input: T) {
  println!("'static value passed in is: {:?}", input);
}

fn print_it1(input: impl Debug + 'static) {
  println!("'static value passed in is: {:?}", input);
}

fn print_it2<T: Debug + 'static>(input: &T) {
  println!("'static value passed in is: {:?}", input);
}



