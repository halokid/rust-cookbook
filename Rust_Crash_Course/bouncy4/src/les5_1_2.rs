
fn double(x: &mut u32) {
  *x *= 2
}

fn do_doublt() {
  let mut x = 5;
  double(&mut x);
  println!("x: {}", x);
}

fn do_iter() {
  let nums = vec![1, 2, 3, 4, 5];
  for i in nums {
    println!("i: {}", i);
  }
}

fn do_iter2() {
  for i in 1..3 {
    let nums = vec![1, 2, 3, 4, 5];
    for j in nums {
      println!("i: {}, j: {}", i, j);
    }
  }
}

/*
fn do_iter3() {
  // todo: 这倒是有点道理。 第一次运行外循环时，我们将 nums 值移动到内循环中。 然后，我们不能在第二次通过循环时再次使用 nums 值。 好吧，合乎逻辑。
  let nums = vec![1, 2, 3, 4, 5];
  for i in 1..3 {
    for j in nums {
      println!("{}, {}", i, j);
    }
  }
}
 */

fn do_iter4() {
  let nums = vec![1, 2, 3, 4, 5];
  for i in 1..3 {
    for j in &nums {
      // let _: u32 = j;       // todo: 验证 j 是什么类型
      let _: &u32 = j;
      println!("{}, {}", i, j);
    }
  }
}

fn do_iter5() {
  let mut nums = vec![1, 2, 3, 4, 5];
  for i in 1..3 {
    for j in &mut nums {
      let _: &mut u32 = j;
      println!("{}, {}", i, j);
      *j *= 2;
    }
  }
}

/*
fn do_iter6() {
  let nums = vec![1, 2, 3, 4, 5];
  for i in 1..3 {
    for j in nums.into_iter() {
      println!("{}, {}", i, j);
    }
  }
}
 */

pub fn comm() {
  // do_iter();

  // do_iter2();

  // do_iter3();

  // do_iter4();

  do_iter5();
}








