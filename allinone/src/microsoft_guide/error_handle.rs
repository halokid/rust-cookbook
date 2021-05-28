
pub fn comm() {
  let fruits = vec!["banana", "apple", "coconut", "orange"];

  let first = fruits.get(0);
  println!("first ---- {:?}", first);

  let third = fruits.get(2);
  println!("third ---- {:?}", third);

  let non_exist = fruits.get(99);
  println!("non_exist ----- {:?}", non_exist);

  // todo: &index 就是索引值
  for &index in [0, 1, 99].iter() {
    match fruits.get(index) {
      // todo: 强调apple很棒, Some里面是匹配  &&str
      Some(&"apple") => println!("apple is awesome"),

      Some(fruit_name) => println!("the fruit is {}", fruit_name),
      None => println!("there is no fruit"),
    }
  }
}

pub fn comm2() {
  let some_number: Option<u8> = Some(7);

  // todo: 1
  match some_number {
    Some(7) => println!("this is a lucky number 1"),
    _ => {},
  }

  // todo: 2 上面的表达式等价于下面
  if let Some(7) = some_number {
    println!("this is a lucky number 2");
  }

  assert_eq!(Some("dog").unwrap_or("cat"), "dog");
  assert_eq!(None.unwrap_or("cat"), "cat")      // true

}


// ---------------------------------------------------------------------

struct Person {
  first:    String,
  middle:   Option<String>,
  last:     String,
}

fn build_full_name(person: &Person) -> String {
  let mut full_name = String::new();
  full_name.push_str(&person.first);
  full_name.push_str(" ");
  // todo: 下面的都是错误的
  // let middle = &person.middle.unwrap_or(" ".to_string());
  // let x = &person.middle.as_ref();
  // full_name.push_str(x.unwrap());

  // todo: 要从 Option 取得所有权的方法如下
  // todo: &&String 得到的类型就是 string
  if let Some(middle) = &person.middle {
    full_name.push_str(&middle);
    full_name.push_str(" ");
  }

  full_name.push_str(&person.last);
  full_name
}

pub fn comm3() {
  let john = Person {
    first: "James".to_string(),
    middle: Some(String::from("Oliver")),
    last: "Smith".to_string()
  };
  assert_eq!(build_full_name(&john), "James Oliver Smith")

}

