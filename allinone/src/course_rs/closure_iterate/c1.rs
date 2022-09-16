use std::collections::HashMap;
use std::thread;
use std::time::Duration;

fn workout(intensity: u32, random_number: u32) {
  // todo: this is closure!
  let action = || {
    println!("muuuu......");
    thread::sleep(Duration::from_secs(2));
    intensity
  };

  if intensity < 25 {
    println!("-->>> 1 {}", action());
    println!("-->>> 2 {}", action());
  } else if random_number == 3 {
    println!("-->>> 3");
  } else {
    println!("-->>> 4 {}", action());
  }
}

pub fn comm() {
  let intensity = 10;
  let random_number = 7;
  workout(intensity, random_number);

  let arr = [1, 2, 3];
  for v in arr.into_iter() {
    println!("{}", v);
  }

  let v1 = vec![1, 2, 3];
  // todo: iterator is Lazy initialize
  let v1_iter = v1.iter();

  for val in v1_iter {
    println!("{}", val);
  }

  // -------------------------------------------------
  let values = vec![1, 2, 3];

  for v in values.into_iter() {
    println!("{}", v)
  }

  // 下面的代码将报错，因为 values 的所有权在上面 `for` 循环中已经被转移走
  // println!("{:?}",values);

  let values = vec![1, 2, 3];
  let _values_iter = values.iter();

  // 不会报错，因为 values_iter 只是借用了 values 中的元素
  println!("{:?}", values);

  let mut values = vec![1, 2, 3];
  // 对 values 中的元素进行可变借用
  let mut values_iter_mut = values.iter_mut();

  // 取出第一个元素，并修改为0
  if let Some(v) = values_iter_mut.next() {
    *v = 0;
  }

  // 输出[0, 2, 3]
  println!("{:?}", values);

  // ------------------------------------------------
  let v1 = vec![1, 2, 3];
  let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
  println!("{:?}", v2);

  // -----------------------------------------------
  let names = ["subface", "subfei"];
  let ages = [18, 18];
  let folks: HashMap<_, _> = names.into_iter()
    .zip(ages.into_iter()).collect();
  println!("{:?}", folks);
}











