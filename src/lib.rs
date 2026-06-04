//! Graph data structures and algorithms.

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![forbid(unsafe_code)]
#![deny(missing_docs)]

pub mod prelude;

#[doc(inline)]
pub use graphs_core as core;

#[cfg(feature = "common")]
#[doc(inline)]
pub use graphs_common as common;

#[cfg(feature = "acyclic")]
#[doc(inline)]
pub use graphs_acyclic as acyclic;

#[cfg(feature = "algorithms")]
#[doc(inline)]
pub use graphs_algorithms as algorithms;

#[cfg(feature = "bits")]
#[doc(inline)]
pub use graphs_bits as bits;

#[cfg(feature = "map")]
#[doc(inline)]
pub use graphs_map as map;

#[cfg(feature = "simple")]
#[doc(inline)]
pub use graphs_simple as simple;

#[cfg(feature = "stable")]
#[doc(inline)]
pub use graphs_stable as stable;

#[cfg(feature = "traversal")]
#[doc(inline)]
pub use graphs_traversal as traversal;

#[cfg(feature = "union-find")]
#[doc(inline)]
pub use graphs_union_find as union_find;

#[cfg(feature = "dot")]
#[doc(inline)]
pub use graphs_dot as dot;
