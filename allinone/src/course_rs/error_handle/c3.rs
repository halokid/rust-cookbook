use std::fs::File;
use std::{fs, io};
use std::io::Read;

pub fn read_username_from_file() -> Result<String, io::Error> {
  let f = File::open("hello.txt");

  let mut f = match f {
    Ok(file) => file,
    Err(e) => {
      return Err(e);
    }
  };

  let mut s = String::new();
  match f.read_to_string(&mut s) {
    Ok(_) => Ok(s),
    Err(e) => Err(e),
  }
}


// todo: ---------------- simplify code -----------------
// pub fn read_username_from_file_x() -> Result<String, io::Error> {
// pub fn read_username_from_file_x() -> Result<String, std::error::Error> {
pub fn read_username_from_file_x() -> Result<String, Box<dyn std::error::Error>> {
  let mut f = File::open("hello.txt")?;
  let mut s = String::new();
  f.read_to_string(&mut s)?;
  Ok(s)
}

// todo: ---------------- more simplify code --------------
pub fn read_username_from_file_y() -> Result<String, io::Error> {
  let mut s = String::new();
  File::open("hello.txt")?.read_to_string(&mut s)?;
  Ok(s)
}

// todo: ---------------- most simplify code ----------------
pub fn read_username_from_file_z() -> Result<String, io::Error> {
  fs::read_to_string("hello.txt")
}





