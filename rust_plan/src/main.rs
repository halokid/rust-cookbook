
use rust_plan::basic;

fn main() {
  // ------------------------- Phase 1 ----------------------------
  let s = "hello".to_string();
  let sx = basic::phase1_ownership::c1(s);

  // println!("s -->>> {}", s);
  println!("sx -->>> {}", sx);

  basic::phase2_borrowing::c1();

  let x = "x";
  let y = "y";
  let z = basic::phase3_lifetimes::longest(x, y);
  println!("z -->>> {}", z);

  basic::advance::c1();
  // ------------------------- Phase 2 ----------------------------
}
