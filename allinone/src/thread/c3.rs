/*
创建线程的进程结束，子线程就会结束。
创建线程的线程结束， 子线程是不会结束的
 */

use std::thread;

pub fn comm() {
    // 创建一个线程
    let new_thread = thread::spawn(move || {
        // 再创建一个线程
        thread::spawn(move || {
            loop {
                println!("I am a new thread.");
            }
        })
    });

    // 等待新创建的线程执行完成
    new_thread.join().unwrap();
    println!("Child thread is finish!");

    // 睡眠一段时间，看子线程创建的子线程是否还在运行
    thread::sleep_ms(100);
}

/*
结果表明，在父线程结束后，其创建的子线程还活着，这并不会因为父线程结束而结束。这个还是比较符合自然规律的，要不然真会断子绝孙，人类灭绝。所以导致线程结束的第二种方式，是结束其所在进程。
 */