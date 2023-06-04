use futures::executor::block_on;

async fn hello_world() {
    println!("hello, world!");
}

pub fn comm() {
    let future = hello_world();
    block_on(future);
}