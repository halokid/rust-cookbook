use std::collections::HashMap;

// abstract 1
type Data = HashMap<String, u32>;

// abstract 2
trait Formatter {
  fn format(&self, data: &Data, buf: &mut String);
}

// abstract 3
struct Report;

impl Report {

  fn generate<T: Formatter> (g: T, s: &mut String) {
    let mut data = HashMap::new();
    data.insert("one".to_string(), 1);
    data.insert("two".to_string(), 2);
    g.format(&data, s);
  }
}

struct Text;
// todo: action(Strategy for Object)
impl Formatter for Text {
  fn format(&self, data: &Data, buf: &mut String) {
    for (k, v) in data {
      let entry = format!("{}: {}, ", k, v);
      buf.push_str(&entry);
    }
  }
}

struct Json;
impl Formatter for Json {
  fn format(&self, data: &Data, buf: &mut String) {
    buf.push('[');
    for (k, v) in data.into_iter() {
      let entry = format!(r#"{{"{}": "{}"}}"#, k, v);
      buf.push_str(&entry);
      buf.push(',');
    }
    buf.pop();    // remove extra, at the end
    buf.push(']');
  }
}

pub fn comm() {
  let mut s = String::from("");
  Report::generate(Text, &mut s);
  println!("s -->>> {:?}", s);
}





