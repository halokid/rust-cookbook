
// trait Iterator {
//   type Item;
//   fn next(&mut self) -> Option<self::Item>;
// }


// todo: 实现自己的迭代器
#[derive(Debug)]
struct Counter {
  length:   usize,
  count:    usize,
}

impl Counter {
  fn new(length: usize) -> Counter {
    Counter {
      length,
      count: 0
    }
  }
}

impl Iterator for Counter {
  type Item = usize;

  fn next(&mut self) -> Option<Self::Item> {
    self.count += 1;
    if self.count <= self.length {
     Some(self.count)
    } else {
      None
    }
  }
}

pub fn comm() {
  let mut counter = Counter::new(6);
  println!("Counter created: {:?}", counter);

  assert_eq!(counter.next(), Some(1));
  assert_eq!(counter.next(), Some(2));

  println!("Counter now is: {:?}", counter);

  for number in Counter::new(10) {
    println!("number ---------- {}", number);
  }

  let sum_util_10: usize = Counter::new(10).sum();
  println!("sum_util_10 ---------- {}", sum_util_10);

  let powers_of_2:  Vec<usize> = Counter::new(8).map(
    |n| 2usize.pow(n as u32) ).collect();
  println!("powers_of_2 -------- {:?}", powers_of_2);

  let p: u128 = 2_u128.pow(128) - 1;
  // println!("p ---------- {}", p);
  // 真实中的加密参数
  let g: u128 = 113;
  let a_private_key = 19; //保密
  let a_send_to_b_num = pow_mod(g, a_private_key, p);
  println!("真实环境中的A发送的值：{}", a_send_to_b_num);
}

// x^y mod p 如何计算 13^15679 mod 458
fn pow_mod(x: u128, y: u128, p: u128) -> u128 {
  if y == 0 {
    return 1;
  } else {
    let z = pow_mod(x, y / 2, p); //如果y=5_u128, y/2=2
    if y % 2 == 0 {
      return ((z % p) * (z % p)) % p;
    } else {
      return ((((x % p) * (z % p)) % p) * (z % p)) % p;
    }
  }
}











