use actix::prelude::*;

struct MyActor {
  count:  usize,
}

impl Actor for MyActor {
  type Context = Context<Self>;
}
