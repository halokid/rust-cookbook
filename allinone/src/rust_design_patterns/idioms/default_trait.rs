use std::path::PathBuf;
use std::time::Duration;

#[derive(Default, Debug, PartialEq)]
struct MyConfiguration {
  output: Option<PathBuf>,
  search_path: Vec<PathBuf>,
  timeout:  Duration,
  check:  bool,
}

impl MyConfiguration {
  
}

pub fn comm() {
  let mut conf = MyConfiguration::default();
  conf.check = true;
  println!("conf -->>> {:#?}", conf);

  let conf1 = MyConfiguration {
    check: true,
    ..Default::default()
  };
  assert_eq!(conf, conf1);
}