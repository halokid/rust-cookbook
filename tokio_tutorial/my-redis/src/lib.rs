use bytes::Bytes;

mod mutex_counter;

#[derive(Debug)]
enum Command {
  Get {
    key: String,
  },

  Set {
    key: String,
    value: Bytes,
  }
}



