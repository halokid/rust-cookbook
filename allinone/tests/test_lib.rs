use allinone::plus;
use allinone::crate_mod;



#[test]
fn add_two() {
  assert_eq!(4, plus(2, 2));

  crate_mod::comm();
}