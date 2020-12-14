
#[derive(Debug)]
struct Person {
  name: String,
  age: u32,
}

// todo: 参数是Person的引用， 假如要修改， 需要标识为mut
fn birthday_immutable(person: &mut Person) {
  person.age += 1;
}

fn birthday_mutable<'a>(mut person: &'a mut Person, replacement: &'a mut Person) {
  person = replacement;     // todo: 仅仅只是指针改变了， person根本没改变， 所以x还是输出alice结构体
  person.age += 1;
}

pub fn comm() {
  let mut alice = Person { name: String::from("Alice"), age: 30 };
  let mut bob = Person { name: String::from("Bob"), age: 20 };
  println!("Alice 1: {:?}, Bob 1: {:?}", alice, bob);

  birthday_immutable(&mut alice);
  println!("Alice 2: {:?}, Bob 2: {:?}", alice, bob);

  birthday_mutable(&mut alice, &mut bob);   // x
  println!("Alice 3: {:?}, Bob 3: {:?}", alice, bob);
}