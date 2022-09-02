pub fn comm() {
  let s1 = String::from("abcd");
  let s2 = "xyz";

  let result = longest(s1.as_str(), s2);
  println!("result -->>> {}", result);

  // ------------------------------------------------
  /// string1 的作用域直到 main 函数的结束，而 string2 的作用域到内部花括号的结束 }，那么根据之前的理论，'a 是两者中作用域较小的那个，也就是 'a 的生命周期等于 string2 的生命周期，同理，由于函数返回的生命周期也是 'a，可以得出函数返回的生命周期也等于 string2 的生命周期。
  let string1 = String::from("long string is long");
  let shorter_var;
  {
    let string2 = String::from("xyz");
    shorter_var = longest(string1.as_str(), string2.as_str());
    println!("The longest string is {}", result);
  }
  // todo: compile error, the `result` life cycle is shorter, not alive here.
  // println!("The longest string is {}", shorter_var);

  // ----------------------------------------------------
  let novel = String::from("Call me Ishmael. Some years ago...");
  let first_sentence = novel.split('.').next().expect("Could not find a '.'");
  let i = ImportantExcerpt {
    part: first_sentence,
  };
}

// fn longest(x: &str, y: &str) -> &str {
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  if x.len() > y.len() {
    x
  } else {
    y
  }
}

// fn longest_x<'a>(x: &str, y: &str) -> &'a str {
//   let result = String::from("really long string");
//   result.as_str()
// }

fn longest_y<'a>(_x: &str, _y: &str) -> String {
  String::from("really long string")
}

struct ImportantExcerpt<'a> {
  part: &'a str,
}

/// 对于 first_word 函数，它的返回值是一个引用类型，那么该引用只有两种情况：
/// 1. 从参数获取
/// 2. 从函数体内部新创建的变量获取
///
/// 如果是后者，就会出现悬垂引用，最终被编译器拒绝，因此只剩一种情况：返回值的引用是获取自参数，这就意味着参数和返回值的生命周期是一样的。道理很简单，我们能看出来，编译器自然也能看出来，因此，就算我们不标注生命周期，也不会产生歧义。

/// 在开始之前有几点需要注意：
///
/// 消除规则不是万能的，若编译器不能确定某件事是正确时，会直接判为不正确，那么你还是需要手动标注生命周期
/// 函数或者方法中，参数的生命周期被称为 输入生命周期，返回值的生命周期被称为 输出生命周期

fn first_world(s: &str) -> &str {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return &s[0..i];
    }
  }

  &s[..]
}






