use std::fmt::Debug;

pub fn linear_search<T: Eq + Clone>(haystack: &[T], needle: &T) -> Option<usize> {
  for (i, h) in haystack.iter().enumerate() {
    if h.eq(needle) {
      return Some(i)
    }
  }
  None
}

pub fn jump_search<T: Eq + PartialOrd + Clone + Debug>(
  haystack: &[T],
  needle:   &T,
  jump_size: usize,
) -> Option<usize> {
  if jump_size < haystack.len() {
    let mut i = 0;
    while i < haystack.len() - 1 {
      if i + jump_size < haystack.len() {
        i += jump_size;
      } else {
        i = haystack.len() - 1;
      }

      if &haystack[i] == needle {
        return Some(i);
      } else if &haystack[i] > needle {
        // todo: find section by section, i to jump_size
        // todo: every section length is jump_size.
        return linear_search(&haystack[i - jump_size..i], needle)
      }
    }
  }
  None
}