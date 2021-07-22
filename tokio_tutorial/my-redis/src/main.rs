use mini_redis::{ client, Result };

#[tokio::main]
pub async fn main() -> Result<()> {
  // open a connection to the redis address
  let mut client = client::connect("127.0.0.1:6379").await?;
  client.set("hello", "world".into()).await?;
  let result = client.get("hello").await?;
  println!("got value from server; result={:?}", result);

  // ----------------------------------------------------
  // todo: 调用 `say_world()`， 但是并不执行函数里面的逻辑实体
  let op = say_world();
  println!("hello");
  // todo: 在 op 对象上调用 `.await`, 开始执行 `say_world()`
  op.await;

  Ok(())
}

async fn say_world() {
  println!("world");
}


