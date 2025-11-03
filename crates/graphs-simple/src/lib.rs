//! Simple graph implementation.

// #![deny(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

#[cfg(not(feature = "std"))]
extern crate alloc;

pub mod directed;
pub mod generic;
pub mod undirected;

#[doc(inline)]
pub use directed::{DiGraph, LoopedDiGraph, MultiDiGraph, PseudoDiGraph, SimpleDiGraph};
#[doc(inline)]
pub use generic::GenericGraph;
#[doc(inline)]
pub use undirected::{Graph, LoopedGraph, MultiGraph, PseudoGraph, SimpleGraph};

pub(crate) mod parts;
