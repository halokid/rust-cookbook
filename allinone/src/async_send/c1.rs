
struct A {
  data: i32,
}

struct B {
  a: A,
}

impl B {
  // fn set_data(&self, data: i32) {
  //   self.a.data = data;   // todo: safe的时候， 你永远不能从不可变的 B 对象上安全地借到一个可变的 data 引用
  // }

  // fn set_data(&self, data: i32) {
  //   unsafe {
  //     (*self.a).data = data;  // todo: 一样编译错误 error[E0614]: type `A` cannot be dereferenced， 这里是为原理阐述，应该用recell之类的内置语法
  //   }
  // }
}







