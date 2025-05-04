
use rust_plan::basic;

fn main() {
  // ------------------------- Phase 1 ----------------------------
  // let s = "hello".to_string();
  // let sx = basic::phase1_ownership::c1(s);
  //
  // // println!("s -->>> {}", s);
  // println!("sx -->>> {}", sx);
  //
  // basic::phase2_borrowing::c1();
  //
  // let x = "x";
  // let y = "y";
  // let z = basic::phase3_lifetimes::longest(x, y);
  // println!("z -->>> {}", z);
  //
  // basic::advance::c1();
  //
  // basic::advance::test_vec_move();
  // basic::advance::test_borrow_confict();
  //
  // basic::advance2::test_struct_ownership();
  // basic::advance2::test_box_ownership();
  // basic::advance2::test_refcell_ownership();
  // basic::advance2::test_lifetimes();

  basic::advance3::test_ownership_return();
  // ------------------------- Phase 2 ----------------------------
}




