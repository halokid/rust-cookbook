/*
和元组(tuple)一样，结构体(struct)支持组合不同的数据类型，但不同于元组，结构体需要给每一部分数据命名以标志其含义。因而结构体比元组更加灵活，不依赖顺序来指定或访问实例中的值。
 */

struct User {
  username:       String,
  email:          String,
  sign_in_count:  u64,
  active:         bool,
}

pub fn comm() {
  let mut user1 = User {
    email:            String::from("xx@xx.com"),
    username:         String::from("xx"),
    active:           true,
    sign_in_count:    1,
  };

  user1.email = String::from("yy@xx.com");
  // user1.email = "yy@xx.com".to_string();

}

