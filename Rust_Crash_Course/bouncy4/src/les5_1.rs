
#[derive(Debug)]
struct Person {
  name:     String,
  age:      u32,
}

// todo: 直接传struct实体， 可以直接定义属性， 不用标识为mut
fn birthday_immutable(person: Person) -> Person {
  Person {
    name:     person.name,
    age:      person.age + 1,
  }
}

fn birthday_mutable(mut person: Person) -> Person {
  person.age += 1;
  person
}

pub fn comm() {
  let alice1 = Person { name: String::from("Aclie"), age: 30 };
  println!("Alice 1: {:?}", alice1);

  let alice2 = birthday_immutable(alice1);
  println!("Alice 2: {:?}", alice2);

  let alice3 = birthday_mutable(alice2);
  println!("Alice 3: {:?}", alice3);
}