use crate::data_structures_algorithms::chapter05::lib::IoTDevice;

struct Node {
  pub dev: IoTDevice,
  left:   Tree,
  righst: Tree,
}

impl Node {
  pub fn new(dev: IoTDevice) -> Tree {
    Some(Box::new(
      Node {
        dev,
        left: None,
        righst: None
      }
    ))
  }
}

type Tree = Option<Box<Node>>;

pub struct DeviceRegistry {
  root: Tree,
  pub length: u64,
}

impl DeviceRegistry {
  pub fn new_empty() -> DeviceRegistry {
    DeviceRegistry {
      root: None,
      length: 0
    }
  }

  fn add_rec(&mut self, node: Tree, device: IoTDevice) -> Tree {
    match node {
      Some(mut n) => {
        if n.dev.numerical_id <= device.numerical_id {
          n.left = self.add_rec(n.left, device)
        } else {
          n.righst = self.add_rec(n.righst, device)
        }
        Some(n)
      }
      _ => Node::new(device)
    }
  }

  pub fn find(&self, numerical_id: u64) -> Option<IoTDevice> {
    self.find_r(&self.root, numerical_id)
  }

  fn find_r(&self, node: &Tree, numerical_id: u64) ->
     Option<IoTDevice> {
    match node {
      Some(n) => {
        if n.dev.numerical_id == numerical_id {
          Some(n.dev.clone())
        } else if n.dev.numerical_id < numerical_id {
          self.find_r(&n.left, numerical_id)
        } else {
          self.find_r(&n.righst, numerical_id)
        }
      }
      _ => None
    }
  }

  pub fn walk(&self, callback: impl Fn(&IoTDevice) -> ()) {
    self.walk_in_order(&self.root, &callback)
  }

  fn walk_in_order(&self, node: &Tree, callback: &impl Fn(&IoTDevice)-> ()) {
    if let Some(n) = node {
      self.walk_in_order(&n.left, callback);
      callback(&n.dev);
      self.walk_in_order(&n.righst, callback);
    }
  }
}





