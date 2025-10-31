#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(not(feature = "std"))]
#[macro_use(vec)]
extern crate alloc;

pub mod rank;
pub mod vec;

pub use rank::{Rank, ZERO};
pub use vec::UnionFind;
