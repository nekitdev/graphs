use crate::{
    base::Directed,
    degree::{Class, Degrees},
    direction::Direction::{self, Incoming, Outgoing},
    references::EdgeReferences,
};

pub trait Edges: EdgeReferences {
    type EdgeIterator<'g>: Iterator<Item = Self::EdgeRef<'g>>
    where
        Self: 'g;

    fn edges(&self, node: Self::NodeId) -> Self::EdgeIterator<'_>;

    fn has_edges(&self, node: Self::NodeId) -> bool {
        self.edges(node).next().is_some()
    }
}

impl<G: Edges + ?Sized> Edges for &G {
    type EdgeIterator<'g>
        = G::EdgeIterator<'g>
    where
        Self: 'g;

    fn edges(&self, node: Self::NodeId) -> Self::EdgeIterator<'_> {
        (*self).edges(node)
    }

    fn has_edges(&self, node: Self::NodeId) -> bool {
        (*self).has_edges(node)
    }
}

impl<G: Edges + ?Sized> Edges for &mut G {
    type EdgeIterator<'g>
        = G::EdgeIterator<'g>
    where
        Self: 'g;

    fn edges(&self, node: Self::NodeId) -> Self::EdgeIterator<'_> {
        (**self).edges(node)
    }

    fn has_edges(&self, node: Self::NodeId) -> bool {
        (**self).has_edges(node)
    }
}

pub trait DirectedEdges: Directed + Edges {
    type DirectedEdgeIterator<'g>: Iterator<Item = Self::EdgeRef<'g>>
    where
        Self: 'g;

    fn directed_edges(
        &self,
        direction: Direction,
        node: Self::NodeId,
    ) -> Self::DirectedEdgeIterator<'_>;

    fn outgoing_edges(&self, node: Self::NodeId) -> Self::DirectedEdgeIterator<'_> {
        self.directed_edges(Outgoing, node)
    }

    fn incoming_edges(&self, node: Self::NodeId) -> Self::DirectedEdgeIterator<'_> {
        self.directed_edges(Incoming, node)
    }

    /// Returns the [`Degrees`] of the given `node`.
    fn degree(&self, node: Self::NodeId) -> Degrees {
        Degrees::new(self.outgoing_degree(node), self.incoming_degree(node))
    }

    /// Returns the [`Class`] of the given `node`.
    fn class(&self, node: Self::NodeId) -> Class {
        Class::compute(self.has_outgoing(node), self.has_incoming(node))
    }

    fn directed_degree(&self, direction: Direction, node: Self::NodeId) -> usize {
        self.directed_edges(direction, node).count()
    }

    fn outgoing_degree(&self, node: Self::NodeId) -> usize {
        self.directed_degree(Outgoing, node)
    }

    fn incoming_degree(&self, node: Self::NodeId) -> usize {
        self.directed_degree(Incoming, node)
    }

    fn has_directed(&self, direction: Direction, node: Self::NodeId) -> bool {
        self.directed_edges(direction, node).next().is_some()
    }

    fn has_outgoing(&self, node: Self::NodeId) -> bool {
        self.has_directed(Outgoing, node)
    }

    fn has_incoming(&self, node: Self::NodeId) -> bool {
        self.has_directed(Incoming, node)
    }
}

impl<G: DirectedEdges + ?Sized> DirectedEdges for &G {
    type DirectedEdgeIterator<'g>
        = G::DirectedEdgeIterator<'g>
    where
        Self: 'g;

    fn directed_edges(
        &self,
        direction: Direction,
        node: Self::NodeId,
    ) -> Self::DirectedEdgeIterator<'_> {
        (*self).directed_edges(direction, node)
    }

    fn outgoing_edges(&self, node: Self::NodeId) -> Self::DirectedEdgeIterator<'_> {
        (*self).outgoing_edges(node)
    }

    fn incoming_edges(&self, node: Self::NodeId) -> Self::DirectedEdgeIterator<'_> {
        (*self).incoming_edges(node)
    }

    fn degree(&self, node: Self::NodeId) -> Degrees {
        (*self).degree(node)
    }

    fn class(&self, node: Self::NodeId) -> Class {
        (*self).class(node)
    }

    fn directed_degree(&self, direction: Direction, node: Self::NodeId) -> usize {
        (*self).directed_degree(direction, node)
    }

    fn outgoing_degree(&self, node: Self::NodeId) -> usize {
        (*self).outgoing_degree(node)
    }

    fn incoming_degree(&self, node: Self::NodeId) -> usize {
        (*self).incoming_degree(node)
    }

    fn has_directed(&self, direction: Direction, node: Self::NodeId) -> bool {
        (*self).has_directed(direction, node)
    }

    fn has_outgoing(&self, node: Self::NodeId) -> bool {
        (*self).has_outgoing(node)
    }

    fn has_incoming(&self, node: Self::NodeId) -> bool {
        (*self).has_incoming(node)
    }
}

impl<G: DirectedEdges + ?Sized> DirectedEdges for &mut G {
    type DirectedEdgeIterator<'g>
        = G::DirectedEdgeIterator<'g>
    where
        Self: 'g;

    fn directed_edges(
        &self,
        direction: Direction,
        node: Self::NodeId,
    ) -> Self::DirectedEdgeIterator<'_> {
        (**self).directed_edges(direction, node)
    }

    fn outgoing_edges(&self, node: Self::NodeId) -> Self::DirectedEdgeIterator<'_> {
        (**self).outgoing_edges(node)
    }

    fn incoming_edges(&self, node: Self::NodeId) -> Self::DirectedEdgeIterator<'_> {
        (**self).incoming_edges(node)
    }

    fn degree(&self, node: Self::NodeId) -> Degrees {
        (**self).degree(node)
    }

    fn class(&self, node: Self::NodeId) -> Class {
        (**self).class(node)
    }

    fn directed_degree(&self, direction: Direction, node: Self::NodeId) -> usize {
        (**self).directed_degree(direction, node)
    }

    fn outgoing_degree(&self, node: Self::NodeId) -> usize {
        (**self).outgoing_degree(node)
    }

    fn incoming_degree(&self, node: Self::NodeId) -> usize {
        (**self).incoming_degree(node)
    }

    fn has_directed(&self, direction: Direction, node: Self::NodeId) -> bool {
        (**self).has_directed(direction, node)
    }

    fn has_outgoing(&self, node: Self::NodeId) -> bool {
        (**self).has_outgoing(node)
    }

    fn has_incoming(&self, node: Self::NodeId) -> bool {
        (**self).has_incoming(node)
    }
}
