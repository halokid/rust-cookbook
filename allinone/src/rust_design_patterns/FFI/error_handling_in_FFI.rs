
// flat enums
enum DatabaseError {
  IsReadOnly = 1,
  IOError = 2,
  FileCorrupter = 3,
}

impl From<DatabaseError> for libc::c_int {
  fn from(e: DatabaseError) -> libc::c_int {
    (e as i8).into()
  }
}

// structured enums
pub mod errors {
  pub(crate) enum DatabaseError {
    IsReadOnly,
    IOError(std::io::Error),
    FileCorrupted(String), // message describing the issue
  }

  impl From<DatabaseError> for libc::c_int {
    fn from(e: DatabaseError) -> libc::c_int {
      match e {
        DatabaseError::IsReadOnly => 1,
        DatabaseError::IOError(_) => 2,
        DatabaseError::FileCorrupted(_) => 3,
      }
    }
  }
}

pub mod c_api {
  use super::errors::DatabaseError;

  #[no_mangle]
  pub extern "C" fn db_error_description(
    e: *const DatabaseError
  ) -> *mut libc::c_char {

    let error: &DatabaseError = unsafe {
      // SAFETY: pointer lifetime is greater than the current stack frame
      &*e
    };

    let error_str: () = match error {
      DatabaseError::IsReadOnly => {
        format!("cannot write to read-only database");
      }
      DatabaseError::IOError(e) => {
        format!("I/O Error: {}", e);
      }
      DatabaseError::FileCorrupted(s) => {
        format!("File corrupted, run repair: {}", &s);
      }
    };

    let c_error = unsafe {
      // SAFETY: copying error_str to an allocated buffer with a NUL
      // character at the end
      let mut malloc: *mut u8 = libc::malloc(error_str.len() + 1) as *mut _;

      if malloc.is_null() {
        return std::ptr::null_mut();
      }

      let src = error_str.as_bytes().as_ptr();

      std::ptr::copy_nonoverlapping(src, malloc, error_str.len());

      std::ptr::write(malloc.add(error_str.len()), 0);

      malloc as *mut libc::c_char
    };

    c_error
  }
}

// custom error types
struct ParseError {
  expected: char,
  line: u32,
  ch: u16
}

impl ParseError { /* ... */ }

/* Create a second version which is exposed as a C structure */
#[repr(C)]
pub struct parse_error {
  pub expected: libc::c_char,
  pub line: u32,
  pub ch: u16
}

impl From<ParseError> for parse_error {
  fn from(e: ParseError) -> parse_error {
    let ParseError { expected, line, ch } = e;
    parse_error { expected: 0, line, ch }
  }
}




