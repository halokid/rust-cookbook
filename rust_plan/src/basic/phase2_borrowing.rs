/**
借用规则

同一时间只能有一个可变借用 或 任意多个不可变借用

借用不能与原始所有权者同时存在（防止数据竞争）
*/

fn no_change_borrowing(s: &String) {
  println!("{}", s);
}

fn change_borrowing(s: &mut String) {
  s.push_str(" has been changed by fn `change_borrowing`");
}

/**
练习：

写一个函数修改一个字符串，但主函数中仍可使用它
*/
pub fn c1() {
  let mut s = "initial s".to_string();
  change_borrowing(&mut s);
  println!("s -->>> {}", s);
}