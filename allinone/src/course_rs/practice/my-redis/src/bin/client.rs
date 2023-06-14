use mini_redis::client;

#[tokio::main]
async fn main() {
  let mut clientx = client::connect("127.0.0.1:6379")
    .await.unwrap();

  let t1 = tokio::spawn(async {
    let res = clientx.get("hello").await;
  });

  let t2 = tokio::spawn(async {
    clientx.set("foo", "bar".into()).await;
  });

  t1.await.unwrap();
  t2.await.unwrap();
}