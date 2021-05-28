use std::path::PathBuf;
use std::io::{Error as IoError, Read};
use std::fs::File;

// todo: 用结构体来表示一种状态
#[derive(Debug)]
struct DivisionByZeroError;     // 该结构指示除法运算不成功

fn safe_division(dividend: f64, divisor: f64) -> Result<f64, DivisionByZeroError> {

  if divisor == 0.0 {
    Err(DivisionByZeroError)
  } else {
    Ok(dividend / divisor)
  }
}

pub fn comm() {
  println!("{:?}", safe_division(9.0, 3.0));
  println!("{:?}", safe_division(4.0, 0.0));
  println!("{:?}", safe_division(0.0, 2.0));
}


// -------------------------------------------------------------
fn read_file_contents(path: PathBuf) -> Result<String, IoError> {
  let mut string = String::new();

  // todo #1: handle this match expression
  let mut file: File = match File::open(path) {
    Ok(file_handle) => file_handle,
    Err(io_error) => return Err(io_error),
  };

  // todo #2: handle this error
  match file.read_to_string(&mut string) {
    Ok(_) => (),
    Err(io_error) => return Err(io_error),
  }

  // todo #3: return the `string` varible as expected by this function signature
  Ok(string)
}

pub fn comm2() {
  assert!(read_file_contents(PathBuf::from("src/main.rs")).is_ok());
  assert!(read_file_contents(PathBuf::from("non-exists-file.txt")).is_err());
}









