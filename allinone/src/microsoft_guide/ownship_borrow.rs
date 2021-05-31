

// todo: 通过引用，我们可以“借用”一些值，而无需拥有它们。

pub fn comm() {
  let greeting = String::from("hello");
  let greeting_reference = &greeting; // We borrow `greeting` but the string data is still owned by `greeting`
  println!("Greeting: {}", greeting); // We can still use `greeting`

  print_greeting(&greeting); // `print_greeting` takes a `&String` not an owned `String` so we borrow `greeting` with `&`
  print_greeting(&greeting); // Since `greeting` didn't move into `print_greeting` we can use it again

  let mut change_ref = String::from("xxx");
  change(&mut change_ref);

  let mut val = String::from("hello");

  // todo: 有可变引用 也有 正常的引用， 会产生编译错误
  // todo: 代码必须同时实现以下任一定义，但不能同时实现这两个定义：
  // todo: 一个或多个不可变引用 (&T)
  // todo: 恰好一个可变引用 (&mut T)
  // let ref1 = &val;
  // let ref2 = &mut val;
  // println!("{}, {}", ref1, ref2);

  let m1 = String::from("abcxxxxx");
  let res;
  {
    let m2 = String::from("xxxx");
    res = longest_word(&m1, &m2);
  }
  println!("res ----- {}", res);
}

fn print_greeting(message: &String) {
  println!("Greeting: {}", message);
}

// fn change(message: &String) {   // todo: 错误
fn change(message:  &mut String) {
  message.push_str("!"); // We try to add a "!" to the end of our message
}

// todo: 帮助文本指出 Rust 无法判断返回的引用是指 x 还是 y。 我们也无法判断。 因此，需要使用一个通用生存期参数对返回类型进行批注。
//
// 每次调用函数时，生存期都可能不同。 我们不知道将传递给 longest_word 函数的引用的具体生存期，也无法确定将返回的引用是否始终是有效的引用。
//
// 借用检查器也无法确定这一点，因为它不知道输入参数的生存期与返回值的生存期之间存在怎样的关系。 这就是我们需要手动批注生存期的原因。
//
// 幸运的是，编译器向我们提供了有关如何修复此错误的提示。 我们可以向函数签名中添加通用生存期参数。 这些参数定义了引用之间的关系，方便借用检查器执行其分析：
fn longest_word<'a>(x: &'a String, y: &'a String) -> &'a String {
  if x.len() > y.len() {
    x
  } else {
    y
  }
}






