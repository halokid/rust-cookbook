
fn pass_by_value(_x: String) {}
fn pass_by_ref(_x: &String) {}

fn pass_by_mut_ref(x: &mut String) {
  pass_by_ref(x);
  // pass_by_value(*x);     // todo: 报错cannot move, 函数的参数是比较接收确定类型的， 假如只是传一个指针， 在rust来看是不算确定类型的
}

pub fn comm1() {
  // 拥有
  let name_outer = String::from("Alice");

  let say_hi = || {
    // 强制move
    let name_inner = name_outer;
    println!("Hello, {}", name_inner);
  };

  // 不再拥有name_outer， 请尝试以下操作
  // println!("using name from inner: {}", name_outer);    // error!

  // 但是 name_inner 在 say_hi 中是存活的
  say_hi();     // success
}

pub fn comm2() {
  // owned by main
  let name_outer = String::from("Alice");

  let say_hi = || {
    // use by reference
    let name_inner = &name_outer;
    println!("Hello, {}", name_inner);
  };

  // main still owns name_outer, this is fine
  println!("Using name from main: {}", name_outer); // success

  // but name_inner lives on, in say_hi!
  say_hi(); // success
  say_hi(); // success
}

pub fn comm3() {
  // todo: rust可以用 {} 来规定变量的生命周期范围
  let say_hi = { // forcing the creation of a smaller scope
    // owned by the smaller scope
    let name_outer = String::from("Alice");

    // doesn't work, closure outlives captured values
    // || {
    move || {     // todo: 加入move之后可以正常工作
      // use by reference
      let name_inner = &name_outer;
      println!("Hello, {}", name_inner);
    }
  };

  // syntactically invalid, name_outer isn't in this scope
  //println!("Using name from main: {}", name_outer); // error!

  say_hi();
  say_hi();
}






