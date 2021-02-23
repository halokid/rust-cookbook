/*
装饰器模式
 */

// todo: 1. 创建一个接口
trait Component {
  fn do_something(&self);   // 原本要实现的功能
}

// todo: 2. 创建实现接口的实体类。
struct BaseObject(usize);
impl Component for BaseObject {
  fn do_something(&self) {
    println!("do somthing: {}", self.0);
  }
}

// todo: 3. 创建实现了 Component 接口的抽象装饰类, 要额外扩展的功能接口都定义在这个类
trait Decorator:  Component {
  fn do_something_more(&self);    // 要实现更多的功能
}

// todo: 4. 创建扩展了 Compoent 类的实体装饰类 DecoratorObject。
struct DecoratorObject {
  base:          BaseObject,    // todo: 入口来调用基类的属性、方法
  more_value:    usize,
}

impl Component for DecoratorObject {
  fn do_something(&self) {
    self.base.do_something();
  }
}

// todo: 5. 实现了 DecoratorObject 具体的行为, 这里定义额外扩展的功能逻辑
impl Decorator for DecoratorObject {
  fn do_something_more(&self) {
    println!("do something more: {}", self.more_value);
  }
}

// todo: 原来的基类的逻辑
fn process(c: &dyn Component) {     // 参数为指针引用
  c.do_something()
}

fn main() {
  let obj = BaseObject(100);
  process(&obj);

  let dobj = DecoratorObject {
    base:         obj,
    more_value:   39,
  };
  process(&dobj);
  dobj.do_something_more();     // todo: 装饰器的具体实现的逻辑
}




