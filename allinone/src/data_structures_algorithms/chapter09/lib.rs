
use std::collections::binary_heap;

pub fn bubble_sort<T: PartialOrd + Clone>(collection: &[T]) -> Vec<T> {
  let mut result: Vec<T> = collection.into();
  for _ in 0..result.len() {
    let mut swaps = 0;
    for i in 1..result.len() {
      if result[i - 1] > result[i] {
        result.swap(i - 1, i);
        swaps += 1;
      }
    }
    if swaps == 0 {
      break;
    }
  }
  result
}




