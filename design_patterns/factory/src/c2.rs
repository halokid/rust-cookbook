/*
抽象工厂模式
 */

trait ProductX {
  fn get_value(&self) -> String;
}

trait ProductY {
  fn get_value(&self) -> String;
}

trait AbstractFactory<'a> {
  fn create_product_x(&self) -> Box<dyn ProductX + 'a>;
  fn create_product_y(&self) -> Box<dyn ProductY + 'a>;
}
// -------------------------------------------------------

struct ProductXs(String);
impl ProductXs {
  fn new(msg: String) -> ProductXs {
    ProductXs(msg + &" ProductX".to_string())
  }
}

impl ProductX for ProductXs {
  fn get_value(&self) -> String {
    self.0.clone()
  }
}

// -------------------------------------------------------
struct ProductYs(String);
impl ProductYs {
  fn new(msg: String) -> ProductYs {
    ProductYs(msg + &" ProductY".to_string())
  }
}

impl ProductY for ProductYs {
  fn get_value(&self) -> String {
    self.0.clone()
  }
}

// -----------------------------------------------------
struct FactoryA;
impl <'a> AbstractFactory<'a> for FactoryA {
  fn create_product_x(&self) -> Box<dyn ProductX> {
    Box::new(ProductXs::new("FactoryA".to_string())) as Box<dyn ProductX>
  }

  fn create_product_y(&self) -> Box<dyn ProductY> {
    Box::new(ProductYs::new("FactoryA".to_string())) as Box<dyn ProductY>
  }
}


// -----------------------------------------------------
struct FactoryB;
impl <'a> AbstractFactory<'a> for FactoryB {
  fn create_product_x(&self) -> Box<dyn ProductX> {
    Box::new(ProductXs::new("FactoryB".to_string())) as Box<dyn ProductX>
  }

  fn create_product_y(&self) -> Box<dyn ProductY> {
    Box::new(ProductYs::new("FactoryB".to_string())) as Box<dyn ProductY>
  }
}


enum FactoryID {
  A,
  B,
}

fn create_factory<'a>(id: FactoryID) -> Box<dyn AbstractFactory<'a> + 'a> {
  match id {
    FactoryID::A => Box::new(FactoryA),
    FactoryID::B => Box::new(FactoryB),
  }
}

fn main() {
  let factory_a = create_factory(FactoryID::A);
  let a_x = factory_a.create_product_x();
  let a_y = factory_a.create_product_y();
  println!("{}", a_x.get_value());
  println!("{}", a_y.get_value());

  let factory_b = create_factory(FactoryID::B);
  let b_x = factory_b.create_product_x();
  let b_y = factory_b.create_product_y();
  println!("{}", b_x.get_value());
  println!("{}", b_y.get_value());
}








