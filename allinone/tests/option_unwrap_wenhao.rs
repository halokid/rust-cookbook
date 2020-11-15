use std::io;
use std::io::Read;
use std::fs::File;

/*
这种写法使用了?运算符向调用者返回错误。

作用是：如果 Result 的值是 Ok，则该表达式返回 Ok 中的值且程序继续执行。
如果值是 Error ，则将 Error 的值作为整个函数的返回值，好像使用了 return 关键字一样。这样写，逻辑更为清晰。
 */

fn read_username_from_file() -> Result<String, io::Error> {
  let mut f = File::open("hello.txt")?;
  let mut s = String::new();
  f.read_to_string(&mut s)?;
  Ok(s)     // todo:  Ok可以指定要返回的数据
}


