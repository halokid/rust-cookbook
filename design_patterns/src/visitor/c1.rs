/*
访问者模式
 */


// todo: 1. 抽象访问者
trait Visitor {
  fn visit(&self, _: &dyn Acceptor);
}

// todo: 2. 抽象接收者
trait Acceptor {
  fn accept(&self, _: &dyn Visitor);
  fn get_value(&self) -> &String;
}

// todo: 3. 实现访问者
struct VisitorX;
impl Visitor for VisitorX {
  fn visit(&self, a: &dyn Acceptor) {
    println!("VisitorX - Accept {}", a.get_value());
  }
}

// todo: 4. 实现接收者
struct AcceptorX(String);
impl Acceptor for AcceptorX {
  fn accept(&self, v: &dyn Visitor) {
    v.visit(self);
  }

  fn get_value(&self) -> &String {
    &self.0
  }
}

fn main() {
  let v = VisitorX;
  let a1 = AcceptorX("Number 1".to_string());
  let a2 = AcceptorX("Number 2".to_string());

  // todo: 同一个visitor， 不同的 acceptor 会执行不同的行为
  a1.accept(&v);
  a2.accept(&v);
}





