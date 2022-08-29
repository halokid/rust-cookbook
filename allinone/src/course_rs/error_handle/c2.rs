use std::fs::File;
use std::io::ErrorKind;

pub fn comm() {
  let f = File::open("heloo.txt");

  let f = match f {
    Ok(file) => file,

    // todo: match different error type
    Err(error) => match error.kind()  {
      ErrorKind::NotFound => match File::create("hello.txt") {
        Ok(fc) => fc,
        Err(e) => panic!("Problem creating the file: {:?}", e)
      },
      other_errpor => panic!("Problem opening the file: {:?}", other_errpor),
    },
  };

  // -----------------------------------------------------
  // todo: direct panic
  // let f = File::open("xx.txt").unwrap();
  // todo: reaload the error information
  let f = File::open("xx.txt").expect("Failed to open xx.txt");
}


