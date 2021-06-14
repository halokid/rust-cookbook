
use super::SerialNumber;

macro_rules! impl_for_type {
    ($t:ty, $mid:expr) => {
        impl SerialNumber for $t {

            fn sn_add(self, n: Self) -> Option<Self> {
                if n.leading_zeros() >= 1 {
                    Some(self.wrapping_add(n))
                } else {
                    None
                }
            }

            fn sn_lt(self, n: Self) -> bool {
                if self < n && (n - self) < $mid {
                    true
                } else if self > n && (self - n) > $mid {
                    true
                } else {
                    false
                }
            }

        }
    };
}

impl_for_type!(u8, 0x80u8);
impl_for_type!(u16, 0x8000u16);
impl_for_type!(u32, 0x80000000u32);
impl_for_type!(u64, 0x8000000000000000u64);


pub fn comm() {
  // let serial_num = 0xFFFFFFFE.sn_add(1u32);
  // let serial_num = 0x80000000.sn_add(1u64);
  let serial_num = 1u32.sn_add(1u32);
  println!("serial_num ---------- {:?}", serial_num);
}













