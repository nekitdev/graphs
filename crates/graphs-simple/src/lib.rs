//! Simple graph implementation.

// #![deny(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

#[cfg(feature = "alloc")]
extern crate alloc;

use cfg_if::cfg_if;

pub(crate) mod at_most_two;

cfg_if! {
    if #[cfg(any(feature = "std", feature = "alloc"))] {
        pub(crate) mod internal;

        pub mod graph;

        pub use graph::{DiGraph, Graph, UnGraph};
    }
}

pub mod error;
pub mod index;

pub use error::{Error, ErrorKind};
pub use index::{EdgeIndex, NodeIndex, edge_index, node_index};

pub(crate) mod next;
