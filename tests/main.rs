use any_null::*;

use std::ptr::{self, NonNull};

#[test]
fn main() {
  const A: *const () = null();
  assert_eq!(A, ptr::null());
  const IS_A_NULL: bool = is_null(&A);
  assert!(IS_A_NULL);
  const B: *mut () = null();
  assert_eq!(B, ptr::null_mut());
  const C: Option<NonNull<()>> = null();
  assert_eq!(C, None);
  assert_eq!(C, NonNull::new(ptr::null_mut()));
  const D: Option<Box<()>> = null();
  assert_eq!(D, None);
  const E: Option<&'static mut ()> = null();
  assert_eq!(E, None);
}