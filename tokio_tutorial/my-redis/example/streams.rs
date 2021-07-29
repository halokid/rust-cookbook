
use tokio_stream::StreamExt;
use mini_redis::client;

async fn pubilsh() -> mini_redis::Result<()> {
  let mut client = client::connect("127.0.0.1:6379").await?;

  // todo: 神奇的 x.into() 方法， 只要是能通过 from 取得的数据类型，都可以用 into() 方法来取得相应的类型数据， 主体对象是一个泛型， x.into() 方法会把 x 转换成当前上下文语法需要的正确类型
  client.publish("numbers", "1".into()).await?;
  client.publish("numbers", "two".into()).await?;
  client.publish("numbers", "3".into()).await?;
  client.publish("numbers", "four".into()).await?;
  client.publish("numbers", "five".into()).await?;
  client.publish("numbers", "6".into()).await?;
}

async fn subscribe() -> mini_redis::Result<()> {
  let client = client::connect("127.0.0.1:6379").await?;
  let subscriber = client::Subscriber(vec!["numbers".to_string()]).await?;
  // todo: 在订阅者上面调用了 into_stream() 函数，这个函数消费了 Subscriber 然后返回了一个 在接收到消息时迭代数据的 Stream
  let messages = subscriber.into_stream();

  // todo: 一个 Rust 的值被 Pin 之后意味着他将不会在内存中被移动，一个被 Pin 的值所具有的一个核心属性是：调用者能够安全的获取其中的指针信息，并且其中的指针信息必定是有效的。这个特性是用来为 async/await 中跨多个 .await 调用的实现提供支持的。
  // todo: 调用 next() 函数需要这个 Stream 是被 Pinned 钉住 的
  tokio::pin!(messages);
}

#[tokio::main]
async fn main() {

}