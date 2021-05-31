
pub fn comm() {
  // 作用域界面原则 `{}`
  {
    let mascot = String::from("halokid");
    println!("mascot --------- {}", mascot);
    // todo: 在这一行 mascot 会被丢弃， string数据内存会被释放
  }

   {
    let mascot = String::from("halokid");
    println!("mascot --------- {}", mascot);
    // todo: 将 mascot 的 ownship（所有权） 转给 ferris
    let ferris = mascot;
    // todo: 在这一行 ferris 会被丢弃， string数据内存会被释放
    // todo: 在 Rust 中，“转移所有权”被称为“移动”。 换句话说，在上面的示例中，String 值已从 mascot 移动到了 ferris。
    //  println!("{}", mascot);    // todo: 这里再输出mascot， 编译器不会通过

     // todo: 在 Rust 中，一个项一次只能拥有一段数据。
  }

  // todo: 如前面的代码片段所示，对 process 的第一次调用会转移变量 s 的所有权。 编译器会跟踪所有权，因此对 process 的第二次调用会导致错误。 在资源被移动后，将无法再使用以前的所有者。此模式对编写 Rust 代码的方式有着深远的影响。 它是 Rust 提出的内存安全承诺的核心。在其他编程语言中，s 变量的 String 值在传递给我们的函数之前可以隐式复制。 但在 Rust 中，此操作不会发生。在 Rust 中，所有权转移（即移动）是默认行为
  let s = String::from("hello");
  process(s);     // ownership of the string in `s` moved into `process`
  // process(s);     // Error! ownership already moved.

  // todo: 要解决上面的问题， 那么我们可以 “复制而不是移动”
  // todo: 简单类型，如数字“复制”类型。 它们实现 Copy 特征，这意味着它们被复制而不是移动。 大多数简单类型都执行相同的操作。 复制数字的成本低，因此复制这些值是有意义的。 复制字符串、向量或其他复杂类型的成本可能非常高昂，因此它们没有实现 Copy 特征，而是被移动
  let n = 1u32;
  process_u(n); // Ownership of the number in `n` copied into `process`
  process_u(n); // `n` can be used again because it wasn't moved, it was copied.

  // todo: 复制不实现 Copy 的类型
  // todo: 解决上述错误的一种方法是：在移动类型之前，显式复制它们，这在 Rust 中称为克隆。 调用 .clone 会复制内存并生成一个新值。 新值被移动，这意味着仍然可以使用旧值。
  let sx = String::from("hellox");
  process(sx.clone()); // Passing another value, cloned from `s`.
  process(sx); // s was never moved and so it can still be used.

  // todo: 这种方法可能有用，但会导致代码运行速度变慢，因为每次调用 clone 都是对数据的一次完整复制。 此方法通常包括内存分配或其他成本高昂的操作。 我们可使用引用来“借用”值，从而避免这些成本。 我们将在下一单元中了解如何使用“引用”。
}

fn process(input: String) {}

fn process_u(input: u32) {}






