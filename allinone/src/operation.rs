
pub fn comm() {
  // 算数运算符, + - * / %
  let a = 5;
  let b = a + 1; //6
  let c = a - 1; //4
  let d = a * 2; //10
  let e = a / 2; // ⭐️ 2 not 2.5
  let f = a % 2; //1

  let g = 5.0 / 2.0; //2.5


  // 比较运算符, == = != < > <= >=
  let a = 1;
  let b = 2;

  let c = a == b; //false
  let d = a != b; //true
  let e = a < b; //true
  let f = a > b; //false
  let g = a <= a; //true
  let h = a >= a; //true

  let i = true > false; //true
  let j = 'a' > 'A'; //true

  // 逻辑运算符, ! && ||
  let a = true;
  let b = false;

  let c = !a; //false
  let d = a && b; //false
  let e = a || b; //true

  // 位运算符,  & | ^ << >>
  let a = 1;
  let b = 2;

  let c = a & b; //0  (01 && 10 -> 00)
  let d = a | b; //3  (01 || 10 -> 11)
  let e = a ^ b; //3  (01 != 10 -> 11)
  let f = a << b; //4  (左移 -> '01'+'00' -> 100)
  let g = a >> a; //0  (右移 -> o̶1̶ -> 0)

  // 赋值运算符.
  let mut a = 2;

  a += 5; //2 + 5 = 7
  a -= 2; //7 - 2 = 5
  a *= 5; //5 * 5 = 25
  a /= 2; //25 / 2 = 12 not 12.5
  a %= 5; //12 % 5 = 2

  a &= 2; //10 && 10 -> 10 -> 2
  a |= 5; //010 || 101 -> 111 -> 7
  a ^= 2; // todo: 因为这个时候， a是7了，二进制为111, 所以这里用111，^ 的运算规则是, 两个都为1, 则返回0， 其中一个为1， 则返回1， 所以  111 != 010,  计算得出 -> 101  换算为10进制 -> 5
  a <<= 1; //'101'+'0' -> 1010 -> 10
  a >>= 2; //101̶0̶ -> 10 -> 2

  let mut x: i32 = 2;
  x ^= 2;

  println!("x: {}", x);

  let a = 15;
  let b = (a as f64) / 2.0;
}


/*
借用(Borrowing)与解引用(Dereference)操作符
Rust 引入了所有权(Ownership)的概念，所以在引用(Reference)的基础上衍生了借用(Borrowing)的概念，所有权概念不在这里展开。

简单而言，引用是为已存在变量创建一个别名；获取引用作为函数参数称为借用；解引用是与引用相反的动作，目的是返回引用指向的变量本身。
 */

// ========== 借用 ===============
pub fn borr() {
  let s1 = String::from("hello");
  let len = calculate_length(&s1);
  println!("len is: {}", len);
}

fn calculate_length(s: &String) -> usize {  // 获取引用作为函数参数称为借用
  s.len()
}


// ========== 解引用 ===============
pub fn dereference() {
  // 获取v的第2个元素的可变引用，并通过解引用修改该元素的值。
  let v = &mut [1, 2, 3, 4, 5];
  {
    let third = v.get_mut(2).unwrap();
    *third += 50;
  }
  println!("v={:?}", v);
}




















