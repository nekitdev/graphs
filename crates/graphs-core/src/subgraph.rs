use bon::Builder;

use crate::{
    attached::{DirectedEdges, Edges},
    base::Base,
    build::{RecoverableEdge, RecoverableNode, TryAddEdges, TryAddNodes},
    data::{Data, DataMut, DataRef},
    direction::Direction,
    edges::EdgeIn,
    identifiers::{EdgeIdentifiers, NodeIdentifiers},
    neighbors::{DirectedNeighbors, Neighbors},
    nodes::NodeIn,
    references::{EdgeRef, EdgeReferences, NodeRef, NodeReferences},
};

#[derive(Builder)]
pub struct Subgraph<G: Base, N: NodeFilter<G>, E: EdgeFilter<G>> {
    graph: G,
    node_filter: N,
    edge_filter: E,
}

impl<G: Base, N: NodeFilter<G>, E: EdgeFilter<G>> Subgraph<G, N, E> {
    pub const fn full(&self) -> &G {
        &self.graph
    }

    pub const fn full_mut(&mut self) -> &mut G {
        &mut self.graph
    }

    pub fn get(self) -> G {
        self.graph
    }

    pub const fn node_filter(&self) -> &N {
        &self.node_filter
    }

    pub const fn edge_filter(&self) -> &E {
        &self.edge_filter
    }

    pub fn include_node(&self, node: G::NodeId) -> bool {
        self.node_filter.include_node(node, self.full())
    }

    pub fn include_edge(&self, edge: G::EdgeId) -> bool {
        self.edge_filter.include_edge(edge, self.full())
    }
}

impl<G: Base, N: NodeFilter<G>, E: EdgeFilter<G>> Base for Subgraph<G, N, E> {
    type NodeId = G::NodeId;
    type EdgeId = G::EdgeId;

    type Connection = G::Connection;

    type Kind = G::Kind;
    type Type = G::Type;
    type Loop = G::Loop;
}

impl<G: Data, N: NodeFilter<G>, E: EdgeFilter<G>> Data for Subgraph<G, N, E> {
    type NodeValue = G::NodeValue;
    type EdgeValue = G::EdgeValue;
}

impl<G: DataRef, N: NodeFilter<G>, E: EdgeFilter<G>> DataRef for Subgraph<G, N, E> {
    fn node_value(&self, id: Self::NodeId) -> Option<&Self::NodeValue> {
        if self.include_node(id) {
            self.full().node_value(id)
        } else {
            None
        }
    }

    fn edge_value(&self, id: Self::EdgeId) -> Option<&Self::EdgeValue> {
        if self.include_edge(id) {
            self.full().edge_value(id)
        } else {
            None
        }
    }
}

impl<G: DataMut, N: NodeFilter<G>, E: EdgeFilter<G>> DataMut for Subgraph<G, N, E> {
    fn node_value_mut(&mut self, id: Self::NodeId) -> Option<&mut Self::NodeValue> {
        if self.include_node(id) {
            self.full_mut().node_value_mut(id)
        } else {
            None
        }
    }

    fn edge_value_mut(&mut self, id: Self::EdgeId) -> Option<&mut Self::EdgeValue> {
        if self.include_edge(id) {
            self.full_mut().edge_value_mut(id)
        } else {
            None
        }
    }
}

impl<G: NodeIdentifiers, N: NodeFilter<G>, E: EdgeFilter<G>> NodeIdentifiers for Subgraph<G, N, E> {
    type NodeIdIterator<'g>
        = FilteredNodeIdentifiers<'g, G, N>
    where
        Self: 'g;

    fn node_identifiers(&self) -> Self::NodeIdIterator<'_> {
        let graph = self.full();

        let iterator = graph.node_identifiers();

        let filter = self.node_filter();

        Self::NodeIdIterator {
            graph,
            iterator,
            filter,
        }
    }
}

impl<G: EdgeIdentifiers, N: NodeFilter<G>, E: EdgeFilter<G>> EdgeIdentifiers for Subgraph<G, N, E> {
    type EdgeIdIterator<'g>
        = FilteredEdgeIdentifiers<'g, G, E>
    where
        Self: 'g;

    fn edge_identifiers(&self) -> Self::EdgeIdIterator<'_> {
        let graph = self.full();

        let iterator = graph.edge_identifiers();

        let filter = self.edge_filter();

        Self::EdgeIdIterator {
            graph,
            iterator,
            filter,
        }
    }
}

pub struct FilteredNodeIdentifiers<'a, G: NodeIdentifiers + 'a, N: NodeFilter<G>> {
    graph: &'a G,
    iterator: G::NodeIdIterator<'a>,
    filter: &'a N,
}

impl<'a, G: NodeIdentifiers + 'a, N: NodeFilter<G>> Iterator for FilteredNodeIdentifiers<'a, G, N> {
    type Item = G::NodeId;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator
            .find(|&node| self.filter.include_node(node, self.graph))
    }
}

pub struct FilteredEdgeIdentifiers<'a, G: EdgeIdentifiers + 'a, E: EdgeFilter<G>> {
    graph: &'a G,
    iterator: G::EdgeIdIterator<'a>,
    filter: &'a E,
}

impl<'a, G: EdgeIdentifiers + 'a, E: EdgeFilter<G>> Iterator for FilteredEdgeIdentifiers<'a, G, E> {
    type Item = G::EdgeId;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator
            .find(|&edge| self.filter.include_edge(edge, self.graph))
    }
}

impl<G: Neighbors, N: NodeFilter<G>, E: EdgeFilter<G>> Neighbors for Subgraph<G, N, E> {
    type NodeIterator<'g>
        = FilteredNeighbors<'g, G, N>
    where
        Self: 'g;

    fn neighbors(&self, node: Self::NodeId) -> Self::NodeIterator<'_> {
        let non_empty = self.include_node(node);

        let graph = self.full();

        let iterator = graph.neighbors(node);

        let filter = self.node_filter();

        Self::NodeIterator {
            non_empty,
            graph,
            iterator,
            filter,
        }
    }
}

impl<G: DirectedNeighbors, N: NodeFilter<G>, E: EdgeFilter<G>> DirectedNeighbors
    for Subgraph<G, N, E>
{
    type DirectedNodeIterator<'g>
        = FilteredDirectedNeighbors<'g, G, N>
    where
        Self: 'g;

    fn directed_neighbors(
        &self,
        direction: Direction,
        node: Self::NodeId,
    ) -> Self::DirectedNodeIterator<'_> {
        let non_empty = self.include_node(node);

        let graph = self.full();

        let iterator = graph.directed_neighbors(direction, node);

        let filter = self.node_filter();

        Self::DirectedNodeIterator {
            non_empty,
            graph,
            iterator,
            filter,
        }
    }

    fn incoming_neighbors(&self, node: Self::NodeId) -> Self::DirectedNodeIterator<'_> {
        let non_empty = self.include_node(node);

        let graph = self.full();

        let iterator = graph.incoming_neighbors(node);

        let filter = self.node_filter();

        Self::DirectedNodeIterator {
            non_empty,
            graph,
            iterator,
            filter,
        }
    }
}

impl<G: Edges, N: NodeFilter<G>, E: EdgeFilter<G>> Edges for Subgraph<G, N, E> {
    type EdgeIterator<'g>
        = FilteredEdges<'g, G, E>
    where
        Self: 'g;

    fn edges(&self, node: Self::NodeId) -> Self::EdgeIterator<'_> {
        let non_empty = self.include_node(node);

        let graph = self.full();

        let iterator = graph.edges(node);

        let filter = self.edge_filter();

        Self::EdgeIterator {
            non_empty,
            graph,
            iterator,
            filter,
        }
    }
}

impl<G: DirectedEdges, N: NodeFilter<G>, E: EdgeFilter<G>> DirectedEdges for Subgraph<G, N, E> {
    type DirectedEdgeIterator<'g>
        = FilteredDirectedEdges<'g, G, E>
    where
        Self: 'g;

    fn directed_edges(
        &self,
        direction: Direction,
        node: Self::NodeId,
    ) -> Self::DirectedEdgeIterator<'_> {
        let non_empty = self.include_node(node);

        let graph = self.full();

        let iterator = graph.directed_edges(direction, node);

        let filter = self.edge_filter();

        Self::DirectedEdgeIterator {
            non_empty,
            graph,
            iterator,
            filter,
        }
    }
}

impl<G: NodeReferences, N: NodeFilter<G>, E: EdgeFilter<G>> NodeReferences for Subgraph<G, N, E> {
    type NodeRef<'g>
        = G::NodeRef<'g>
    where
        Self: 'g;

    type NodeRefIterator<'g>
        = FilteredNodeReferences<'g, G, N>
    where
        Self: 'g;

    fn node_references(&self) -> Self::NodeRefIterator<'_> {
        let graph = self.full();

        let iterator = graph.node_references();

        let filter = self.node_filter();

        Self::NodeRefIterator {
            graph,
            iterator,
            filter,
        }
    }
}

impl<G: EdgeReferences, N: NodeFilter<G>, E: EdgeFilter<G>> EdgeReferences for Subgraph<G, N, E> {
    type EdgeRef<'g>
        = G::EdgeRef<'g>
    where
        Self: 'g;

    type EdgeRefIterator<'g>
        = FilteredEdgeReferences<'g, G, E>
    where
        Self: 'g;

    fn edge_references(&self) -> Self::EdgeRefIterator<'_> {
        let graph = self.full();

        let iterator = graph.edge_references();

        let filter = self.edge_filter();

        Self::EdgeRefIterator {
            graph,
            iterator,
            filter,
        }
    }
}

pub struct FilteredNeighbors<'a, G: Neighbors + 'a, N: NodeFilter<G>> {
    non_empty: bool,
    graph: &'a G,
    iterator: G::NodeIterator<'a>,
    filter: &'a N,
}

impl<'a, G: Neighbors + 'a, N: NodeFilter<G>> Iterator for FilteredNeighbors<'a, G, N> {
    type Item = G::NodeId;

    fn next(&mut self) -> Option<Self::Item> {
        if self.non_empty {
            self.iterator
                .find(|&node| self.filter.include_node(node, self.graph))
        } else {
            None
        }
    }
}

pub struct FilteredDirectedNeighbors<'a, G: DirectedNeighbors + 'a, N: NodeFilter<G>> {
    non_empty: bool,
    graph: &'a G,
    iterator: G::DirectedNodeIterator<'a>,
    filter: &'a N,
}

impl<'a, G: DirectedNeighbors + 'a, N: NodeFilter<G>> Iterator
    for FilteredDirectedNeighbors<'a, G, N>
{
    type Item = G::NodeId;

    fn next(&mut self) -> Option<Self::Item> {
        if self.non_empty {
            self.iterator
                .find(|&node| self.filter.include_node(node, self.graph))
        } else {
            None
        }
    }
}

pub struct FilteredEdges<'a, G: Edges + 'a, E: EdgeFilter<G>> {
    non_empty: bool,
    graph: &'a G,
    iterator: G::EdgeIterator<'a>,
    filter: &'a E,
}

impl<'a, G: Edges + 'a, E: EdgeFilter<G>> Iterator for FilteredEdges<'a, G, E> {
    type Item = G::EdgeRef<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.non_empty {
            self.iterator
                .find(|edge| self.filter.include_edge(edge.id(), self.graph))
        } else {
            None
        }
    }
}

pub struct FilteredDirectedEdges<'a, G: DirectedEdges + 'a, E: EdgeFilter<G>> {
    non_empty: bool,
    graph: &'a G,
    iterator: G::DirectedEdgeIterator<'a>,
    filter: &'a E,
}

impl<'a, G: DirectedEdges + 'a, E: EdgeFilter<G>> Iterator for FilteredDirectedEdges<'a, G, E> {
    type Item = G::EdgeRef<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.non_empty {
            self.iterator
                .find(|edge| self.filter.include_edge(edge.id(), self.graph))
        } else {
            None
        }
    }
}

pub struct FilteredNodeReferences<'a, G: NodeReferences + 'a, N: NodeFilter<G>> {
    graph: &'a G,
    iterator: G::NodeRefIterator<'a>,
    filter: &'a N,
}

impl<'a, G: NodeReferences + 'a, N: NodeFilter<G>> Iterator for FilteredNodeReferences<'a, G, N> {
    type Item = G::NodeRef<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator
            .find(|node| self.filter.include_node(node.id(), self.graph))
    }
}

pub struct FilteredEdgeReferences<'a, G: EdgeReferences + 'a, E: EdgeFilter<G>> {
    graph: &'a G,
    iterator: G::EdgeRefIterator<'a>,
    filter: &'a E,
}

impl<'a, G: EdgeReferences + 'a, E: EdgeFilter<G>> Iterator for FilteredEdgeReferences<'a, G, E> {
    type Item = G::EdgeRef<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator
            .find(|edge| self.filter.include_edge(edge.id(), self.graph))
    }
}

impl<G: TryAddNodes, N: NodeFilter<G>, E: EdgeFilter<G>> TryAddNodes for Subgraph<G, N, E> {
    type Error = G::Error;

    fn try_add_node(&mut self, node: NodeIn<Self>) -> RecoverableNode<Self> {
        self.full_mut().try_add_node(node)
    }
}

impl<G: TryAddEdges, N: NodeFilter<G>, E: EdgeFilter<G>> TryAddEdges for Subgraph<G, N, E> {
    type Error = G::Error;

    fn try_add_edge(&mut self, edge: EdgeIn<Self>) -> RecoverableEdge<Self> {
        self.full_mut().try_add_edge(edge)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Include;

impl<G: Base> NodeFilter<G> for Include {
    fn include_node(&self, _node: G::NodeId, _graph: &G) -> bool {
        true
    }
}

impl<G: Base> EdgeFilter<G> for Include {
    fn include_edge(&self, _edge: G::EdgeId, _graph: &G) -> bool {
        true
    }
}

pub trait NodeFilter<G: Base> {
    fn include_node(&self, node: G::NodeId, graph: &G) -> bool;
}

pub trait EdgeFilter<G: Base> {
    fn include_edge(&self, edge: G::EdgeId, graph: &G) -> bool;
}

impl<G: Base, F: Fn(G::NodeId, &G) -> bool> NodeFilter<G> for F {
    fn include_node(&self, node: G::NodeId, graph: &G) -> bool {
        self(node, graph)
    }
}

impl<G: Base, F: Fn(G::EdgeId, &G) -> bool> EdgeFilter<G> for F {
    fn include_edge(&self, edge: G::EdgeId, graph: &G) -> bool {
        self(edge, graph)
    }
}

pub trait Filter: Base {
    fn filter<N: NodeFilter<Self>, E: EdgeFilter<Self>>(
        self,
        node_filter: N,
        edge_filter: E,
    ) -> Subgraph<Self, N, E>
    where
        Self: Sized,
    {
        Subgraph {
            graph: self,
            node_filter,
            edge_filter,
        }
    }

    fn filter_nodes<N: NodeFilter<Self>>(self, node_filter: N) -> Subgraph<Self, N, Include>
    where
        Self: Sized,
    {
        self.filter(node_filter, Include)
    }

    fn filter_edges<E: EdgeFilter<Self>>(self, edge_filter: E) -> Subgraph<Self, Include, E>
    where
        Self: Sized,
    {
        self.filter(Include, edge_filter)
    }
}

impl<G: Base + ?Sized> Filter for G {}
