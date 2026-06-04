use crate::{
    attached::{DirectedEdges, Edges},
    base::{Base, Directed},
    connections::DirectedConnectionMethods,
    data::{Data, DataMut, DataRef},
    degree::{Class, Degrees},
    direction::Direction,
    identifiers::{EdgeIdentifiers, NodeIdentifiers},
    neighbors::{DirectedNeighbors, Neighbors},
    references::{DirectedEdgeRef, EdgeRef, EdgeReferences, NodeReferences},
};

pub struct Reversed<G: Directed> {
    graph: G,
}

pub struct ReversedEdgeRef<D: DirectedEdgeRef> {
    edge: D,
}

impl<D: DirectedEdgeRef> ReversedEdgeRef<D> {
    pub const fn new(edge: D) -> Self {
        Self { edge }
    }

    pub const fn normal(&self) -> &D {
        &self.edge
    }

    pub const fn normal_mut(&mut self) -> &mut D {
        &mut self.edge
    }

    pub fn get(self) -> D {
        self.edge
    }
}

impl<D: DirectedEdgeRef> EdgeRef for ReversedEdgeRef<D> {
    type EdgeId = D::EdgeId;
    type Connection = D::Connection;
    type Value = D::Value;

    fn id(&self) -> Self::EdgeId {
        self.normal().id()
    }

    fn connection(&self) -> Self::Connection {
        self.normal().connection().reversed()
    }

    fn value(&self) -> &Self::Value {
        self.normal().value()
    }
}

pub struct ReversedEdgeIterator<I: Iterator>
where
    I::Item: DirectedEdgeRef,
{
    iterator: I,
}

impl<I: Iterator> ReversedEdgeIterator<I>
where
    I::Item: DirectedEdgeRef,
{
    pub const fn new(iterator: I) -> Self {
        Self { iterator }
    }
}

impl<I: Iterator> Iterator for ReversedEdgeIterator<I>
where
    I::Item: DirectedEdgeRef,
{
    type Item = ReversedEdgeRef<I::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator.next().map(ReversedEdgeRef::new)
    }
}

impl<G: Directed> Base for Reversed<G> {
    type NodeId = G::NodeId;
    type EdgeId = G::EdgeId;

    type Connection = G::Connection;

    type Kind = G::Kind;
    type Type = G::Type;
    type Loop = G::Loop;
}

impl<G: Directed> Reversed<G> {
    pub const fn new(graph: G) -> Self {
        Self { graph }
    }

    pub const fn normal(&self) -> &G {
        &self.graph
    }

    pub const fn normal_mut(&mut self) -> &mut G {
        &mut self.graph
    }

    pub fn get(self) -> G {
        self.graph
    }
}

impl<G: Directed + NodeIdentifiers> NodeIdentifiers for Reversed<G> {
    type NodeIdIterator<'g>
        = G::NodeIdIterator<'g>
    where
        Self: 'g;

    fn node_identifiers(&self) -> Self::NodeIdIterator<'_> {
        self.normal().node_identifiers()
    }
}

impl<G: Directed + EdgeIdentifiers> EdgeIdentifiers for Reversed<G> {
    type EdgeIdIterator<'g>
        = G::EdgeIdIterator<'g>
    where
        Self: 'g;

    fn edge_identifiers(&self) -> Self::EdgeIdIterator<'_> {
        self.normal().edge_identifiers()
    }
}

impl<G: Directed + Data> Data for Reversed<G> {
    type NodeValue = G::NodeValue;
    type EdgeValue = G::EdgeValue;
}

impl<G: Directed + DataRef> DataRef for Reversed<G> {
    fn node_value(&self, id: Self::NodeId) -> Option<&Self::NodeValue> {
        self.normal().node_value(id)
    }

    fn edge_value(&self, id: Self::EdgeId) -> Option<&Self::EdgeValue> {
        self.normal().edge_value(id)
    }
}

impl<G: Directed + DataMut> DataMut for Reversed<G> {
    fn node_value_mut(&mut self, id: Self::NodeId) -> Option<&mut Self::NodeValue> {
        self.normal_mut().node_value_mut(id)
    }

    fn edge_value_mut(&mut self, id: Self::EdgeId) -> Option<&mut Self::EdgeValue> {
        self.normal_mut().edge_value_mut(id)
    }
}

impl<G: Directed + NodeReferences> NodeReferences for Reversed<G> {
    type NodeRef<'g>
        = G::NodeRef<'g>
    where
        Self: 'g;

    type NodeRefIterator<'g>
        = G::NodeRefIterator<'g>
    where
        Self: 'g;

    fn node_references(&self) -> Self::NodeRefIterator<'_> {
        self.normal().node_references()
    }
}

impl<G: Directed + EdgeReferences> EdgeReferences for Reversed<G> {
    type EdgeRef<'g>
        = ReversedEdgeRef<G::EdgeRef<'g>>
    where
        Self: 'g;

    type EdgeRefIterator<'g>
        = ReversedEdgeIterator<G::EdgeRefIterator<'g>>
    where
        Self: 'g;

    fn edge_references(&self) -> Self::EdgeRefIterator<'_> {
        Self::EdgeRefIterator::new(self.normal().edge_references())
    }
}

impl<G: Directed + Edges> Edges for Reversed<G> {
    type EdgeIterator<'g>
        = ReversedEdgeIterator<G::EdgeIterator<'g>>
    where
        Self: 'g;

    fn edges(&self, node: Self::NodeId) -> Self::EdgeIterator<'_> {
        Self::EdgeIterator::new(self.normal().edges(node))
    }

    fn has_edges(&self, node: Self::NodeId) -> bool {
        self.normal().has_edges(node)
    }
}

impl<G: DirectedEdges> DirectedEdges for Reversed<G> {
    type DirectedEdgeIterator<'g>
        = ReversedEdgeIterator<G::DirectedEdgeIterator<'g>>
    where
        Self: 'g;

    fn directed_edges(
        &self,
        direction: Direction,
        node: Self::NodeId,
    ) -> Self::DirectedEdgeIterator<'_> {
        Self::DirectedEdgeIterator::new(self.normal().directed_edges(direction.reversed(), node))
    }

    fn outgoing_edges(&self, node: Self::NodeId) -> Self::DirectedEdgeIterator<'_> {
        Self::DirectedEdgeIterator::new(self.normal().incoming_edges(node))
    }

    fn incoming_edges(&self, node: Self::NodeId) -> Self::DirectedEdgeIterator<'_> {
        Self::DirectedEdgeIterator::new(self.normal().outgoing_edges(node))
    }

    fn class(&self, node: Self::NodeId) -> Class {
        Class::compute(self.has_outgoing(node), self.has_incoming(node))
    }

    fn degree(&self, node: Self::NodeId) -> Degrees {
        Degrees::new(self.outgoing_degree(node), self.incoming_degree(node))
    }

    fn directed_degree(&self, direction: Direction, node: Self::NodeId) -> usize {
        self.normal().directed_degree(direction.reversed(), node)
    }

    fn outgoing_degree(&self, node: Self::NodeId) -> usize {
        self.normal().incoming_degree(node)
    }

    fn incoming_degree(&self, node: Self::NodeId) -> usize {
        self.normal().outgoing_degree(node)
    }

    fn has_directed(&self, direction: Direction, node: Self::NodeId) -> bool {
        self.normal().has_directed(direction.reversed(), node)
    }

    fn has_outgoing(&self, node: Self::NodeId) -> bool {
        self.has_incoming(node)
    }

    fn has_incoming(&self, node: Self::NodeId) -> bool {
        self.has_outgoing(node)
    }
}

impl<G: Directed + Neighbors> Neighbors for Reversed<G> {
    type NodeIterator<'g>
        = G::NodeIterator<'g>
    where
        Self: 'g;

    fn neighbors(&self, node: Self::NodeId) -> Self::NodeIterator<'_> {
        self.normal().neighbors(node)
    }

    fn has_neighbors(&self, node: Self::NodeId) -> bool {
        self.normal().has_neighbors(node)
    }
}

impl<G: DirectedNeighbors> DirectedNeighbors for Reversed<G> {
    type DirectedNodeIterator<'g>
        = G::DirectedNodeIterator<'g>
    where
        Self: 'g;

    fn directed_neighbors(
        &self,
        direction: Direction,
        node: Self::NodeId,
    ) -> Self::DirectedNodeIterator<'_> {
        self.normal().directed_neighbors(direction.reversed(), node)
    }

    fn outgoing_neighbors(&self, node: Self::NodeId) -> Self::DirectedNodeIterator<'_> {
        self.normal().incoming_neighbors(node)
    }

    fn incoming_neighbors(&self, node: Self::NodeId) -> Self::DirectedNodeIterator<'_> {
        self.normal().outgoing_neighbors(node)
    }

    fn has_outgoing_neighbors(&self, node: Self::NodeId) -> bool {
        self.normal().has_incoming_neighbors(node)
    }

    fn has_incoming_neighbors(&self, node: Self::NodeId) -> bool {
        self.normal().has_outgoing_neighbors(node)
    }
}
