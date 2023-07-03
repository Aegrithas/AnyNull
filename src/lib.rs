#![feature(ptr_metadata)]
#![feature(const_trait_impl)]
#![feature(const_ptr_is_null)]

#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

use std::ptr::{self, NonNull};
use std::rc::{Rc, Weak as WeakRc};
use std::sync::{Arc, Weak as WeakArc};
use std::ptr::Thin;

/**
 * Any type that can be some form of null.
 */
#[const_trait]
pub trait Nullable {
  /**
   * Whatever value of Self means null.
   * 
   * For `*const _`, this is `std::ptr::null()`.
   * 
   * For `*mut _`, this is `std::ptr::null_mut()`.
   * 
   * See also [`NoneIsNull`]
   */
  fn null() -> Self;
  
  /**
   * Just a null check without relying on [`PartialEq`] (i.e., `ptr == null()`).
   *
   * For `*const _` and `*mut _`, this uses the inherent method of the same name.
   */
  fn is_null(&self) -> bool;
  
}

/**
 * Conveniently wraps arounnd [`<P as Nullable>::null()`](Nullable::null).
 */
pub const fn null<P: ~const Nullable>() -> P {
  P::null()
}

/**
 * Wraps around [`<P as Nullable>::is_null(ptr)`](Nullable::is_null)
 */
pub const fn is_null<P: ~const Nullable>(ptr: &P) -> bool {
  ptr.is_null()
}

impl<T: Thin + ?Sized> const Nullable for *const T {
  
  fn null() -> Self {
    ptr::null()
  }
  
  fn is_null(&self) -> bool {
    <*const T>::is_null(*self)
  }
  
}

impl<T: Thin + ?Sized> const Nullable for *mut T {
  
  fn null() -> Self {
    ptr::null_mut()
  }
  
  fn is_null(&self) -> bool {
    <*mut T>::is_null(*self)
  }
  
}

impl<T: NoneIsNull> const Nullable for Option<T> {
  
  fn null() -> Self {
    None
  }
  
  fn is_null(&self) -> bool {
    self.is_none()
  }
  
}
/**
 * Many types are not themselves nullable,
 * but when wrapped in an [`Option`] effectively use [`None`] as null
 * (especially with the niche-filling optimization).
 * 
 * This includes references, [`NonNull`], and most smart pointers
 * (including all of the smart pointers in [`std`]).
 */
pub trait NoneIsNull: Sized {}

impl<T: ?Sized> NoneIsNull for NonNull<T> {}

impl<T: ?Sized> NoneIsNull for Box<T> {}

impl<T: ?Sized> NoneIsNull for Rc<T> {}

impl<T: ?Sized> NoneIsNull for WeakRc<T> {}

impl<T: ?Sized> NoneIsNull for Arc<T> {}

impl<T: ?Sized> NoneIsNull for WeakArc<T> {}

impl<'a, T: ?Sized> NoneIsNull for &'a T {}

impl<'a, T: ?Sized> NoneIsNull for &'a mut T {}