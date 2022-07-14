
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

pub fn merge_sort<T: PartialOrd + Clone>(collection: &[T]) -> Vec<T> {
  if collection.len() > 1 {
    let (l, r) = collection.split_at(collection.len() / 2);
    let sorted_l = merge_sort(l);
    let sorted_r = merge_sort(r);

    let mut result: Vec<T> = collection.into();
    let (mut i, mut j) = (0, 0);
    let mut k = 0;
    while i < sorted_l.len() && j < sorted_r.len() {
      if sorted_l[i] <= sorted_r[j] {
        result[k] = sorted_l[i].clone();
        i += 1;
      } else {
        result[k] = sorted_r[j].clone();
        j += 1;
      }
      k += 1;
    }

    while i < sorted_l.len() {
      result[k] = sorted_l[i].clone();
      k += 1;
      i += 1;
    }

    while j < sorted_r.len() {
      result[k] = sorted_r[j].clone();
      k += 1;
      j += 1;
    }

    result
  } else {
    collection.to_vec();
  }
}






