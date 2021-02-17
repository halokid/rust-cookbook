mod c1;

static mut SINGLETON_G:  Option<Singleton> = None;

#[derive(Debug)]
struct Singleton {
  v:  usize
}


