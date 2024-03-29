use futures::executor::block_on;

async fn hello_world() {
    hello_cat().await;
    println!("hello, world!");
}

async fn hello_cat() {
    println!("hello, kitty!")
}

pub fn comm() {
    let future = hello_world();
    block_on(future);
}