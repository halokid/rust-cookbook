
// 若要实现新的泛型类型，必须在结构名称之后的尖括号内声明类型参数的名称。 然后，可以使用结构定义中的泛型类型，否则，我们将指定具体的数据类型。

// todo: 如果一个函数返回是泛型，那么这个泛型就应该指定一个非泛型的类型，告诉编译器，不然就会编译错误

// todo: 同类型的泛型
#[derive(Debug)]
struct Point<T> {
  x: T,
  y: T,
}

// todo: 不同类型的泛型
struct Pointx<T, U> {
  x: T,
  y: U,
}

pub fn comm() {
  let boolean = Point{ x: true, y: false };
  let ingeter = Point{ x: 1, y: 9 };
  let float = Point{ x: 1.7, y: 4.3 };
  let string_slice = Point{ x: "high", y: "low" };

  let bx = Pointx{ x: 5, y: false};
  // todo: Pointx<f64, &'static str>
  let float_and_string = Pointx { x: 1.0, y: "hey" };
}


