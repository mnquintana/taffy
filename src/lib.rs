#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]
#![deny(unsafe_code)]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

// We always need std for the tests
// See <https://github.com/la10736/rstest/issues/149#issuecomment-1156402989>
#[cfg(all(test, not(feature = "std")))]
#[macro_use]
extern crate std;

#[cfg(all(not(feature = "std"), feature = "alloc"))]
extern crate alloc;

#[cfg_attr(feature = "serde", macro_use)]
#[cfg(feature = "serde")]
extern crate serde;

pub mod error;
pub mod geometry;
pub mod layout;
pub mod math;
pub mod node;
pub mod prelude;
pub mod style;

mod flexbox;
mod forest;
#[cfg(all(not(feature = "alloc"), not(feature = "std")))]
mod indexmap;
mod sys;

pub use crate::node::Taffy;
