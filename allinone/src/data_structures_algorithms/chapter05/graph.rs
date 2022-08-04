/*
use std::cmp::Ordering;

type KeyType = u64;

enum TentativeWeight {
  Infinite,
  Number(u32),
}

impl Eq for TentativeWeight {}

impl PartialEq<Self> for TentativeWeight {
  fn eq(&self, other: &Self) -> bool {
    todo!()
  }
}

impl PartialOrd<Self> for TentativeWeight {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for TentativeWeight {
  fn cmp(&self, other: &TentativeWeight) -> Ordering {
    match other {
      TentativeWeight::Infinite => match self {
        TentativeWeight::Infinite => Ordering::Equal,
        _ => Ordering::Less,
      },

      TentativeWeight::Number(o) => match self {
        TentativeWeight::Infinite => Ordering::Greater,
        TentativeWeight::Number(s) => s.cmp(o),
      }
    }
  }
}

#[derive(Clone, Debug)]
struct Edge {
  weight:     u32,
  node:       usize,
}

fn min_index(weights: &Vec<TentativeWeight>, nodes: &Vec<usize>) -> usize {
  let mut min_weight = (weights[0].clone(), 0);
  for node in nodes.iter() {
    if let Some(n) = weights.get(*node) {
      if n < &min_weight.0 {
        min_weight = ((&weights[*node]).clone(), node.clone());
      }
    }
  }
  return min_weight.1;
}
 */
