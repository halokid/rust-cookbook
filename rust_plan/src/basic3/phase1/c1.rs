
async fn do_work() -> i32 {
  42
}

pub async fn c1() {
  let result = do_work().await;
  println!("Got result: {}", result);
}