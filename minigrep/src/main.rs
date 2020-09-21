use std::env;
use std::fs;
use std::process;

fn main() {
  let args: Vec<String> = env::args().collect();
  println!("{:?}", args);

//  let query = &args[1];
//  let filename = &args[2];
//  let config = parse_config(&args);
//  let config = Config::new(&args);
  let config = Config::new(&args).unwrap_or_else(|err| {
    println!("Problem parsing arguments: {}", err);
    process::exit(1);
  });

  println!("Searching for {}", config.query);
  println!("In file {}", config.filename);

  let contents = fs::read_to_string(config.filename)
                        .expect("Something went wrong reading the file");
  println!("With text:\n{}", contents);
}

struct Config {
  query:      String,
  filename:   String,
}

impl Config {
  fn new(args: &[String]) -> Result<Config, &'static str> {
    if args.len() < 3 {
      return Err("not enouht arguments");
    }

    let query = args[1].clone();
    let filename = args[2].clone();

    Ok(Config { query, filename })
  }
}


//impl Config {
//  fn new(args: &[String]) -> Config {
//    let query = args[1].clone();
//    let filename = args[2].clone();
//
//    Config { query, filename }
//  }
//}


//fn parse_config(args: &[String]) -> (&str, &str) {
//  let query = &args[1];
//  let filename = &args[2];
//
//  (query, filename)
//}

//fn parse_config(args: &[String]) -> Config {
//  let query = args[1].clone();
//  let filename = args[2].clone();
//
//  Config { query, filename }
//}


