
// todo: 这里是一个mod
mod math {
  type Complex = (f64, f64);
  pub fn sin(f: f64) { }
  pub fn cos(f: f64) { }
  pub fn tan(f: f64) { }

  // 私有
  struct Foo;

  // 公有
  pub struct Bar{
    // 私有
    field:  i32,

    // 公有
    pub name:   String,
  }

  pub enum State {
    PublicAccessibleVariant,
    PublicAccessibleVariant2,
  }
}

pub fn comm() {
  let mx = math::cos(45.0);
  // let bar = math::Bar{ name: "".to_string() };
}

mod authentication {

  #[derive(Debug)]
  pub struct User {
    username:   String,
    password_hash:  u64,
  }

  impl User {
    pub fn new(username: &str, password: &str) -> User {
      User {
        username: username.to_string(),
        password_hash: hash_password(password),
      }
    }

    // todo: 可以用专门的getter, setter方法来获取 username, password
    pub fn get_username(&self) -> &String {
      &self.username
    }

    // todo: 因为这里要更改 User 的 password， 所以第一个参数是 &mut self
    pub fn set_password(&mut self, password: &str) {
      self.password_hash = hash_password(password)
    }
  }

  fn hash_password(input: &str) -> u64 { 29 }

  impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "{}, {}", self.username, self.password_hash)
    }
  }
}

pub fn comm2() {
  let mut user = authentication::User::new("halokid", "pass");
  println!("user ------ {}", user);
  // println!("user username -------- {}", user.username);
  user.set_password("xx");
  let username = user.get_username();
  println!("user x ------ {}", username);
}


// --------------------------------------------------------
mod text_process {
  pub mod letters {
    pub fn count_letters(text:  &str) -> usize {
      text.chars().filter(| ref c | c.is_alphabetic()).count()
    }
  }

  pub mod numbers {
    pub fn count_numbers(text:  &str) -> usize {
      text.chars().filter(| ref c | c.is_numeric()).count()
    }
  }
}

fn count_letters_and_numbers(text: &str) -> (usize, usize) {
  let number_of_letters = text_process::letters::count_letters(text);
  let number_of_numbers = text_process::numbers::count_numbers(text);
  (number_of_letters, number_of_numbers)
}

pub fn comm3() {
  println!("{:?}", count_letters_and_numbers("221B Baker Street"));
}






