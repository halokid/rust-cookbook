
mod aaa {
  // const X: i32 = 10;

  fn print_aaa() {
    println!("{}", 42);
  }

  mod bbb {
    fn print_bbb() {
      println!("{}", 37);
    }
  }
}


pub mod ccc {
  pub fn print_ccc() {
    println!("{}", 25);
  }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}


