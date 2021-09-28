// todo: 异步场景7： 多个异步非阻塞任务并行执行, 每个任务执行完汇总返回数据给主线程的变量

use tokio::sync::oneshot;

#[tokio::main]
pub async fn comm() {
    let (tx1, mut rx1) = oneshot::channel();
    let (tx2, mut rx2) = oneshot::channel();

    tokio::spawn(async move {
        tx1.send("first").unwrap();
    });

    tokio::spawn(async move {
        tx2.send("second").unwrap();
    });

    let mut a = None;
    let mut b = None;

    while a.is_none() || b.is_none() {
        tokio::select! {
            v1 = (&mut rx1), if a.is_none() => {
              a = Some(v1.unwrap());
            }

            v2 = (&mut rx2), if b.is_none() => {
              b = Some(v2.unwrap());
            }
        }
    };

    let res = (a.unwrap(), b.unwrap());
    println!("res ----------- {:?}", res);
    // assert_eq!(res.0, "first");
    // assert_eq!(res.1, "second");
}




