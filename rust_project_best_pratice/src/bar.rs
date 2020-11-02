use crate::foo;     // todo: crate的声明范围是整个src文件夹， 可以调用src里面的任意文件

pub fn bar_say_foo() {
  foo::say_foo();
}

