/*
组合模式
 */

trait Composite {
  fn get_name(&self) -> String;
  fn get_child(&self) -> Option<&Box<dyn Composite>>;
  fn set_child(&mut self, _: Box<dyn Composite>);
  fn print_child_name_recursive(&self);
}

struct File {
  name:     String,
  child:    Option<Box<dyn Composite>>,
}

impl File {
  fn new(name:  String) -> File {
    File {
      name:     name,
      child:    None,
    }
  }
}

impl Composite for File {
  fn get_name(&self) -> String {
    self.name.clone()
  }

  fn get_child(&self) -> Option<&Box<dyn Composite>> {
    match self.child {
      Some(ref x) => Some(x),
      None => None,
    }
  }

  fn set_child(&mut self, c: Box<dyn Composite>) {
    self.child = c
  }

  fn print_child_name_recursive(&self) {
    print!(" -> {} ", self.get_name());
    if let Some(x) = self.get_child() {
      x.print_child_name_recursive();
    } else {
      println!();
    }
  }
}




