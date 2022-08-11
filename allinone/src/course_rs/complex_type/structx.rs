

#[derive(Debug)]
struct User {
  active:       bool,
  username:     String,
  email:        String,
  sign_in_count:   u64,
}

fn build_user(email: String, username: String) -> User {
  User {
    active: true,
    username,
    email,
    sign_in_count: 1
  }
}

// ----------------------------------------------------

// #[derive(Debug)]
// struct Userx {
//   username:  &str,
//   email:     &str,
//   sign_in_count:    u64,
//   active:    bool,
// }

pub fn comm() {
  let user1 = build_user(String::from("user1@xx.com"), String::from("user1"));
  println!("user1 -->>> {:?}", user1);

  let user2 = User{
    email: "user2@xx.com".to_string(),
    ..user1
  };

  println!("user2 -->>> {:?}", user2);
  dbg!(&user2);
  println!("user2 -->>> {:#?}", user2);
}











