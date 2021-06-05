use rusty_pizza::Pizza;

#[test]
fn can_make_pepperoni_pizza() {
  let pizza = Pizza::pepperoni(12);
  println!("pizza 1 ------ {:?}", pizza);
}

#[test]
fn can_make_mozzarella_pizza() {
  let pizza = Pizza::mozzarella(16);
  println!("pizza 2 ------ {:?}", pizza);
}

// --------- 输出结果 -------------
/*
Compiling rusty_pizza v0.1.0 (/Users/kxy/gitcode/rust-cookbook/package_crate_mod_guide/rusty_pizza)
Finished test [unoptimized + debuginfo] target(s) in 0.42s

todo: 单元测试
Running target/debug/deps/rusty_pizza-b090de1498799b78

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

todo: 集成测试
Running target/debug/deps/pizza-56ac2907ef47bfd2

running 2 tests
test can_make_mozzarella_pizza ... ok
test can_make_pepperoni_pizza ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

todo: 文档测试
Doc-tests rusty_pizza

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

 */
