/**
推荐进阶练习计划（4 周）
第 1 周： 深入所有权，做标准库中 String、Vec 的所有权转移模拟
第 2 周： 专注于函数参数的借用和可变借用，用 RefCell 模拟可变性
第 3 周： 生命周期练习，阅读并改写有生命周期注解的源码片段
第 4 周： 综合练习，写一个小型解析器或 CLI 工具，练习三者配合
*/

/**
1. String 的所有权转移示例
步骤详细解释：
let s1 = String::from("hello");
创建一个 String。
在堆上分配空间，存储 "hello"。
s1 在栈上，包含：
ptr 指向堆上 "hello"
len = 5
capacity = 5（或者更大）
let s2 = s1;
把 s1 的栈上的数据（指针、长度、容量）拷贝到 s2。
注意！只是拷贝栈上的数据，不是堆数据。
然后 Rust 认为：s1 不再有效，避免 s1 和 s2 双重释放（double free）。
println!("{}", s2);
正常打印 "hello"。
如果你试图用 s1，编译器就会提示：
error[E0382]: borrow of moved value: `s1`

-----------------
内存示意图：

栈 (stack)                    |           堆 (heap)
s2: {ptr, len=5, cap=5}      |           "h", "e", "l", "l", "o"
（s1 已无效）
*/
pub fn c1() {
  let s1 = "hello".to_string();
  println!("s1 1 -->>> {}", s1);

  let s2 = s1;
  // println!("s1 2 -->>> {}", s1);
}

/**
步骤详细解释：
let v1 = vec![1, 2, 3];
创建一个堆上数组 [1, 2, 3]。
v1 在栈上，持有：
ptr 指向堆上 [1, 2, 3]
len = 3
capacity = 3（或者更大）
let v2 = v1;
把 v1 栈上的 {ptr, len, capacity} 复制到 v2。
堆数据不复制，只复制指针。
v1 被标记为无效，防止两次释放。
println!("{:?}", v2);
正常打印 [1, 2, 3]

内存示意图：

栈 (stack)	                      堆 (heap)
v2: {ptr, len=3, cap=3}	          1, 2, 3
（v1 已无效）
*/
pub fn c2() {
  let v1 = vec![1, 2, 3];
  let v2 = v1;
  println!("{:?}", v2);
}


/**
3. 统一总结
堆数据没有复制。
栈上的指针被移动到新变量。
旧变量（如 s1, v1）立即失效。
如果想要继续使用，必须显式 clone。
let s2 = s1.clone();
这样， s1 和  s2 都能独立存在

clone 会发生什么？
clone 会申请新的堆空间。
把堆上 "hello" 内容逐字节复制过去。
两个指针分别指向不同的堆内存。
s1 和 s2 都是有效的。
*/


// ---------------------------------------------
/**
详细解释：
s1 是一个 String，本来在 c3 函数里。
当你调用 takes_ownership(s1) 时，s1 的所有权被移动到函数参数 s。
函数内的 s，在栈上持有 {ptr, len, cap}，指向堆上 "hello"。
s1 在 c3 函数中 变为无效，不能再访问！
函数执行结束时，s 会被销毁（drop），堆上 "hello" 的内存也会被释放。

*/
fn takes_ownership(s: String) {
  println!("{}", s);
}

fn c3() {
  let s1 = String::from("hello");
  takes_ownership(s1);
  // println!("{}", s1); // ❌ 编译错误！s1 已无效
}

/**
默认传参（像 takes_ownership(s1)) 就是 move。
如果希望函数用完后还想继续用 s1，你有两个选择：
函数返回所有权
函数接受引用（borrow）
*/
fn takes_and_gives_back(s: String) -> String {
  println!("{}", s);
  s
}

/**
&s1 是 s1 的不可变借用（immutable borrow）。
函数 borrow_string 只是借一下 s1，不会拿走所有权。
所以 s1 依然有效！
s1 和 &s 同时访问同一块堆内存，但 &s 不能修改。
没有拷贝，也不会释放。
*/
fn borrow_string(s: &String) {
  println!("{}", s);
}








