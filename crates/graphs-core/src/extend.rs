use crate::build::{AddEdgeResult, AddNodeResult, Build, Edge, EdgeOf};

pub struct ExtendNodes<'g, G: Build + ?Sized, N: Iterator<Item = G::NodeValue>> {
    graph: &'g mut G,
    nodes: N,
}

pub struct ExtendEdges<'g, G: Build + ?Sized, E: Iterator<Item = EdgeOf<G>>> {
    graph: &'g mut G,
    edges: E,
}

pub struct ExtendDefault<'g, G: Build + ?Sized, C: Iterator<Item = G::Connection>>
where
    G::EdgeValue: Default,
{
    graph: &'g mut G,
    connections: C,
}

impl<'g, G: Build + ?Sized, N: Iterator<Item = G::NodeValue>> ExtendNodes<'g, G, N> {
    pub const fn new(graph: &'g mut G, nodes: N) -> Self {
        Self { graph, nodes }
    }
}

impl<'g, G: Build + ?Sized, E: Iterator<Item = EdgeOf<G>>> ExtendEdges<'g, G, E> {
    pub const fn new(graph: &'g mut G, edges: E) -> Self {
        Self { graph, edges }
    }
}

impl<'g, G: Build + ?Sized, C: Iterator<Item = G::Connection>> ExtendDefault<'g, G, C>
where
    G::EdgeValue: Default,
{
    pub const fn new(graph: &'g mut G, connections: C) -> Self {
        Self { graph, connections }
    }
}

impl<'g, G: Build + ?Sized, N: Iterator<Item = G::NodeValue>> Iterator for ExtendNodes<'g, G, N> {
    type Item = AddNodeResult<G>;

    fn next(&mut self) -> Option<Self::Item> {
        self.nodes.next().map(|node| self.graph.add_node(node))
    }
}

impl<'g, G: Build + ?Sized, E: Iterator<Item = EdgeOf<G>>> Iterator for ExtendEdges<'g, G, E> {
    type Item = AddEdgeResult<G>;

    fn next(&mut self) -> Option<Self::Item> {
        self.edges.next().map(|edge| self.graph.add_edge(edge))
    }
}

impl<'g, G: Build + ?Sized, C: Iterator<Item = G::Connection>> Iterator for ExtendDefault<'g, G, C>
where
    G::EdgeValue: Default,
{
    type Item = AddEdgeResult<G>;

    fn next(&mut self) -> Option<Self::Item> {
        self.connections
            .next()
            .map(Edge::new_default)
            .map(|edge| self.graph.add_edge(edge))
    }
}

pub trait Extend: Build {
    fn extend_nodes<N: IntoIterator<Item = Self::NodeValue>>(
        &mut self,
        nodes: N,
    ) -> ExtendNodes<'_, Self, N::IntoIter> {
        ExtendNodes::new(self, nodes.into_iter())
    }

    fn extend_edges<E: IntoIterator<Item = EdgeOf<Self>>>(
        &mut self,
        edges: E,
    ) -> ExtendEdges<'_, Self, E::IntoIter> {
        ExtendEdges::new(self, edges.into_iter())
    }

    fn extend_default<C: IntoIterator<Item = Self::Connection>>(
        &mut self,
        connections: C,
    ) -> ExtendDefault<'_, Self, C::IntoIter>
    where
        Self::EdgeValue: Default,
    {
        ExtendDefault::new(self, connections.into_iter())
    }
}

impl<G: Build + ?Sized> Extend for G {}
