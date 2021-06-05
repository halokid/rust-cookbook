/// This function divides two numbers.
///
/// # Example #1
///
/// ```
/// let result = basic_math::doc_test::div(10, 2);
/// assert_eq!(result, 5);
/// ```
///
/// # Example #2
///
/// ```
/// let result = basic_math::doc_test::div(6, 3);
/// assert_eq!(result, 2);
/// ```
///
/// # Panics
///
/// The function panics if the second argument is zero.
///
/// ```rust,should_panic
/// // panics on division by zero
/// basic_math::doc_test::div(10, 0);
/// ```
pub fn div(a: i32, b: i32) -> i32 {
  if b == 0 {
    panic!("Devide-by-zero error");
  }
  a / b
}

///
/// # Example #1
///
/// ```
/// let result = basic_math::doc_test::sub(9, 2);
/// assert_eq!(result, 7);
/// ```
///
/// # Example #2
///
/// ```
/// let result = basic_math::doc_test::sub(6, 9);
/// assert_eq!(result, -3);
/// ```
pub fn sub(a: i32, b: i32) -> i32 {
  a - b
}



