use std::cell::RefCell;
use std::rc::Rc;
use crate::data_structures_algorithms::chapter05::lib::IoTDevice;

type BareTree = Rc<RefCell<Node>>;
type Tree = Option<BareTree>;

#[derive(Clone, Debug, PartialEq)]
enum Color {
  Red,
  Black,
}

#[derive(PartialEq)]
enum RBOperation {
  LeftNode,
  RightNode,
}

#[derive(PartialEq)]
enum Rotation {
  Left,
  Right,
}

struct Node {
  pub color: Color,
  pub dev:   IoTDevice,
  pub parent: Tree,
  left: Tree,
  right: Tree,
}

impl PartialEq for Node {
  fn eq(&self, other: &Node) -> bool {
    self.dev == other.dev
  }
}










