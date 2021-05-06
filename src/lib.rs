#![no_std]

#[cfg(feature = "doc_item")]
extern crate doc_item;
#[cfg(feature = "serde")]
extern crate serde;

mod impl_hasher;
#[cfg(feature = "serde")]
mod impl_serde;

#[cfg(feature = "doc_item")]
use doc_item::since;

#[cfg_attr(feature = "doc_item", since(content = "1.8.0"))]
#[derive(Clone, Debug, Default)]
pub struct IdentityHasher {
    hash: u64,
    #[cfg(debug_assertions)]
    used: bool,
}
