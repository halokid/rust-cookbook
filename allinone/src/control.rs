
pub fn comm() {
  // match 可代替 switch case
  let tshirt_width = 20;
  let tshirt_size = match tshirt_width {
    16 => "S",
    17 | 18 => "M",     // 匹配 17 和 18
    19 ... 21 => "L",      // 配置19 到 21
    _ => "不可用的宽度",      // 匹配其他所有
  };
  println!("{}", tshirt_size);


  // while
  let mut a = 1;
  while a <= 10 {
    println!("目前的value: {}", a);
    a += 1;   // Rust不支持++/--自增自减语法
  }

  // loop
  println!("============== loop ================");
  let mut a = 0;
  loop {
    if a == 0 {
      println!("跳过value: {}", a);
      a += 1;
      continue;
    } else if a == 2 {
      println!("跳出value: {}", a);
      break
    }

    println!("当前value: {}", a);
    a += 1;
  }

  // for
  println!("=============== for ================");
  for a in 0..10 {
    println!("当前value: {}", a);
  }

  println!("----------------------------");
  'outer_for: for c1 in 1..6 {
    'inner_for: for c2 in 1..6 {
      println!("当前value: [{}] [{}]", c1, c2);

      if c1 == 2 && c2 == 2 {
        break 'outer_for;     // 结束外层循环
      }

    }
  }

  let group: [&str; 4] = ["mark", "larry", "bill", "steve"];
  for person in group.iter() {
    println!("当前的person: {}", person);
  }
}









