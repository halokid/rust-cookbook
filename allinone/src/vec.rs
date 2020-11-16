
pub fn comm() {
  let v: Vec<i32> = Vec::new();     // 空集合
  let v = vec![1, 2, 3];      // 含初始值的集合，vec!是为方便初始化Vec提供的宏。

  println!("第三个元素: {}", &v[2]);
//  println!("第100个元素: {}", &v[100]);     // panic error

  assert_eq!(v.get(2), Some(&3));
  assert_eq!(v.get(100), None);

  // 遍历然后修改vec的元素
  let mut v2 = vec![100, 32, 57];
  for i in &mut v2 {
    *i += 50;
  }
  println!("v2: {:?}", v2);     // todo: 输出一个vec的用法

  // 修改vev第二个元素
  let mut v = vec![1, 2, 3, 4, 5];
  {
    let third = v.get_mut(2).unwrap();
    *third += 50;
  }
  println!("修改第三个元素后 v: {:?}", v);

  // 遍历pop元素
  let mut stack = vec![1, 2, 3, 4, 5];      // 后进先出
  while let Some(top) = stack.pop() {
    println!("top: {}", top);
  }
}