/*
过滤器模式
 */

// todo: 1. 创建标准类
struct Person {
  name:             String,
  gender:           String,
  marital_status:   String,
}
impl Person {
  fn new(name: String, gender: String, marital_status: String) -> Person {
    Person {
      name:       name,
      gender:     gender,
      marital_status: marital_status,
    }
  }

  fn get_gender(&self) -> String {
    format!(self.gender)
    // self.gender    // todo: 不可以转移所有权， 不能这么写
  }
}

// todo: 2. 创建一个标准类的接口
trait FilterCriteria {
  // fn get_males(ps: Box<[Person]>) -> Box<[Person]> {}
  fn get_males(ps: [Person; 2]) -> [Person; 1];
}

// todo: 3. 创建实现标准类接口的实体类
struct MaleFilter;

/*
impl MaleFilter {
  fn new() -> Box<dyn FilterCriteria> {
    // MaleFilter
    Box::new(MaleFilter) as dyn FilterCriteria
  }
}
*/

impl FilterCriteria for MaleFilter {
  fn get_males(ps: [Person; 2]) -> [Person; 1] {
    let mut psx: [Person; 1] = [Person; 1];
    let i = 0;
    for index in 0..2 {
      println!("{}", index);
      if ps[index].get_gender() == "male" {
        psx[i] = *ps[index]
      }
    }
    psx
  }
}


// todo: 4. 根据过滤条件来组合person数据
fn main() {
  let mut ps: [Person; 4] = [Person; 4];
  ps[0] = Person::new("Mike".to_string(), "male".to_string(), "Single".to_string());
  ps[1] = Person::new("Diana".to_string(), "Female".to_string(), "Single".to_string());

  // let male = MaleFilter::new();
  let male = Box::new(MaleFilter) as Box<dyn FilterCriteria>;
  male.get_males(ps)
}





