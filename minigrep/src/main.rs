use std::env;
use std::process;

use minigrep::Config;
use minigrep::run;

fn main() {
  let args: Vec<String> = env::args().collect();
  println!("{:?}", args);

  let config = Config::new(&args).unwrap_or_else(|err| {
    println!("Problem parsing arguments: {}", err);
    process::exit(1);
  });

  println!("Searching for {}", config.query);
  println!("In file {}", config.filename);

//  run(config);      // config返回是一个Result， 如果不处理Result返回的Err，会产生一个告警
  if let Err(e) = run(config) {
    println!("Application error: {}", e);
    process::exit(1);
  }
}




