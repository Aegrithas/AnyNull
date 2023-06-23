
#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

use std::ptr::{self, NonNull};
use std::rc::{Rc, Weak as WeakRc};
use std::sync::{Arc, Weak as WeakArc};

/// Any type that can be some form of null.
pub trait Nullable {
  
  /// Whatever value of Self means null.
  /// 
  /// For `*const _`, this is `std::ptr::null()`.
  /// 
  /// For `*mut _`, this is `std::ptr::null_mut()`.
  /// 
  /// See also [`NoneIsNull`]
  const NULL: Self;
  
  /// Just a null check without relying on [`PartialEq`] (i.e., `ptr == null()`).
  /// 
  /// For `*const _` and `*mut _`, this uses the inherent method of the same name.
  fn is_null(&self) -> bool;
  
}

/// Conveniently wraps arounnd [`<P as Nullable>::NULL`](Nullable::NULL).
pub const fn null<P: Nullable>() -> P {
  P::NULL
}

/// Wraps around [`<P as Nullable>::is_null(ptr)`](Nullable::is_null)
pub fn is_null<P: Nullable>(ptr: &P) -> bool {
  ptr.is_null()
}

impl<T> Nullable for *const T {
  
  const NULL: Self = ptr::null();
  
  fn is_null(&self) -> bool {
    <*const T>::is_null(*self)
  }
  
}

impl<T> Nullable for *mut T {
  
  const NULL: Self = ptr::null_mut();
  
  fn is_null(&self) -> bool {
    <*mut T>::is_null(*self)
  }
  
}

impl<T: NoneIsNull> Nullable for Option<T> {
  
  const NULL: Self = None;
  
  fn is_null(&self) -> bool {
    self.is_none()
  }
  
}

/// Many types are not themselves nullable,
/// but when wrapped in an [`Option`] effectively use [`None`] as null
/// (especially with the niche-filling optimization).
/// 
/// This includes references, [`NonNull`], and most smart pointers
/// (including all of the smart pointers in [`std`]).
pub trait NoneIsNull: Sized {}

impl<T: ?Sized> NoneIsNull for NonNull<T> {}

impl<T: ?Sized> NoneIsNull for Box<T> {}

impl<T: ?Sized> NoneIsNull for Rc<T> {}

impl<T: ?Sized> NoneIsNull for WeakRc<T> {}

impl<T: ?Sized> NoneIsNull for Arc<T> {}

impl<T: ?Sized> NoneIsNull for WeakArc<T> {}

impl<'a, T: ?Sized> NoneIsNull for &'a T {}

impl<'a, T: ?Sized> NoneIsNull for &'a mut T {}