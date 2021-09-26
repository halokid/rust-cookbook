
use std::thread::sleep;
use std::thread;
use tokio::time::Duration;
use std::time;

// todo: 参考 https://users.rust-lang.org/t/how-to-mutate-a-global-variable-using-tokio-and-futures/50276/4

// todo: 异步场景1： 等待一个异步阻塞操作返回，然后主线程获取数据

#[tokio::main]
pub async fn comm() {
  let name = "foo".into();
  let mut say_hello = "".into();

  tokio::task::spawn_blocking(move || {
    say_hello = get_ret(name);
    println!("在spawn里面say hello ------- {}", say_hello);
    // let i = "".into();      // todo: 在async的闭包里， 外面不能获取i
  });

  // println!("say hello ------- {}", say_hello.clone());
  // println!("say hello i ------- {}", i);      // todo: 错误 ^ not found in this scope
}

fn get_ret(name: String) -> String {
  return format!("hello {}", name)
}