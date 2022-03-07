
// todo: use Fn trait objects
type Migration<'a> = Box<dyn Fn() -> &'a str >;

struct Schema<'a> {
  executes:   Vec<Migration<'a>>,
  rollbacks:  Vec<Migration<'a>>,
}

impl<'a> Schema<'a> {
  fn new() -> Self {
    Self {
      executes:   vec![],
      rollbacks:  vec![],
    }
  }

  fn add_migration<E, R> (&mut self, execute: E, rollback: R)
  where
      E:  Fn() -> &'a str + 'static,
      R:  Fn() -> &'a str + 'static,
  {
    self.executes.push(Box::new(execute));
    self.rollbacks.push(Box::new(rollback))
  }

  fn execute(&self) -> Vec<&str> {
    self.executes.iter().map(|cmd| cmd()).collect()
  }

  fn rollback(&self) -> Vec<&str> {
    self.rollbacks.iter().map(|cmd| cmd()).collect()
  }
}

fn add_field() -> &'static str {
  "add field"
}

fn remove_field() -> &'static str {
  "remove field"
}

pub fn comm() {
  let mut schema = Schema::new();
  // todo: type the expression this is a `Fn()`, use (|| + return)
  schema.add_migration(|| "create table", || "drop table");
  // schema.add_migration("create table", "drop table");
  schema.add_migration(add_field, remove_field);
  println!("schema.execute() -->>> {:?}", schema.execute());
  println!("schema.rollback() -->>> {:?}", schema.rollback());
}









