/*
桥接模式
 */


// todo:  1. 定义实现类的接口
trait Implememtor {
  fn decorate(&self, _: String) -> String;
}

// todo: 2. 定义具体实现类
struct ParentImpl;
impl Implememtor for ParentImpl {
  fn decorate(&self, msg: String) -> String {
    "Parent impl (".to_string() + &msg + &")".to_string()
  }
}

struct BracketImpl;
impl Implememtor for BracketImpl {
  fn decorate(&self, msg: String) -> String {
    "Bracket impl {".to_string() + &msg + &"}".to_string()
  }
}

// todo: 定义抽象类
struct Abstraction<'a> {
  implementer:  &'a dyn Implememtor
}

impl<'a>  Abstraction<'a> {
  fn new(i: &dyn Implememtor) -> Abstraction {
    Abstraction {
      implementer:  i,
    }
  }

  fn convert(&self, msg: String) -> String {
    self.implementer.decorate(msg)
  }
}


// todo: 定义了抽象类的具体实现
struct RefinedAbstraction<'a> {
  abstraction:  Abstraction<'a>,
}

impl <'a> RefinedAbstraction<'a> {
  fn new(i: &dyn Implememtor) -> RefinedAbstraction {
    RefinedAbstraction {
      abstraction:    Abstraction::new(i)
    }
  }

  fn convert(&self, msg: String) -> String {
    self.abstraction.convert(msg)
  }

  // todo: 具体的抽象类实现
  fn print_convert_msg(&self, msg: String) {
    println!("{}", self.abstraction.convert(msg));
  }
}

fn main() {
  let parent_impl = &ParentImpl;
  let bracket_impl = &BracketImpl;

  let abst_p = RefinedAbstraction::new(parent_impl as &dyn Implememtor);
  let abst_b = RefinedAbstraction::new(bracket_impl as &dyn Implememtor);

  println!("{}", abst_p.convert("YOYO".to_string()));
  abst_b.print_convert_msg("opps".to_string());
}






