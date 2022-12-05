// todo: 线程同步：Atomic 原子类型与内存顺序, Atomic的原理相当于乐观锁

use std::ops::Sub;
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;
use std::thread::JoinHandle;
use std::time::Instant;

const N_TIMES: u64 = 100000;
const N_THREADS: usize = 10;

static R: AtomicU64 = AtomicU64::new(0);

fn add_n_times(n: u64) -> JoinHandle<()> {
  thread::spawn(move || {
    for _ in 0..n {
      R.fetch_add(1, Ordering::Relaxed);
    }
  })
}

pub fn comm() {
  let s = Instant::now();
  let mut threads = Vec::with_capacity(N_THREADS);

  for _ in 0..N_THREADS {
    threads.push(add_n_times(N_TIMES));
  }

  for thread in threads {
    thread.join().unwrap();
  }

  println!("{}, {}", N_TIMES * N_THREADS as u64, R.load(Ordering::Relaxed));
  println!("{:?}", Instant::now().sub(s));
}

/*
限定内存顺序的 5 个规则
在理解了内存顺序可能存在的改变后，你就可以明白为什么 Rust 提供了Ordering::Relaxed用于限定内存顺序了，事实上，该枚举有 5 个成员:

Relaxed， 这是最宽松的规则，它对编译器和 CPU 不做任何限制，可以乱序
Release 释放，设定内存屏障(Memory barrier)，保证它之前的操作永远在它之前，但是它后面的操作可能被重排到它前面
Acquire 获取, 设定内存屏障，保证在它之后的访问永远在它之后，但是它之前的操作却有可能被重排到它后面，往往和Release在不同线程中联合使用
AcqRel, 是 Acquire 和 Release 的结合，同时拥有它们俩提供的保证。比如你要对一个 atomic 自增 1，同时希望该操作之前和之后的读取或写入操作不会被重新排序
SeqCst 顺序一致性， SeqCst就像是AcqRel的加强版，它不管原子操作是属于读取还是写入的操作，只要某个线程有用到SeqCst的原子操作，线程中该SeqCst操作前的数据操作绝对不会被重新排在该SeqCst操作之后，且该SeqCst操作后的数据操作也绝对不会被重新排在SeqCst操作前。
这些规则由于是系统提供的，因此其它语言提供的相应规则也大同小异，大家如果不明白可以看看其它语言的相关解释。
 */








