
use std::env::args;
use std::option::Option::Some;
use std::env::Args;

pub fn comm() {
  let mut args = args();
  loop {
    match args.next() {
      None => return,
      Some(arg) => println!("{}", arg),
    }
  }
}


pub fn comm2() {
  println!("--------------------");
  let mut args = args();
  while let Some(arg) = args.next() {
    println!("{}", arg);
  }
}

// ------------------------------------------------------

#[derive(Debug)]
struct Frame {
  width:        u32,
  height:       u32,
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

  /*
  match args.next() {
    None => Err(TooFewArgs),    // Err 封装一个enum类型
    Some(height_str) => {

    }
  }
   */

  // 第一个参数是width
  let width_str = match args.next() {
    None => return Err(TooFewArgs),
    Some(width_str) => width_str,
  };

  // 第二个参数是height
  let height_str = match args.next() {
    None => return Err(TooFewArgs),
    Some(height_str) => height_str,
  };

  // 第三个参数
  match args.next() {
    Some(_) => return Err(TooManyArgs),   // 直接返回太多参数的错误提示
    None => (),     // 假如没有第三个参数则忽略
  };

  let width = match width_str.parse() {
    Err(_) => return Err(InvalidInteger(width_str)),  // 假如width_str为string类型的话，则匹配到Err
    Ok(width) => width,
  };

  let height = match height_str.parse() {
    Err(_) => return Err(InvalidInteger(height_str)),
    Ok(height) => height,
  };

  Ok(Frame{
    width,
    height,
  })
}

// ------------ 更好的写法 ---------------------

// todo: 可以返回问号的函数
fn require_arg(args: &mut Args) -> Result<String, ParseError> {
  match args.next() {
    None => Err(ParseError::TooFewArgs),
    Some(s) => Ok(s),
  }
}

fn require_no_args(args: &mut Args) -> Result<(), ParseError> {
  match args.next() {
    Some(_) => Err(ParseError::TooManyArgs),
    None => Ok(()),
  }
}

fn parse_u32(s: String) -> Result<u32, ParseError> {
  match s.parse() {
    Err(_) => Err(ParseError::InvalidInteger(s)),
    Ok(x) => Ok(x),
  }
}


fn parse_args2() -> Result<Frame, ParseError> {
  let mut args = std::env::args();

  let _commend_name = require_arg(&mut args);  // todo: 这里可以不加问号, 前导下划线表示“我知道我在做什么，我不关心这个值
  let _commend_name = require_arg(&mut args)?;  // todo: 当然也可以加上问号

  let width_str = require_arg(&mut args)?;  // todo: 加问号函数返回的作用
  let height_str = require_arg(&mut args)?;

  require_no_args(&mut args);   // 不需要更多参数了

  let width = parse_u32(width_str)?;
  let height = parse_u32(height_str)?;

  Ok(Frame{ width, height })
}


// ----------- 更抽象的写法 ------------------

struct ParseArgs(std::env::Args);

impl ParseArgs {
  fn new() -> ParseArgs {
    unimplemented!()
  }

  fn require_arg(&mut self) -> Result<String, ParseError> {
    match self.0.next() {    // todo: self.0 是指 ParseArgs() 第一个类型， ParseArgs是一个tuple
      None => Err(ParseError::TooFewArgs),
      Some(s) => Ok(s),
    }
  }

  fn require_no_args(&mut self) -> Result<(), ParseError> {
    unimplemented!()
  }
}

fn parse_args3() -> Result<Frame, ParseError> {
  let mut args = ParseArgs::new();
  args.require_arg()?;  // skip the command name

  let width_str = args.require_arg()?;
  let height_str = args.require_arg()?;
  args.require_no_args()?;

  let width = parse_u32(width_str)?;
  let height = parse_u32(height_str)?;

  Ok(Frame { width, height })
}


pub fn comm3() {
  println!("{:?}", parse_args3());
}




