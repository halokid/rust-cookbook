/*
建造者模式
 */

// todo: 1. 目标类 Person
#[derive(Clone, Debug)]
struct Person {
  name:     String,     // 必须
  age:      u8,         // 必须
  job:      Option<String>,
}

impl Person {
  fn new(name: String, age: u8) -> Person {
    Person {
      // name:     Default::default(),
      name:     name,
      // age:      Default::default(),
      age:      age,
      job:      None,
    }
  }

  fn set_job(&mut self, job: Option<String>) {
    self.job = job
  }
}

// ----------------------------------------------------
// todo: 2. 抽象构建者类
trait Builder {
  // fn  build_name(&mut self);
  // fn  build_age(&mut self);
  fn  build_job(&mut self);
  fn  build(&mut self) -> Person;
}

// ----------------------------------------------------
// todo: 3. 实体构建者类
struct AliceBuilder {
  obj:    Person,        // 实例化
}

impl AliceBuilder {
  fn new() -> AliceBuilder {
    AliceBuilder {
      obj:    Person::new("Alice".to_string(), 12),
    }
  }
}

impl Builder for AliceBuilder {
  fn build_job(&mut self) {
    self.obj.set_job(Some("Student".to_string()))
  }

  fn build(&mut self) -> Person {
    self.obj.clone()
  }
}

// todo: 4. 主导类 Director
struct PersonDirector {
  build:  Box<dyn Builder>,   // todo: 指向抽象构建类
}

impl PersonDirector {
  fn new(b :Box<dyn Builder>) -> PersonDirector {
    PersonDirector {
      build: b,
    }
  }

  fn build(&mut self) -> Person {
    self.build.build_job();     // todo: 这里只有方法调用， 不指定参数
    self.build.build()
  }
}

fn main() {
  // todo:  from struct abstract to trait
  let alice_builder = Box::new(AliceBuilder::new()) as Box<dyn Builder>;

  let mut director = PersonDirector::new(alice_builder);
  let alice = director.build();

  println!("alice: {:?}", alice);
}





