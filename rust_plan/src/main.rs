
use rust_plan::basic;
use rust_plan::basic2;
use rust_plan::basic3;

fn c1() {
  // - Basic Phase 1: Understand ownership, borrowing, and lifecycle -----
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

  // basic::advance3::test_ownership_return();
  // basic::advance3::test_lifetimes_struct();

  // - Basic Phase 2: Pratice trait、enum、pattern matching -------------
  // basic2::shape::c1();
  // basic2::shape::c2();
  // basic2::challenge::c1();
  // basic2::challenge2::c1();
  // basic2::challenge2::c1();
  // basic2::challenge3::c1();
  // basic3::c1::c1();
}

#[tokio::main]
async fn main() {
  // basic3::c1::c1().await;

  // basic3::phase1::c1::c1().await;

  // basic3::phase2::c1::c11().await;
  // basic3::phase2::c1::c12().await;
  // basic3::phase2::concurrent_crawler_simulator::c1().await;

  basic3::phase3::echo_server::c1().await.expect("echo server start fail");
}







