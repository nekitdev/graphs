//! Simple graph implementation.

// #![deny(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

#[cfg(not(feature = "std"))]
extern crate alloc;

pub mod digraph;
pub mod errors;
pub mod generic;
pub mod graph;

pub use digraph::{DiGraph, LoopedDiGraph, MultiDiGraph, PseudoDiGraph, SimpleDiGraph};
pub use generic::Generic;
pub use graph::{Graph, LoopedGraph, MultiGraph, PseudoGraph, SimpleGraph};

pub(crate) mod internal;
