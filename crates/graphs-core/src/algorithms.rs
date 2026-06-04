//! Algorithms on graphs.
//!
//! For graphs of type `G`, one can write some algorithm implementing the [`Algorithm<G>`] trait,
//! then use the [`Apply`] trait to apply it to said graphs.
//!
//! As an example, here is the implementation of finding cycles in [`Undirected`] graphs,
//! using [`graphs-union-find`]:
//!
//! ```
//! use thiserror::Error;
//!
//! use graphs_core::{
//!     algorithms::{Algorithm, Apply},
//!     base::{Base, Undirected},
//!     connections::{Connection, UndirectedMethods},
//!     indexed::NodeIndexed,
//!     references::EdgeReferences,
//! };
//! use graphs_simple::undirected::Graph;
//! use graphs_union_find::vec::UnionFind;
//!
//! #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Error)]
//! #[error("cycle caused by `{connection}`")]
//! pub struct Cycle<C: Connection> {
//!     pub connection: C,
//! }
//!
//! impl<C: Connection> Cycle<C> {
//!     pub const fn new(connection: C) -> Self {
//!         Self { connection }
//!     }
//! }
//!
//! pub type CycleIn<G> = Cycle<<G as Base>::Connection>;
//!
//! pub type Output<G> = Option<CycleIn<G>>;
//!
//! pub struct UndirectedCycles;
//!
//! impl<G: Undirected + NodeIndexed + EdgeReferences> Algorithm<G> for UndirectedCycles {
//!     type Output = Output<G>;
//!
//!     fn perform(&mut self, graph: G) -> Self::Output {
//!         let mut edge_sets = UnionFind::new(graph.node_bound());
//!
//!         for edge in graph.edge_references() {
//!             let connection = edge.connection();
//!
//!             let (one, two) = connection.parts();
//!
//!             let one_index = graph.node_index(one);
//!             let two_index = graph.node_index(two);
//!
//!             // union `one_index` and `two_index`
//!             if edge_sets.union(one_index, two_index).already() {
//!                 // if they were already in the same set, `connection` causes the cycle
//!                 return Some(Cycle::new(connection));
//!             }
//!         }
//!
//!         None
//!     }
//! }
//! ```
//!
//! [`Undirected`]: crate::base::Undirected
//!
//! [`graphs-union-find`]: https://docs.rs/graphs-union-find

use crate::base::Base;

/// Represents algorithms on graphs of type `G`.
pub trait Algorithm<G: Base> {
    /// The associated type for the algorithm output.
    type Output;

    /// Performs [`Self`] on the given graph.
    fn perform(&mut self, graph: G) -> Self::Output;
}

impl<G: Base, A: Algorithm<G>> Algorithm<G> for &mut A {
    type Output = A::Output;

    fn perform(&mut self, graph: G) -> Self::Output {
        (*self).perform(graph)
    }
}

/// Represents graphs that algorithms can be applied to.
pub trait Apply: Base {
    /// Applies the given algorithm to the graph.
    ///
    /// Use [`by_ref`] or [`by_mut`] to apply the algorithm by reference or mutable reference,
    /// respectively, instead of taking ownership.
    ///
    /// [`by_ref`]: crate::by::By::by_ref
    /// [`by_mut`]: crate::by::By::by_mut
    fn apply<A: Algorithm<Self>>(self, mut algorithm: A) -> A::Output
    where
        Self: Sized,
    {
        algorithm.perform(self)
    }
}

impl<G: Base + ?Sized> Apply for G {}
