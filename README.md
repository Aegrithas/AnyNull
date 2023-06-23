# AnyNull

A simple Rust utility crate for unifying various "kinds" of null. Instead of `ptr::null()`, `ptr::null_mut()`, `None::<&_>`, etc., now all forms of null can be called just `null()`.