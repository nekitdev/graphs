//! Reversing directions in graphs.

use crate::{
    base::Base,
    count::{EdgeCount, NodeCount},
    data::{Data, DataMut, DataRef},
    direction::{Direction, Incoming},
    identifiers::{EdgeIdentifiers, NodeIdentifiers},
    indexed::{EdgeCompact, EdgeIndexed, NodeCompact, NodeIndexed},
    kind::Kinded,
    neighbors::{DirectedNeighbors, Neighbors},
    visit::Visit,
};

/// Represents adapters that reverse directions in graphs.
pub struct Reversed<G> {
    graph: G,
}

impl<G> Reversed<G> {
    /// Constructs [`Self`] from the given graph.
    pub const fn new(graph: G) -> Self {
        Self { graph }
    }

    /// Returns the underlying graph behind immutable reference.
    pub const fn get(&self) -> &G {
        &self.graph
    }

    /// Returns the underlying graph behind mutable reference.
    pub const fn get_mut(&mut self) -> &mut G {
        &mut self.graph
    }

    /// Consumes [`Self`] and returns the contained graph.
    pub fn take(self) -> G {
        self.graph
    }
}

impl<G: Base> Base for Reversed<G> {
    type NodeId = G::NodeId;
    type EdgeId = G::EdgeId;
}

impl<G: Data> Data for Reversed<G> {
    type NodeValue = G::NodeValue;
    type EdgeValue = G::EdgeValue;
}

impl<G: DataRef> DataRef for Reversed<G> {
    fn node_value(&self, id: Self::NodeId) -> Option<&Self::NodeValue> {
        self.get().node_value(id)
    }

    fn edge_value(&self, id: Self::EdgeId) -> Option<&Self::EdgeValue> {
        self.get().edge_value(id)
    }
}

impl<G: DataMut> DataMut for Reversed<G> {
    fn node_value_mut(&mut self, id: Self::NodeId) -> Option<&mut Self::NodeValue> {
        self.get_mut().node_value_mut(id)
    }

    fn edge_value_mut(&mut self, id: Self::EdgeId) -> Option<&mut Self::EdgeValue> {
        self.get_mut().edge_value_mut(id)
    }
}

impl<G: DirectedNeighbors> Neighbors for Reversed<G> {
    type Iterator<'n>
        = G::DirectedIterator<'n>
    where
        Self: 'n;

    fn neighbors(&self, node: Self::NodeId) -> Self::Iterator<'_> {
        self.graph.directed_neighbors(node, Incoming)
    }
}

impl<G: DirectedNeighbors> DirectedNeighbors for Reversed<G> {
    type DirectedIterator<'n>
        = G::DirectedIterator<'n>
    where
        Self: 'n;

    fn directed_neighbors(
        &self,
        node: Self::NodeId,
        direction: Direction,
    ) -> Self::DirectedIterator<'_> {
        self.graph.directed_neighbors(node, direction.reversed())
    }
}

impl<G: Kinded> Kinded for Reversed<G> {
    type Kind = G::Kind;
}

impl<G: NodeCount> NodeCount for Reversed<G> {
    fn node_count(&self) -> usize {
        self.graph.node_count()
    }
}

impl<G: EdgeCount> EdgeCount for Reversed<G> {
    fn edge_count(&self) -> usize {
        self.graph.edge_count()
    }
}

impl<G: NodeIndexed> NodeIndexed for Reversed<G> {
    fn node_bound(&self) -> usize {
        self.graph.node_bound()
    }

    fn node_index(&self, id: Self::NodeId) -> usize {
        self.graph.node_index(id)
    }

    fn node_id(&self, index: usize) -> Self::NodeId {
        self.graph.node_id(index)
    }
}

impl<G: EdgeIndexed> EdgeIndexed for Reversed<G> {
    fn edge_bound(&self) -> usize {
        self.graph.edge_bound()
    }

    fn edge_index(&self, id: Self::EdgeId) -> usize {
        self.graph.edge_index(id)
    }

    fn edge_id(&self, index: usize) -> Self::EdgeId {
        self.graph.edge_id(index)
    }
}

impl<G: NodeCompact> NodeCompact for Reversed<G> {}
impl<G: EdgeCompact> EdgeCompact for Reversed<G> {}

impl<G: NodeIdentifiers> NodeIdentifiers for Reversed<G> {
    type Identifiers<'i>
        = G::Identifiers<'i>
    where
        Self: 'i;

    fn node_identifiers(&self) -> Self::Identifiers<'_> {
        self.graph.node_identifiers()
    }
}

impl<G: EdgeIdentifiers> EdgeIdentifiers for Reversed<G> {
    type Identifiers<'i>
        = G::Identifiers<'i>
    where
        Self: 'i;

    fn edge_identifiers(&self) -> Self::Identifiers<'_> {
        self.graph.edge_identifiers()
    }
}

impl<G: Visit> Visit for Reversed<G> {
    type Visitor = G::Visitor;

    fn build_visitor(&self) -> Self::Visitor {
        self.graph.build_visitor()
    }

    fn reset_visitor(&self, visitor: &mut Self::Visitor) {
        self.graph.reset_visitor(visitor);
    }
}
