use std::convert::TryInto;
use std::sync::Arc;

pub fn comm() {
  // let a: u8 = 10;
  // let b: u16 = 1500;

  let b: i16 = 1500;

  let b_: u8 = match b.try_into() {
    Ok(b1) => b1,
    Err(e) => {
      println!("{:?}", e.to_string());
      0
    }
  };
}

struct Foo {
  x:  u32,
  y:  u16,
}

struct Bar {
  a:  u32,
  b:  u16,
}

fn reinterpret(foo: Foo) -> Bar {
  let Foo {x, y} = foo;
  Bar {
    a: x,
    b: y,
  }
}

// ------------------------------------------
#[derive(Clone)]
struct Container<T>(Arc<T>);

fn clone_containers<T>(foo: &Container<i32>, bar: &Container<T>) {
  let foo_cloned = foo.clone();
  let car_cloned = bar.clone();
}

/// 然而，bar_cloned 的类型却是 &Container<T>，这个不合理啊，明明我们为 Container<T> 派生了 Clone 特征，因此它也应该是 Container<T> 类型才对。万事皆有因，我们先来看下 derive 宏最终生成的代码大概是啥样的：
/// ```
/// impl<T> Clone for Container<T> where T: Clone {
///     fn clone(&self) -> Self {
///         Self(Arc::clone(&self.0))
///     }
/// }
/// ```
/// 从上面代码可以看出，派生 Clone 能实现的根本是 T 实现了Clone特征：where T: Clone， 因此 Container<T> 就没有实现 Clone 特征。
///
/// 编译器接着会去尝试引用方法调用，此时 &Container<T> 引用实现了 Clone，最终可以得出 bar_cloned 的类型是 &Container<T>。
///
/// 当然，也可以为 Container<T> 手动实现 Clone 特征：
/// ```
/// impl<T> Clone for Container<T> {
///     fn clone(&self) -> Self {
///         Self(Arc::clone(&self.0))
///     }
/// }
/// ```
/// 此时，编译器首次尝试值方法调用即可通过，因此 bar_cloned 的类型变成 Container<T>。
/// 这一块儿内容真的挺复杂，每一个坚持看完的读者都是真正的勇士，我也是：为了写好这块儿内容，作者足足花了 4 个小时！















