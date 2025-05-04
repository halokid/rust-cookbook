use std::cell::RefCell;
use std::rc::Rc;

/**
复杂所有权模型（结构体嵌套、Box/Rc、可变共享/内部可变性、生命周期）
*/

#[derive(Debug)]
struct Address {
  city: String,
}

#[derive(Debug)]
struct Person {
  name: String,
  address:  Address,
}

pub fn test_struct_ownership() {
  let p = Person {
    name: "Alice".to_string(),
    address: Address {
      city: "London".to_string()
    },
  };

  /**
  题目 1：结构体嵌套所有权

  p.address.city 被拿走后，p 还能用吗？
  如何只移动 city，保留 name？
  ---------------
  Q:
  p.address.city 是 字段的 move，意味着整个 p.address 被破坏（address 不完整），导致 p 也无法再用。
  Rust 要求：结构体字段一旦部分 move，整个结构体就不能再被使用（除非字段实现了 Copy）
  */
  let city = p.address.city;
  // println!("p -->>> {:#?}", p);
  println!("p.name -->>> {}", p.name);
  // println!("p.adress -->>> {}", p.address.city);
}

/**
题目 2：Box 所有权

这段代码能编译通过吗？
Box 解引用后是 move 还是 clone？
如何只读值而不 move？
---------------------
Q:
Box<T> 解引用 *b 会 尝试移动 其中的 T。
而 String 不实现 Copy，解引用时 move 发生。
所以代码不能编译。

*/
pub fn test_box_ownership() {
  let b = Box::new(String::from("hello"));
  // let s = *b;
  let s = b.clone();
  println!("s -->>> {}", b);
}

/**
是否发生 move？
a、b、s 谁拥有值？
为什么 Rc 可以这么用？
------------------------
Q:
Rc 是 多所有者容器，通过引用计数在多个变量间共享数据。
Rc::clone 并不会深拷贝数据，只是 增加引用计数，指向同一份 String。
所以：s、a、b 都是拥有者，不会 move。

Rc 适用于：单线程下多个拥有者共享只读数据。多线程需用 Arc。
*/
pub fn test_rc_ownership() {
  let s = Rc::new(String::from("shared"));
  let a = Rc::clone(&s);
  let b = Rc::clone(&s);

  println!("{}, {}, {}", a, b, s); // ✅ OK
}


/**
问题：
是否违反借用规则？
RefCell 内部是如何实现可变性的？
怎么改才能运行？
------------------
Q:
RefCell 在运行时检查借用规则（而不是编译时）
一旦借用了 borrow_mut()（可变借用），就不能再使用 borrow()（不可变借用）
此处逻辑违反了“不可变 & 可变不能同时存在”的规则
RefCell 是 Rust 中唯一允许在运行时可变共享的类型，用于突破编译器限制，但必须小心使用。
*/
pub fn test_refcell_ownership() {
  let data = RefCell::new(String::from("hi"));
  // right
  let mut b1 = data.borrow_mut();
  b1.push_str(" world");  // ✅ 用完后 drop b1
  drop(b1);                // 👈 显式释放

  let b2 = data.borrow(); // ✅ 现在可借用不可变引用
  println!("{}", b2);

  // worong
  // let mut b1 = data.borrow_mut();
  // let b2 = data.borrow();  //  ❌ 编译时报错
  //
  // b1.push_str(" world");
  // println!("{}", b2);
}

/**
这段代码能否编译？
函数签名是否正确？为什么？
------------------------
Q:
编译器无法推断 x 和 y 哪个引用的生命周期更长。
如果没有生命周期标注，Rust 会报错： error[E0106]: missing lifetime specifier

返回值的生命周期 'a 取决于 x 和 y 中生命周期较短者
保证返回的引用不会超过任何输入引用的生命周期
*/
// fn longest(x: &str, y: &str) -> &str {
fn longest<'a> (x: &'a str, y: &'a str) -> &'a str {
  if x.len() > y.len() {
    x
  }  else {
    y
  }
}

pub fn test_lifetimes() {
  let x = "a";
  let y = "bc";
  let z = longest(x, y);
}







