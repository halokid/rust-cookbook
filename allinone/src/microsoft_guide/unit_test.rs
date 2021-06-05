
fn add(a: i32, b: i32) -> i32 {
  a + b
}

/*
#[test]
fn add_works() {
  assert_eq!(add(1, 2), 3);
  // assert_eq!(add(10, 12), 9);
}

#[test]
#[ignore = "not yet"]
fn add_fails() {
  assert_eq!(add(-3, 5), -5);
}

#[test]
#[should_panic]
fn check_fails() {
  assert_eq!(add(2, 3), 6)
}

 */

// -------------------------------------------

// todo: cfg 属性负责控制条件编译，并仅会在谓词为 true 时编译其所附带的内容。 每当执行 $ cargo test 命令时，Cargo 都会自动发出 test 编译标志，因此，当我们运行测试时，该标志将始终为 true。 use super::*; 声明是 add_function_tests 模块内部代码访问外部模块中 add 的必要条件。
#[cfg(test)]  // todo: 定义一块测试逻辑
mod add_function_tests {
  use super::*;

  #[test]
  fn add_works() {
    assert_eq!(add(1, 2), 3);
    assert_eq!(add(10, 12), 22);
    assert_eq!(add(5, -2), 3);
  }

  #[test]
  // #[should_panic]    // todo: 表示这个测试会panic
  fn add_fails() {
    assert_eq!(add(2, 2), 7);
  }

  #[test]
  #[ignore]
  fn add_negatives() {
    assert_eq!(add(-2, -2), -4)
  }
}















