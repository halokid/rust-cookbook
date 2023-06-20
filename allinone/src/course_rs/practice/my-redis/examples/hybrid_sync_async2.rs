use std::time::Duration;
use tokio::runtime::Builder;
use tokio::sync::mpsc;
use tokio::time::sleep;

// TODO: 2. use the send message to run async in sync
pub struct Task {
  name:  String,
}

async fn handle_task(task: Task) {
  println!("Got task {}", task.name);
}

#[derive(Clone)]
pub struct TaskSpawner {
  spawn:  mpsc::Sender<Task>,
}

impl TaskSpawner {

  pub fn new() -> TaskSpawner {
    let (send, mut recv) = mpsc::channel(16);

    let rt = Builder::new_multi_thread()
      .enable_all()
      .build()
      .unwrap();

    std::thread::spawn(move || {
      rt.block_on(async move {
        while let Some(task) = recv.recv().await {
          tokio::spawn(handle_task(task));
        }
      });
    });

    TaskSpawner {
      spawn: send,
    }
  }
}

fn main() {

}







