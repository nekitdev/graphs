//! Traits for converting between graph indices and identifiers.

use trait_aliases::trait_aliases;

use crate::{
    base::Base,
    cardinality::{Order, Size},
};

/// The [`node_index`] panic message.
///
/// [`node_index`]: NodeIndexed::node_index
pub const NODE_INDEX: &str = "could not convert node id to index";

/// The [`node_id`] panic message.
///
/// [`node_id`]: NodeIndexed::node_id
pub const NODE_ID: &str = "could not convert node index to id";

/// The [`edge_index`] panic message.
///
/// [`edge_index`]: EdgeIndexed::edge_index
pub const EDGE_INDEX: &str = "could not convert edge id to index";

/// The [`edge_id`] panic message.
///
/// [`edge_id`]: EdgeIndexed::edge_id
pub const EDGE_ID: &str = "could not convert edge index to id";

/// Represents graphs that can convert between node indices and identifiers.
pub trait NodeIndexed: Base {
    /// Returns the node bound of this graph (this is usually the node count).
    fn node_bound(&self) -> usize;

    /// Attempts to convert the given node identifier to its index.
    ///
    /// Returns [`None`] if the conversion could not be performed.
    fn try_node_index(&self, id: Self::NodeId) -> Option<usize>;

    /// Attempts to convert the given node index to its identifier.
    ///
    /// Returns [`None`] if the conversion could not be performed.
    fn try_node_id(&self, index: usize) -> Option<Self::NodeId>;

    /// Converts the given node identifier to its index.
    ///
    /// # Panics
    ///
    /// The default implementation calls [`try_node_index`] and panics on [`None`].
    ///
    /// [`try_node_index`]: Self::try_node_index
    fn node_index(&self, id: Self::NodeId) -> usize {
        self.try_node_index(id).expect(NODE_INDEX)
    }

    /// Converts the given node index to its identifier.
    ///
    /// # Panics
    ///
    /// The default implementation calls [`try_node_id`] and panics on [`None`].
    ///
    /// [`try_node_id`]: Self::try_node_id
    fn node_id(&self, index: usize) -> Self::NodeId {
        self.try_node_id(index).expect(NODE_ID)
    }
}

impl<G: NodeIndexed + ?Sized> NodeIndexed for &G {
    fn node_bound(&self) -> usize {
        (*self).node_bound()
    }

    fn try_node_index(&self, id: Self::NodeId) -> Option<usize> {
        (*self).try_node_index(id)
    }

    fn try_node_id(&self, index: usize) -> Option<Self::NodeId> {
        (*self).try_node_id(index)
    }

    fn node_index(&self, id: Self::NodeId) -> usize {
        (*self).node_index(id)
    }

    fn node_id(&self, index: usize) -> Self::NodeId {
        (*self).node_id(index)
    }
}

impl<G: NodeIndexed + ?Sized> NodeIndexed for &mut G {
    fn node_bound(&self) -> usize {
        (**self).node_bound()
    }

    fn try_node_index(&self, id: Self::NodeId) -> Option<usize> {
        (**self).try_node_index(id)
    }

    fn try_node_id(&self, index: usize) -> Option<Self::NodeId> {
        (**self).try_node_id(index)
    }

    fn node_index(&self, id: Self::NodeId) -> usize {
        (**self).node_index(id)
    }

    fn node_id(&self, index: usize) -> Self::NodeId {
        (**self).node_id(index)
    }
}

/// Represents graphs that can convert between edge indices and identifiers.
pub trait EdgeIndexed: Base {
    /// Returns the edge bound of this graph (this is usually the edge count).
    fn edge_bound(&self) -> usize;

    /// Attempts to convert the given edge identifier to its index.
    ///
    /// Returns [`None`] if the conversion could not be performed.
    fn try_edge_index(&self, id: Self::EdgeId) -> Option<usize>;

    /// Attempts to convert the given edge index to its identifier.
    ///
    /// Returns [`None`] if the conversion could not be performed.
    fn try_edge_id(&self, index: usize) -> Option<Self::EdgeId>;

    /// Converts the given edge identifier to its index.
    ///
    /// # Panics
    ///
    /// The default implementation calls [`try_edge_index`] and panics on [`None`].
    ///
    /// [`try_edge_index`]: Self::try_edge_index
    fn edge_index(&self, id: Self::EdgeId) -> usize {
        self.try_edge_index(id).expect(EDGE_INDEX)
    }

    /// Converts the given edge index to its identifier.
    ///
    /// # Panics
    ///
    /// The default implementation calls [`try_edge_id`] and panics on [`None`].
    ///
    /// [`try_edge_id`]: Self::try_edge_id
    fn edge_id(&self, index: usize) -> Self::EdgeId {
        self.try_edge_id(index).expect(EDGE_ID)
    }
}

impl<G: EdgeIndexed + ?Sized> EdgeIndexed for &G {
    fn edge_bound(&self) -> usize {
        (*self).edge_bound()
    }

    fn try_edge_index(&self, id: Self::EdgeId) -> Option<usize> {
        (*self).try_edge_index(id)
    }

    fn try_edge_id(&self, index: usize) -> Option<Self::EdgeId> {
        (*self).try_edge_id(index)
    }

    fn edge_index(&self, id: Self::EdgeId) -> usize {
        (*self).edge_index(id)
    }

    fn edge_id(&self, index: usize) -> Self::EdgeId {
        (*self).edge_id(index)
    }
}

impl<G: EdgeIndexed + ?Sized> EdgeIndexed for &mut G {
    fn edge_bound(&self) -> usize {
        (**self).edge_bound()
    }

    fn try_edge_index(&self, id: Self::EdgeId) -> Option<usize> {
        (**self).try_edge_index(id)
    }

    fn try_edge_id(&self, index: usize) -> Option<Self::EdgeId> {
        (**self).try_edge_id(index)
    }

    fn edge_index(&self, id: Self::EdgeId) -> usize {
        (**self).edge_index(id)
    }

    fn edge_id(&self, index: usize) -> Self::EdgeId {
        (**self).edge_id(index)
    }
}

/// Represents graphs that are [`NodeIndexed`] and *compact* in terms of node indices.
///
/// Compactness is defined via the following:
///
/// 1. [`Order`] is implemented, and [`order`] matches [`node_bound`].
///
/// 2. There is one-to-one correspondence between graph's node identifiers and the `0..count` range,
///    where `count` is either [`order`] or [`node_bound`], which are equivalent per (1).
///
/// [`order`]: Order::order
/// [`node_bound`]: NodeIndexed::node_bound
pub trait NodeCompact: NodeIndexed + Order {}

/// Represents graphs that are [`EdgeIndexed`] and *compact* in terms of edge indices.
///
/// Compactness is defined via the following:
///
/// 1. [`Size`] is implemented, and [`size`] matches [`edge_bound`]
///
/// 2. There is one-to-one correspondence between graph's edge identifiers and the `0..count` range,
///    where `count` is either [`size`] or [`edge_bound`], which are equivalent per (1).
///
/// [`size`]: Size::size
/// [`edge_bound`]: EdgeIndexed::edge_bound
pub trait EdgeCompact: EdgeIndexed + Size {}

trait_aliases! {
    /// Represents graphs that can convert between both node and edge indices and identifiers.
    ///
    /// Implemented automatically for any graph that is both [`NodeIndexed`] and [`EdgeIndexed`].
    pub trait Indexed = NodeIndexed + EdgeIndexed;

    /// Represents graphs that are compact in terms of both node and edge indices.
    ///
    /// Implemented automatically for any graph that is both [`NodeCompact`] and [`EdgeCompact`].
    pub trait Compact = NodeCompact + EdgeCompact;
}
