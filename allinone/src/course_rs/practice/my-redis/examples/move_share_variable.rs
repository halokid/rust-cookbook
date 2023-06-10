use tokio::task;

#[tokio::main]
async fn main() {
  let v = vec![1, 2, 3];

  task::spawn(async move {
    println!("Here is a vec -->>> {:?}", v);
  }).await.expect("task panic");
}