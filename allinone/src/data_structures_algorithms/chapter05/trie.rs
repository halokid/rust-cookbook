use std::collections::HashMap;

type Link = Box<Node>;

struct Node {
  pub key:  char,
  next:  HashMap<char, Link>,
  pub value:  Option<iotde>
}