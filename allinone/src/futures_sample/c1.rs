/*
use futures::channel::mpsc;
use futures::executor;
use futures::executor::ThreadPool;
use futures::StreamExt;

pub fn comm() {
    //创建一个executor线程池
    let pool = ThreadPool::new().expect("Failed to build pool");
    //创建一个unbounded mpsc channel来进行任务间消息通信
    let (tx, rx) = mpsc::unbounded::<i32>();
}

 */