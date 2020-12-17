
pub fn comm() {
  let nums = [1, 2, 3, 4, 5];
  println!("{:?}", nums);

  // todo: Vec是 “ 连续的，可增长的数组类型”
  let mut v = vec![1, 2, 3];
  v.push(4);
  assert_eq!(v.pop(), Some(4));

  v.push(4);
  v.push(5);
  v.push(6);
  assert_eq!(v.pop(), Some(6));
  assert_eq!(v[2], 3);
  println!("{:?}", v);
}