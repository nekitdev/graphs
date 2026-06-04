//! Simple graph implementation.

// #![deny(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

#[cfg(not(feature = "std"))]
extern crate alloc;

pub mod directed;
pub mod errors;
pub mod generic;
pub mod indices;
pub mod parts;
pub mod prelude;
pub mod references;
pub mod undirected;

pub(crate) mod internals;
