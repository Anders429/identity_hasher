//! Provides an [`IdentityHasher`] which passes on the value it is given.
//!
//! This is an incredibly naive hasher. It is not cryptographically secure, it does not prevent DoS
//! attacks, and it can only be used to write a single value of up to 8 bytes. Its main use case is
//! internally on primitive values.
//!
//! An example use case is to store [`u64`]s in a [`HashSet`]. This is done as follows:
//!
//! ```
//! use identity_hasher::IdentityHasher;
//! # #[cfg(use_nested_groups)]
//! use std::{collections::HashSet, hash::BuildHasherDefault};
//! # #[cfg(not(use_nested_groups))]
//! # use std::collections::HashSet;
//! # #[cfg(not(use_nested_groups))]
//! # use std::hash::BuildHasherDefault;
//!
//! let mut set = HashSet::with_hasher(BuildHasherDefault::<IdentityHasher>::default());
//! set.insert(42);
//! set.insert(100);
//!
//! assert_eq!(set.len(), 2);
//! ```

#![no_std]

#[cfg(feature = "doc_item")]
extern crate doc_item;
#[cfg(feature = "serde")]
extern crate serde;
#[cfg(all(feature = "serde", feature = "serde_test"))]
extern crate serde_test;

mod impl_hasher;
#[cfg(feature = "serde")]
mod impl_serde;

#[cfg(feature = "doc_item")]
use doc_item::since;

/// A hasher which passes on the value it is given.
///
/// This hasher acts as an identity function, returning exactly what is written to it. Consequently,
/// this hasher can only be written to a single time. Additionally, no more than 8 bytes may be
/// written to it since the output of a hasher must be 8 bytes.
///
/// Incorrect usage of this hasher will panic on debug builds, but the panics will be optimized away
/// on release builds. Incorrect usage isn't undefined behavior, but it will not function as
/// expected.
///
/// # Examples
/// ```
/// use identity_hasher::IdentityHasher;
/// use std::hash::Hasher;
///
/// let mut hasher = IdentityHasher::default();
/// hasher.write_u64(42);
///
/// assert_eq!(hasher.finish(), 42);
/// ```
///
/// Note that writing multiple times to the hasher is incorrect usage. The following will panic on
/// debug builds and is considered incorrect.
///
/// ``` no_run
/// use identity_hasher::IdentityHasher;
/// use std::hash::Hasher;
///
/// let mut hasher = IdentityHasher::default();
/// hasher.write_u64(42);
/// hasher.write_u32(100);  // Can't write twice!
/// ```
#[cfg_attr(feature = "doc_item", since(content = "1.8.0"))]
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct IdentityHasher {
    hash: u64,
    #[cfg(debug_assertions)]
    used: bool,
}
