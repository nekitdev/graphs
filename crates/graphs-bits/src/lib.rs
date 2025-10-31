#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

#[cfg(feature = "alloc")]
#[macro_use]
extern crate alloc;

pub mod block;
pub mod capacity;

#[cfg(any(feature = "alloc", feature = "std"))]
pub mod set;

#[cfg(any(feature = "alloc", feature = "std"))]
pub mod vec;

pub use block::{BitBlock, DefaultBlock};
pub use capacity::{Bits, Blocks, Capacity};

#[cfg(any(feature = "alloc", feature = "std"))]
pub use set::BitSet;

#[cfg(any(feature = "alloc", feature = "std"))]
pub use vec::BitVec;
