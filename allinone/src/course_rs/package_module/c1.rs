

pub mod front_of_house {
  pub mod hosting {
    pub fn add_to_waitlist() {}

    pub fn seat_at_table() {}
  }

  pub mod serving {
    pub fn take_order() {}

    pub fn serve_order() {}

    pub fn take_payment() {}
  }
}

pub fn eat_at_restaurant() {
  // todo: absolute path
  crate::course_rs::package_module::c1::front_of_house::hosting::add_to_waitlist();

  // todo: relative path
  front_of_house::hosting::add_to_waitlist();
}

fn serve_order() {
  self::back_of_house::cook_order();
}

mod back_of_house {
  fn fix_incorrect_order() {
    super::serve_order();
  }

  pub fn cook_order() {}
}
