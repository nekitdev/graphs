use crate::{
    base::{Base, Directed},
    degree::{Class, Degrees},
    direction::{Direction, Incoming, Outgoing},
};

pub trait Edges: Base {
    type Iterator<'e>: Iterator<Item = Self::EdgeId>
    where
        Self: 'e;

    fn edges(&self, node: Self::NodeId) -> Self::Iterator<'_>;
}

impl<G: Edges + ?Sized> Edges for &G {
    type Iterator<'e>
        = G::Iterator<'e>
    where
        Self: 'e;

    fn edges(&self, node: Self::NodeId) -> Self::Iterator<'_> {
        (*self).edges(node)
    }
}

impl<G: Edges + ?Sized> Edges for &mut G {
    type Iterator<'e>
        = G::Iterator<'e>
    where
        Self: 'e;

    fn edges(&self, node: Self::NodeId) -> Self::Iterator<'_> {
        (**self).edges(node)
    }
}

pub trait DirectedEdges: Directed + Edges {
    type DirectedIterator<'e>: Iterator<Item = Self::EdgeId>
    where
        Self: 'e;

    fn edges_in(&self, direction: Direction, node: Self::NodeId) -> Self::DirectedIterator<'_>;

    fn outgoing_edges(&self, node: Self::NodeId) -> Self::DirectedIterator<'_> {
        self.edges_in(Outgoing, node)
    }

    fn incoming_edges(&self, node: Self::NodeId) -> Self::DirectedIterator<'_> {
        self.edges_in(Incoming, node)
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
        self.outgoing_edges(node).count()
    }

    fn incoming_degree(&self, node: Self::NodeId) -> usize {
        self.incoming_edges(node).count()
    }

    fn has_outgoing(&self, node: Self::NodeId) -> bool {
        self.outgoing_edges(node).next().is_some()
    }

    fn has_incoming(&self, node: Self::NodeId) -> bool {
        self.incoming_edges(node).next().is_some()
    }
}
