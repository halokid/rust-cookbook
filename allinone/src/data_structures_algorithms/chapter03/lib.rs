

struct MyStruct {
  a:  u8,
  b:  u8,
  c:  u8,
}

#[cfg(test)]
mod test {
  use std::mem;
  use crate::data_structures_algorithms::chapter03::lib::MyStruct;

  #[test]
  fn check_mem_size() {
    // todo: below will output 3, that means 3 bytes(24 bits)
    println!("size of MyStruct -->>> {}", mem::size_of::<MyStruct>());
    println!("3 * mem::size_of::<u8>() -->>> {}", 3 * mem::size_of::<u8>());

    println!("size of Vec MyStruct 2 items -->>> {}", mem::size_of::<[MyStruct; 2]>());
    println!("3 * mem::size_of::<u8>() * 2 -->>> {}", 3 * mem::size_of::<u8>() * 2);
  }
}


