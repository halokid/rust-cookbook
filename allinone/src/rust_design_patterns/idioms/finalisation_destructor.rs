use futures::future::ok;

fn bar() -> Result<(), ()> {
  struct Foo;

  impl Drop for Foo {
    fn drop(&mut self) {
      println!("Foo exit");
    }
  }

  let _exit = Foo;
  baz()?;   // todo: if baz() return Err, this will return immediately, if return true, will process next
  println!("-->>> run here means baz() return true.");
  Ok(())
}

fn baz() -> Result<(), ()> {
  Ok(())
  // Err(())
}

pub fn comm() -> Result<(), ()> {
  bar()
}