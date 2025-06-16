//! Traits for graphs that can return neighbors of nodes.

use crate::{
    base::Base,
    degree::{Class, Degrees},
    direction::{Direction, Incoming, Outgoing},
};

/// Represents graphs that can return neighbors from the given node.
pub trait Neighbors: Base {
    /// The associated type for neighbor iterators.
    type Iterator<'n>: Iterator<Item = Self::NodeId>
    where
        Self: 'n;

    /// Returns the neighbors of the given `node`.
    fn neighbors(&self, node: Self::NodeId) -> Self::Iterator<'_>;
}

impl<G: Neighbors + ?Sized> Neighbors for &G {
    type Iterator<'n>
        = G::Iterator<'n>
    where
        Self: 'n;

    fn neighbors(&self, node: G::NodeId) -> Self::Iterator<'_> {
        (*self).neighbors(node)
    }
}

impl<G: Neighbors + ?Sized> Neighbors for &mut G {
    type Iterator<'n>
        = G::Iterator<'n>
    where
        Self: 'n;

    fn neighbors(&self, node: G::NodeId) -> Self::Iterator<'_> {
        (**self).neighbors(node)
    }
}

/// Represents graphs that can return neighbors from the given node in the given direction.
pub trait DirectedNeighbors: Neighbors {
    /// The associated type for directed neighbor iterators.
    type DirectedIterator<'n>: Iterator<Item = Self::NodeId>
    where
        Self: 'n;

    /// Returns the neighbors from the given `node` in the given `direction`.
    fn directed_neighbors(
        &self,
        node: Self::NodeId,
        direction: Direction,
    ) -> Self::DirectedIterator<'_>;

    /// Returns the [`Outgoing`] neighbors from the given `node`.
    fn outgoing_neighbors(&self, node: Self::NodeId) -> Self::DirectedIterator<'_> {
        self.directed_neighbors(node, Outgoing)
    }

    /// Returns the [`Incoming`] neighbors from the given `node`.
    fn incoming_neighbors(&self, node: Self::NodeId) -> Self::DirectedIterator<'_> {
        self.directed_neighbors(node, Incoming)
    }

    /// Returns the [`Degrees`] of the given `node`.
    fn degree(&self, node: Self::NodeId) -> Degrees {
        Degrees::new(self.outgoing_degree(node), self.incoming_degree(node))
    }

    /// Returns the [`Class`] of the given `node`.
    fn class(&self, node: Self::NodeId) -> Class {
        Class::compute(self.has_outgoing(node), self.has_incoming(node))
    }

    fn outgoing_degree(&self, node: Self::NodeId) -> usize {
        self.outgoing_neighbors(node).count()
    }

    fn incoming_degree(&self, node: Self::NodeId) -> usize {
        self.incoming_neighbors(node).count()
    }

    fn has_outgoing(&self, node: Self::NodeId) -> bool {
        self.outgoing_neighbors(node).peekable().peek().is_some()
    }

    fn has_incoming(&self, node: Self::NodeId) -> bool {
        self.incoming_neighbors(node).peekable().peek().is_some()
    }
}

impl<G: DirectedNeighbors + ?Sized> DirectedNeighbors for &G {
    type DirectedIterator<'n>
        = G::DirectedIterator<'n>
    where
        Self: 'n;

    fn directed_neighbors(
        &self,
        node: G::NodeId,
        direction: Direction,
    ) -> Self::DirectedIterator<'_> {
        (*self).directed_neighbors(node, direction)
    }
}

impl<G: DirectedNeighbors + ?Sized> DirectedNeighbors for &mut G {
    type DirectedIterator<'n>
        = G::DirectedIterator<'n>
    where
        Self: 'n;

    fn directed_neighbors(
        &self,
        node: G::NodeId,
        direction: Direction,
    ) -> Self::DirectedIterator<'_> {
        (**self).directed_neighbors(node, direction)
    }
}
