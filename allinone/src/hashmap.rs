use std::collections::HashMap;

pub fn comm() {
  let mut scores = HashMap::new();

  scores.insert(String::from("Blue"), 10);
  scores.insert(String::from("Yellow"), 50);

  let team_name = String::from("Blue");
  let score = scores.get(&team_name);
  // println!("score: {}", score?);    // 报错: cannot use the `?` operator in a function that returns `()`
  println!("score: {}", score.unwrap());
}