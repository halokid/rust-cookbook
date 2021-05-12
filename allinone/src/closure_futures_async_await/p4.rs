
// todo: 两个等价的异步函数

use futures::Future;

// 1
async fn async_return_i32() -> i32 {
  42
}

// 2
fn return_future_i32() -> impl Future<Output = i32> {
  async{ 42 }
}


