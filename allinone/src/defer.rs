
#[derive(Debug)]
struct Foobar(i32);

impl Drop for Foobar {
  fn drop(&mut self) {
    println!("Dropping a Foobar: {:?}", self);
  }
}

fn main() {
  println!("Before x");
  let _x = Foobar(1);
  println!("After x");
  {
    println!("Before y");
    let _y = Foobar(2);
    println!("After y");
  }
  // todo: 可以起一个变量用作生命周期的监控， 假如生命周期要没了， 那执行某个逻辑， 类似go的defer
  // todo: 这句执行完之后， 程序会回收 _x 的生命周期， 会触发Drop impl
  println!("End of main");
}

/**
输出:
Before x
After x
Before y
After y
Dropping a Foobar: Foobar(2)
End of main
Dropping a Foobar: Foobar(1)
*/

