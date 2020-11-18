use std::fs::File;
use std::io::{Write, Read};
use std::io;


pub fn comm() {
  // panic!("crash and burn");

  /*
Result 的处理有时太过于繁琐，Rust 提供了一种简洁的处理方式 unwrap 。即，如果成功，直接返回 Result::Ok<T> 中的值，如果失败，则直接调用 !panic，程序结束。
 */
  // 若成功，f则被赋值为文件句柄，失败则结束。
  // let f = File::open("hello.txt").unwrap();

  // expect 是更人性化的处理方式，允许在调用!panic时，返回自定义的提示信息，对调试很有帮助。
  // let f = File::open("hello.txt").expect("Failed to open hello.txt");


}

/*
Result 返回 Ok<T>,  Err<Error> 两种结果
 */
fn create_file() {
  let f = File::create("hello.txt");

  let mut file = match f {
    Ok(file) => file,
    Err(error) => {
      panic!("创建文件错误: {:?}", error)
    }
  };

  match file.write_all(b"Hello world!") {
    Ok(()) => {}
    Err(error) => {
      panic!("写入文件错误: {:?}", error)
    }
  };
}

// 比较复杂点的实现方式
fn read_username_from_file() -> Result<String, io::Error> {
  let f = File::open("hello.txt");

  let mut f = match f {
    Ok(file) => file,
    Err(e) => return Err(e),
  };

  let mut s = String::new();

  match f.read_to_string(&mut s) {
    Ok(_) => Ok(s),
    Err(e) => Err(e),
  }

}

// 更简单的实现方式
fn read_username_from_filex() -> Result<String, io::Error> {
  let mut f = File::open("hello.txt")?;
  let mut s = String::new();

  // todo: 运用 ? 来处理 Ok, Err 两种情况
  /*
  作用是：如果 Result 的值是 Ok，则该表达式返回 Ok 中的值且程序继续执行。如果值是 Error ，则将 Error 的值作为整个函数的返回值，好像使用了 return 关键字一样。这样写，逻辑更为清晰
   */
  f.read_to_string(&mut s)?;
  Ok(s)

  // 也可以使用这种方法
  // let sx = f.read_to_string(&mut s).unwrap();
}















