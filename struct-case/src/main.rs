

struct User {
  username: String,
  email: String,
  sign_in_count: u64,
  active: bool,
}

fn main() {

  let mut user1 = User {
    email:          String::from("xx@xx.com"),
    username:       String::from("lv"),
    active:         true,
    sign_in_count:  1,
  };
  user1.email = String::from("yy@yy.com");

  println!("Hello, world!");
}


fn build_user(email: String, username: String) -> User {
  User {
    email:          email,
    username:       username,
    active:         true,
    sign_in_count:  1,
  }
}