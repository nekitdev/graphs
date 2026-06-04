//! Traits for graphs that can return neighbors of nodes.

use crate::{
    base::{Base, Directed},
    direction::Direction::{self, Incoming, Outgoing},
};

/// Represents graphs that can return neighbors of the given node.
pub trait Neighbors: Base {
    /// The associated type for neighbor iterators.
    type NodeIterator<'g>: Iterator<Item = Self::NodeId>
    where
        Self: 'g;

    /// Returns the neighbors of the given node.
    fn neighbors(&self, node: Self::NodeId) -> Self::NodeIterator<'_>;

    /// Checks whether the node has neighbors.
    ///
    /// The default implementation calls [`neighbors`] and checks whether the iterator is non-empty.
    ///
    /// Implementors of this trait should provide more optimized implementations if possible.
    ///
    /// [`neighbors`]: Self::neighbors
    fn has_neighbors(&self, node: Self::NodeId) -> bool {
        self.neighbors(node).next().is_some()
    }
}

impl<G: Neighbors + ?Sized> Neighbors for &G {
    type NodeIterator<'g>
        = G::NodeIterator<'g>
    where
        Self: 'g;

    fn neighbors(&self, node: Self::NodeId) -> Self::NodeIterator<'_> {
        (*self).neighbors(node)
    }

    fn has_neighbors(&self, node: Self::NodeId) -> bool {
        (*self).has_neighbors(node)
    }
}

impl<G: Neighbors + ?Sized> Neighbors for &mut G {
    type NodeIterator<'g>
        = G::NodeIterator<'g>
    where
        Self: 'g;

    fn neighbors(&self, node: Self::NodeId) -> Self::NodeIterator<'_> {
        (**self).neighbors(node)
    }

    fn has_neighbors(&self, node: Self::NodeId) -> bool {
        (**self).has_neighbors(node)
    }
}

/// Represents graphs that can return neighbors of the given node in the given direction.
pub trait DirectedNeighbors: Directed + Neighbors {
    /// The associated type for directed neighbor iterators.
    type DirectedNodeIterator<'g>: Iterator<Item = Self::NodeId>
    where
        Self: 'g;

    fn directed_neighbors(
        &self,
        direction: Direction,
        node: Self::NodeId,
    ) -> Self::DirectedNodeIterator<'_>;

    /// Returns the neighbors *outgoing* from the given node.
    fn outgoing_neighbors(&self, node: Self::NodeId) -> Self::DirectedNodeIterator<'_> {
        self.directed_neighbors(Outgoing, node)
    }

    /// Returns the neighbors *incoming* to the given node.
    fn incoming_neighbors(&self, node: Self::NodeId) -> Self::DirectedNodeIterator<'_> {
        self.directed_neighbors(Incoming, node)
    }

    /// Checks whether the node has *outgoing* neighbors.
    ///
    /// The default implementation calls [`outgoing_neighbors`] and checks whether
    /// the iterator is non-empty.
    ///
    /// Implementors of this trait should provide more optimized implementations if possible.
    ///
    /// [`outgoing_neighbors`]: Self::outgoing_neighbors
    fn has_outgoing_neighbors(&self, node: Self::NodeId) -> bool {
        self.outgoing_neighbors(node).next().is_some()
    }

    /// Checks whether the node has *incoming* neighbors.
    ///
    /// The default implementation calls [`incoming_neighbors`] and checks whether
    /// the iterator is non-empty.
    ///
    /// Implementors of this trait should provide more optimized implementations if possible.
    ///
    /// [`incoming_neighbors`]: Self::incoming_neighbors
    fn has_incoming_neighbors(&self, node: Self::NodeId) -> bool {
        self.incoming_neighbors(node).next().is_some()
    }
}

impl<G: DirectedNeighbors + ?Sized> DirectedNeighbors for &G {
    type DirectedNodeIterator<'g>
        = G::DirectedNodeIterator<'g>
    where
        Self: 'g;

    fn directed_neighbors(
        &self,
        direction: Direction,
        node: Self::NodeId,
    ) -> Self::DirectedNodeIterator<'_> {
        (*self).directed_neighbors(direction, node)
    }

    fn outgoing_neighbors(&self, node: Self::NodeId) -> Self::DirectedNodeIterator<'_> {
        (*self).outgoing_neighbors(node)
    }

    fn incoming_neighbors(&self, node: Self::NodeId) -> Self::DirectedNodeIterator<'_> {
        (*self).incoming_neighbors(node)
    }

    fn has_outgoing_neighbors(&self, node: Self::NodeId) -> bool {
        (*self).has_outgoing_neighbors(node)
    }

    fn has_incoming_neighbors(&self, node: Self::NodeId) -> bool {
        (*self).has_incoming_neighbors(node)
    }
}

impl<G: DirectedNeighbors + ?Sized> DirectedNeighbors for &mut G {
    type DirectedNodeIterator<'g>
        = G::DirectedNodeIterator<'g>
    where
        Self: 'g;

    fn directed_neighbors(
        &self,
        direction: Direction,
        node: Self::NodeId,
    ) -> Self::DirectedNodeIterator<'_> {
        (**self).directed_neighbors(direction, node)
    }

    fn outgoing_neighbors(&self, node: Self::NodeId) -> Self::DirectedNodeIterator<'_> {
        (**self).outgoing_neighbors(node)
    }

    fn incoming_neighbors(&self, node: Self::NodeId) -> Self::DirectedNodeIterator<'_> {
        (**self).incoming_neighbors(node)
    }

    fn has_outgoing_neighbors(&self, node: Self::NodeId) -> bool {
        (**self).has_outgoing_neighbors(node)
    }

    fn has_incoming_neighbors(&self, node: Self::NodeId) -> bool {
        (**self).has_incoming_neighbors(node)
    }
}
