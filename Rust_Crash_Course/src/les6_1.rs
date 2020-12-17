
#[derive(Debug)]
struct Person {
  name: Option<String>,
  age: Option<u32>,
}

fn print_person(mut person: Person) {
  // match person.name {     // todo: 不是借用的话， 这样肯定会获取person的所有权
  //   Some(name) => println!("Name is {}", name),
  //   None => println!("No name provided"),
  // }

  // match &person.name {
  //   Some(name) => println!("Name is {}", name),
  //   None => println!("No name provided"),
  // }

  // todo: 但是，我们可以使用 ref 关键字更明确地说明这一点。 这个关键词表明，当模式匹配时，我们希望模式是一个引用，而不是原始值的移动。
  match person.name {
    Some(ref name) => println!("Name is {}", name),
    None => println!("No name provided"),
  }

  // match person.age {
  //   Some(age) => println!("Age is {}", age),
  //   None => println!("No age provided"),
  // }

  match person.age {
    Some(ref mut age) => {
      println!("Age is {}", age);
      *age += 1;
    }
    None => println!("No age provided"),
  }

  /*
  注意最后一行，age 仍然是30岁，而不是31岁。 花一分钟时间试着理解这里发生了什么... 完成了吗？ 酷。
根据 person.age 模式匹配
如果是Some，我们需要将 age 移动到局部 age
但是由于类型是 u32，它将创建一个副本并移动该副本
当我们递增 age 时，我们递增了一个从未使用过的副本
   */

  // person.name = None;    // todo: 上面用 &person借用的话， 就不用转移person的所有权了
  println!("Full person: {:?}", person);
}

pub fn comm() {
  print_person(Person {
    name: Some(String::from("Alice")),
    age: Some(30),
  });

}




