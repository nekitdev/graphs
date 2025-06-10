//! Graph data structures and algorithms.

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

#[cfg(feature = "alloc")]
extern crate alloc;

pub use graphs_core as core;

#[cfg(feature = "simple")]
pub use graphs_simple as simple;
