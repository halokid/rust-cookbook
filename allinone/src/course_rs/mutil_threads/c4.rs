// todo: 使用 thread_local 宏可以初始化线程局部变量，然后在线程内部使用该变量的 with 方法获取变量值

/*
use std::cell::RefCell;
use std::thread;

thread_local!(static FOO: RefCell<u32> = RefCell::new(1));

FOO.with( |f| {
  assert_eq ! ( *f.borrow(), 1);
  *f.borrow_mut() = 2;
});

// 每个线程开始时都会拿到线程局部变量的FOO的初始值
let t = thread::spawn(move | | {
FOO.with( | f | {
assert_eq ! ( * f.borrow(), 1);
* f.borrow_mut() = 3;
});
});

// 等待线程完成
t.join().unwrap();

// 尽管子线程中修改为了3，我们在这里依然拥有main线程中的局部值：2
FOO.with( | f| {
assert_eq ! ( * f.borrow(), 2);
});
 */



