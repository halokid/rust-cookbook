

pub fn comm() {
  let s1 = String::from("abcd");
  let s2 = "xyz";

  let result = longest(s1.as_str(), s2);
  println!("result -->>> {}", result);
}

fn longest(x: &str, y: &str) -> &str {
  if x.len() > y.len() {
    x
  } else {
    y
  }
}