

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
  // todo: 如果我们检查代码，作为人类，我们会看到 magic1 比 magic2 长。 我们会看到，结果包含对 magic1 的引用，该引用的生存期足够长，因此有效。 但是，Rust 在编译时无法运行该代码。 它会将 &magic1 和 &magic2 引用都视为可能的返回值，并会发出我们此前看到的错误。
  // {
  //   let m2 = String::from("xxxx");
  //   res = longest_word(&m1, &m2);
  // }

  let m2 = String::from("xxx");
  res = longest_word(&m1, &m2);

  println!("res ----- {}", res);

  comm2()
}

fn print_greeting(message: &String) {
  println!("Greeting: {}", message);
}

// fn change(message: &String) {   // todo: 错误
fn change(message:  &mut String) {
  message.push_str("!"); // We try to add a "!" to the end of our message
}

// todo: 帮助文本指出 Rust 无法判断返回的引用是指 x 还是 y。 我们也无法判断。 因此，需要使用一个通用生存期参数对返回类型进行批注。 所以要用 <'a> 来表示, 所以有可能在fn返回的变量都要用 'a 在前面做标识
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

#[derive(Debug)]
struct Highligth<'document> (&'document str);

pub fn comm2() {
  let text = String::from("The qucik brown fox jumps over the lazy dog.");
  let fox = Highligth(&text[4..9]);     // todo: string类型就是一个数组
  let dog = Highligth(&text[35..43]);
  println!("fox {:?}", fox);
  println!("dog {:?}", dog);

  // erase(text); // todo： 假如这里这样调用， 则text的所有权会被转移
  // println!("fox {:?}", fox);    // todo: 这样编译会报错，因为上面 text 的所有权已经被转移了
}

fn erase(_: String) {}

// todo: 下面这个函数的参数 和 返回值 都是 borrowed value，但是签名并没有说明它是来自“向量”还是“值”
// fn copy_and_return<'a>(vector: &mut Vec<String>, value: &str) -> &String {
fn copy_and_return<'a>(vector: &'a mut Vec<String>, value: &'a str) -> &'a String {
  vector.push(String::from(value));
  vector.get(vector.len() - 1).unwrap()
}

pub fn comm3() {
  let name1 = "Joe";
  let name2 = "Chris";
  let name3 = "Anne";

  let mut names = Vec::new();

  assert_eq!("Joe", copy_and_return(&mut names, &name1));

  assert_eq!(
    names,
    vec!["Joe".to_string(), "Chris".to_string()]
  )
}


// todo: 结构声明 struct Nominee<'a> { person: &'a Person } 中 <'a> 段的含义是什么？
// todo: Nominee 结构的生存期不能超过它借用的 Person 值的生存期。一旦超过的话， 可能在 Nominee 里返回Person， 就会内存报错




















