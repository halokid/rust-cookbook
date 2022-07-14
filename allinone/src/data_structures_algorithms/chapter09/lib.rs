
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

pub fn shell_sort<T: PartialOrd + Clone>(collection: &[T]) -> Vec<T> {
  let n = collection.len();
  let mut gap = n / 2;
  let mut result: Vec<T> = collection.into();

  while gap > 0 {
    // todo: `i` first equal `gap`
    for i in gap..n {
      let temp = result[i].clone();

      // todo: `j` first equal `i`
      let mut j = i;
      while j >= gap && result[j - gap] > temp {
        result[j] = result[j - gap].clone();
        j -= gap;
      }
      result[j] = temp;
    }
    gap /= 2
  }
  result
}

pub fn heap_sort<T: PartialOrd + Clone + Ord>(collection: &[T]) -> Vec<T> {
  let mut heap = binary_heap::BinaryHeap::new();
  for c in collection {
    heap.push(c.clone());
  }
  heap.into_sorted_vec()
}





