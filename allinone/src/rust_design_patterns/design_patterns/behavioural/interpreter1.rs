use core::panicking::panic;

pub struct Interpreter<'a> {
  it:   std::str::Chars<'a>,
}

impl<'a> Interpreter<'a> {

  pub fn new(infix: &'a str) -> Self {
    Self {
      it: infix.chars()
    }
  }

  fn next_char(&mut self) -> Option<char> {
    self.it.next()
  }

  fn term(&mut self, out: &mut String) {
    match self.next_char() {
      Some(ch) if ch.is_digit(10) => out.push(ch),
      Some(ch) => panic!("Unexpected symbol '{}'", ch),
      None => panic!("Unexpected end of string"),
    }
  }

  pub fn interpret(&mut self, out: &mut String) {
    self.term(out);

    while let Some(op) = self.next_char() {
      if op == '+' || op == '-' {
        self.term(out);
      } else {
        panic!("Unexpected symbol '{}'", op)
      }
    }
  }
}


