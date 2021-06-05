
struct Groups<T> {
  inner:  Vec<T>,
}

impl<T> Groups<T> {
  fn new(inner: Vec<T>) -> Self {
    Groups { inner }
  }
}

// todo: 泛型类型 T 需要是特征 PartialEq 的实现器，因为需要比较值才能执行此分配。 但不用担心，因为已为你在 impl<T: PartialEq> 段解决了该部分内容。
// todo: 作为 T 参数传递的值必须实现 PartialEq 特征
impl<T: PartialEq> Iterator for Groups<T> {
  type Item = Vec<T>;     // todo: iterator来说， item是什么，  next返回就是什么

  // todo: 假如vec里面的元素一样， 就组成 子vec
  fn next(&mut self) -> Option<Self::Item> {
    if self.inner.is_empty() {
      return None
    }

    let mut cursor = 1;
    let first = &self.inner[0];
    for element in &self.inner[1..] {
      if element == first {
        cursor += 1;
      }  else {
        break;
      }
    }

    // we use the `Vec::drain` to extract items up until the cursor
    let items = self.inner.drain(0..cursor).collect();

    // return the extracted items
    Some(items)
  }
}

pub fn comm() {
  let data = vec![4, 1, 1, 2, 1, 3, 3, -2, -2, -2, 5, 5];
  let new_groups = Groups::new(data).into_iter()
                      .collect::< Vec< Vec<_> > >();
  println!("new_groups ---------- {:?}", new_groups);
}









