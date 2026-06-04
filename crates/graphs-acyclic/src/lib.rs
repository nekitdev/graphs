//! Acyclic graphs.
//!
//! Since *undirected* acyclic graphs are either *trees* or *forests*
//! depending on the connected components, it makes only so much sense to support them,
//! therefore this crate focuses on *directed* acyclic graphs (DAGs).

#[cfg(not(feature = "std"))]
extern crate alloc;

pub mod acyclic;
pub mod order_map;

#[doc(inline)]
pub use acyclic::Acyclic;
