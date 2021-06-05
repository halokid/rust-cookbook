
trait AsJson {
  fn as_json(&self) -> String;
}

// 可以编写一个函数，该函数接受任何实现 AsJson 特征的类型。 其编写形式为 impl 后跟一组特征边界
// todo: impl AsJson 是 AsJson 的一个实现，接收人和一个符合这个实现特征的类型都可以
fn send_data_as_json(value: &impl AsJson) {
  println!("Sending json data to server...");
  println!("-> {}", value.as_json());
  println!("Done!");
}

// todo: 或者用下面这个方式来定义参数是AsJson特征的类型
fn send_data_as_jsonx<T:  AsJson>(value: &T) {
  println!("X --- Sending json data to server...");
  println!("-> {}", value.as_json());
  println!("Done!");
}

// todo: AsJson的特征实现
struct Person {
  name:   String,
  age:    u8,
  favorite_fruit: String,
}

struct Dog {
  name:   String,
  color:  String,
  likes_petting:  bool,
}

impl AsJson for Person {
  fn as_json(&self) -> String {
    format!(
      r#"{{ "type": "person", "name": "{}", "age": {}, "favoriteFruit": "{}" }}"#,
      self.name, self.age, self.favorite_fruit
    )
  }
}

impl AsJson for Dog {
  fn as_json(&self) -> String {
    format!(
      r#"{{ "type": "dog", "name": "{}", "color": "{}", "likesPetting": {} }}"#,
      self.name, self.color, self.likes_petting
    )
  }
}

pub fn comm() {
  let laura = Person {
    name: "Laura".to_string(),
    age: 31,
    favorite_fruit: "apples".to_string()
  };

  let fido = Dog {
    name: "Fido".to_string(),
    color: "Block".to_string(),
    likes_petting: false
  };

  send_data_as_json(&laura);
  send_data_as_jsonx(&fido);

  let kitty = Cat {
    name: "Kitty".to_string(),
    sharp_claws: false,
  };
  // send_data_as_json(&kitty);
}

struct Cat {
  name:   String,
  sharp_claws:  bool,
}









