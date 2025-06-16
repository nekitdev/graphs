//! Traits for converting between graph indices and identifiers.

use crate::{
    base::Base,
    count::{EdgeCount, NodeCount},
};

/// Represents graphs that can convert between node and edge indices and identifiers.
pub trait NodeIndexed: Base {
    /// Returns the node bound of this graph (this is usually the node count).
    fn node_bound(&self) -> usize;

    /// Converts the given node identifier to its index.
    fn node_index(&self, id: Self::NodeId) -> usize;

    /// Converts the given node index to its identifier.
    fn node_id(&self, index: usize) -> Self::NodeId;
}

impl<G: NodeIndexed + ?Sized> NodeIndexed for &G {
    fn node_bound(&self) -> usize {
        (*self).node_bound()
    }

    fn node_index(&self, id: G::NodeId) -> usize {
        (*self).node_index(id)
    }

    fn node_id(&self, index: usize) -> G::NodeId {
        (*self).node_id(index)
    }
}

impl<G: NodeIndexed + ?Sized> NodeIndexed for &mut G {
    fn node_bound(&self) -> usize {
        (**self).node_bound()
    }

    fn node_index(&self, id: G::NodeId) -> usize {
        (**self).node_index(id)
    }

    fn node_id(&self, index: usize) -> G::NodeId {
        (**self).node_id(index)
    }
}

/// Represents graphs that can convert between edge indices and identifiers.
pub trait EdgeIndexed: Base {
    /// Returns the edge bound of this graph (this is usually the edge count).
    fn edge_bound(&self) -> usize;

    /// Converts the given edge identifier to its index.
    fn edge_index(&self, id: Self::EdgeId) -> usize;

    /// Converts the given edge index to its identifier.
    fn edge_id(&self, index: usize) -> Self::EdgeId;
}

impl<G: EdgeIndexed + ?Sized> EdgeIndexed for &G {
    fn edge_bound(&self) -> usize {
        (*self).edge_bound()
    }

    fn edge_index(&self, id: G::EdgeId) -> usize {
        (*self).edge_index(id)
    }

    fn edge_id(&self, index: usize) -> G::EdgeId {
        (*self).edge_id(index)
    }
}

impl<G: EdgeIndexed + ?Sized> EdgeIndexed for &mut G {
    fn edge_bound(&self) -> usize {
        (**self).edge_bound()
    }

    fn edge_index(&self, id: G::EdgeId) -> usize {
        (**self).edge_index(id)
    }

    fn edge_id(&self, index: usize) -> G::EdgeId {
        (**self).edge_id(index)
    }
}

/// Represents graphs that can convert between both node and edge indices and identifiers.
///
/// This trait is automatically implemented for any type that implements
/// both [`NodeIndexed`] and [`EdgeIndexed`].
pub trait Index: NodeIndexed + EdgeIndexed {}

impl<G: NodeIndexed + EdgeIndexed + ?Sized> Index for G {}

/// Represents graphs that are compact in terms of their node indices.
///
/// Compactness means that the entire `0..self.node_count()` range has no holes.
pub trait NodeCompact: NodeIndexed + NodeCount {}

impl<G: NodeCompact + ?Sized> NodeCompact for &G {}
impl<G: NodeCompact + ?Sized> NodeCompact for &mut G {}

/// Represents graphs that are compact in terms of their edge indices.
///
/// Compactness means that the entire `0..self.edge_count()` range has no holes.
pub trait EdgeCompact: EdgeIndexed + EdgeCount {}

impl<G: EdgeCompact + ?Sized> EdgeCompact for &G {}
impl<G: EdgeCompact + ?Sized> EdgeCompact for &mut G {}

/// Represents graphs that are compact in terms of both their node and edge indices.
///
/// This trait is automatically implemented for any type that implements
/// both [`NodeCompact`] and [`EdgeCompact`].
pub trait Compact: NodeCompact + EdgeCompact {}

impl<G: NodeCompact + EdgeCompact + ?Sized> Compact for G {}
