
struct Container {
  value:  u32,
}

impl Container {
  pub fn new(value: u32) -> Self {
    Container { value }
  }
}

pub fn comm() {
  assert_eq!(Container::new(42).value, 42);
  println!("Container value {}", Container::new(29).value);

  assert_eq!(Containerx::new("Foo").value, "Foo");
  println!("Containerx value {}", Containerx::new("halokid").value);
}


// ------------------------------------
// todo: 把接受 u32 的 转成接收泛型 T
struct Containerx<T> {      // todo: <T> 是证明 Containerx的作用于 和 T 的作用域是一样的， 假如 Containerx 的作用域大于T的作用域， 那么返回 Containerx的时候， 可能已经超过T了，则value会出现错误
  value:  T,
}

impl<T> Containerx<T> {
  pub fn new(value: T) -> Self {
    Containerx { value }
  }
}



