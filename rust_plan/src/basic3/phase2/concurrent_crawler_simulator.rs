use tokio::time::{sleep, Duration};
use tokio::spawn;

async fn download(url: &str, delay: u64) {
  println!("Start downloading: {}", url);
  sleep(Duration::from_secs(delay)).await;
  println!("Finished downloading: {}", url);
}

pub async fn c1() {
  let urls = vec![
    ("https://a.com", 2),
    ("https://b.com", 3),
    ("https://c.com", 1),
  ];

  let handles: Vec<_> = urls.into_iter().map(|(url, delay)| {
    spawn(download(url, delay))
  }).collect();

  for h in handles {
    h.await.unwrap();
  }

  println!("All downloads complete.")
}