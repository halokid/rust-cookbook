

#[derive(Debug)]
struct Frame {
  width: u32,
  height: u32,
}

#[derive(Debug)]
enum ParseError {
  TooFewArgs,
  TooManyArgs,
  InvalidInteger(String),
}

fn parse_args() -> Result<Frame, ParseError> {
  use self::ParseError::*;

  let mut args = std::env::args().skip(1);

  let width_str = match args.next() {
    None => return Err(TooFewArgs),
    Some(width_str) => width_str,
  };

  let height_str = match args.next() {
    None => return Err(TooFewArgs),
    Some(height_str) => height_str,
  };

  match args.next() {
    Some(_) => return Err(TooManyArgs),
    None => (),
  }

  let width = match width_str.parse() {
    Err(_) => return Err(InvalidInteger(width_str)),
    Ok(width) => width,
  };

  let height = match height_str.parse() {
    Err(_) => return Err(InvalidInteger(height_str)),
    Ok(height) => height,
  };

  Ok(Frame {
    width,
    height,
  })
}



