
// todo: use trait object, the key point is separate
// todo: behavior with schema, use the behavior trait
// todo: to define behavior(method), use sttruct to process
// todo: schema, because schema is the process encapsulate

// declear the behavior trait, the behavior name `Migration`
pub trait Migration {
  fn execute(&self) -> &str;
  fn rollback(&self) -> &str;
}

// behavior one as a struct
pub struct CreateTable;
impl Migration for CreateTable {
  fn execute(&self) -> &str {
    "create table"
  }

  fn rollback(&self) -> &str {
    "drop table"
  }
}


// behavior two as a struct
pub struct AddField;
impl Migration for AddField {
  fn execute(&self) -> &str {
    "add field"
  }

  fn rollback(&self) -> &str {
    "remove field"
  }
}

// behavior schema as struct
struct Schema {
  commands: Vec<Box<dyn Migration>>,
}

impl Schema {
  fn new() -> Self {
    commands:  vec![]
  }

  fn add_migration(&mut self, cmd: Box<dyn Migration>) {
    self.commands.push(cmd);
  }

  fn execute(&self) -> Vec<&str> {
    self.commands.iter().map( |cmd|
      cmd.execute()
    ).collect()
  }
}














