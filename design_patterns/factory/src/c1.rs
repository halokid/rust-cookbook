/*
工厂方法模式
 */

trait Product {
  fn convert(&self, _: String) -> String;
}

trait Factory {
  fn create_product(&self) -> Box<dyn Product>;

  // 不直接实例化Product, 再执行convert, 而是把这个逻辑封装到一个方法里面，直接调用自身的方法
  fn convert(&self, s: String) -> String {
    self.create_product().convert(s)
  }
}

// ------------------------------------------------
struct ProductX;
impl Product for ProductX {
  fn convert(&self, s: String) -> String {
    println!("ProductX convert: {}", s.to_uppercase());     // 具体逻辑为转大写
    return s.to_uppercase()
  }
}

struct ProductY;
impl Product for ProductY {
  fn convert(&self, s: String) -> String {
    println!("ProductY convert: {}", s.to_lowercase());     // 具体逻辑为转小写
    return s.to_lowercase()
  }
}

// -------------------------------------------------
struct FactoryAll;
impl Factory for FactoryAll{
  fn create_product(&self) -> Box<dyn Product> {
    // Box::new(ProductX) as Box<dyn Product>      // factory for prtoduct X
    Box::new(ProductY) as Box<dyn Product>      // factory for prtoduct Y
  }
}


fn main() {
  let f = FactoryAll;
  println!("{}", f.convert("HaloKid".to_string()))
}





