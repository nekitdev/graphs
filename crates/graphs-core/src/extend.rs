use trait_aliases::trait_aliases;

use crate::{
    build::{AddEdges, AddNodes, RecoverableEdge, RecoverableNode, TryAddEdges, TryAddNodes},
    edges::EdgeIn,
    nodes::NodeIn,
};

#[must_use = "extending returns iterators which have to be consumed"]
pub struct TryExtendIterNodes<'g, G: TryAddNodes + ?Sized, N: Iterator<Item = NodeIn<G>>> {
    graph: &'g mut G,
    nodes: N,
}

#[must_use = "extending returns iterators which have to be consumed"]
pub struct TryExtendIntoNodes<'g, G: TryAddNodes + ?Sized, I: Iterator>
where
    I::Item: Into<NodeIn<G>>,
{
    graph: &'g mut G,
    iterator: I,
}

impl<'g, G: TryAddNodes + ?Sized, N: Iterator<Item = NodeIn<G>>> TryExtendIterNodes<'g, G, N> {
    pub const fn new(graph: &'g mut G, nodes: N) -> Self {
        Self { graph, nodes }
    }
}

impl<'g, G: TryAddNodes + ?Sized, I: Iterator> TryExtendIntoNodes<'g, G, I>
where
    I::Item: Into<NodeIn<G>>,
{
    pub const fn new(graph: &'g mut G, iterator: I) -> Self {
        Self { graph, iterator }
    }
}

impl<G: TryAddNodes + ?Sized, N: Iterator<Item = NodeIn<G>>> Iterator
    for TryExtendIterNodes<'_, G, N>
{
    type Item = RecoverableNode<G>;

    fn next(&mut self) -> Option<Self::Item> {
        self.nodes.next().map(|node| self.graph.try_add_node(node))
    }
}

impl<G: TryAddNodes + ?Sized, I: Iterator> Iterator for TryExtendIntoNodes<'_, G, I>
where
    I::Item: Into<NodeIn<G>>,
{
    type Item = RecoverableNode<G>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator
            .next()
            .map(|item| self.graph.try_add_node(item.into()))
    }
}

#[must_use = "extending returns iterators which have to be consumed"]
pub struct ExtendIterNodes<'g, G: AddNodes + ?Sized, N: Iterator<Item = NodeIn<G>>> {
    graph: &'g mut G,
    nodes: N,
}

#[must_use = "extending returns iterators which have to be consumed"]
pub struct ExtendIntoNodes<'g, G: AddNodes + ?Sized, I: Iterator>
where
    I::Item: Into<NodeIn<G>>,
{
    graph: &'g mut G,
    iterator: I,
}

impl<'g, G: AddNodes + ?Sized, N: Iterator<Item = NodeIn<G>>> ExtendIterNodes<'g, G, N> {
    pub const fn new(graph: &'g mut G, nodes: N) -> Self {
        Self { graph, nodes }
    }
}

impl<'g, G: AddNodes + ?Sized, I: Iterator> ExtendIntoNodes<'g, G, I>
where
    I::Item: Into<NodeIn<G>>,
{
    pub const fn new(graph: &'g mut G, iterator: I) -> Self {
        Self { graph, iterator }
    }
}

impl<G: AddNodes + ?Sized, N: Iterator<Item = NodeIn<G>>> Iterator for ExtendIterNodes<'_, G, N> {
    type Item = G::NodeId;

    fn next(&mut self) -> Option<Self::Item> {
        self.nodes.next().map(|node| self.graph.add_node(node))
    }
}

impl<G: AddNodes + ?Sized, I: Iterator> Iterator for ExtendIntoNodes<'_, G, I>
where
    I::Item: Into<NodeIn<G>>,
{
    type Item = G::NodeId;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator
            .next()
            .map(|item| self.graph.add_node(item.into()))
    }
}

#[must_use = "extending returns iterators which have to be consumed"]
pub struct TryExtendIterEdges<'g, G: TryAddEdges + ?Sized, E: Iterator<Item = EdgeIn<G>>> {
    graph: &'g mut G,
    edges: E,
}

#[must_use = "extending returns iterators which have to be consumed"]
pub struct TryExtendIntoEdges<'g, G: TryAddEdges + ?Sized, I: Iterator>
where
    I::Item: Into<EdgeIn<G>>,
{
    graph: &'g mut G,
    iterator: I,
}

impl<'g, G: TryAddEdges + ?Sized, E: Iterator<Item = EdgeIn<G>>> TryExtendIterEdges<'g, G, E> {
    pub const fn new(graph: &'g mut G, edges: E) -> Self {
        Self { graph, edges }
    }
}

impl<'g, G: TryAddEdges + ?Sized, I: Iterator> TryExtendIntoEdges<'g, G, I>
where
    I::Item: Into<EdgeIn<G>>,
{
    pub const fn new(graph: &'g mut G, iterator: I) -> Self {
        Self { graph, iterator }
    }
}

impl<G: TryAddEdges + ?Sized, E: Iterator<Item = EdgeIn<G>>> Iterator
    for TryExtendIterEdges<'_, G, E>
{
    type Item = RecoverableEdge<G>;

    fn next(&mut self) -> Option<Self::Item> {
        self.edges.next().map(|edge| self.graph.try_add_edge(edge))
    }
}

impl<G: TryAddEdges + ?Sized, I: Iterator> Iterator for TryExtendIntoEdges<'_, G, I>
where
    I::Item: Into<EdgeIn<G>>,
{
    type Item = RecoverableEdge<G>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator
            .next()
            .map(|item| self.graph.try_add_edge(item.into()))
    }
}

#[must_use = "extending returns iterators which have to be consumed"]
pub struct ExtendIterEdges<'g, G: AddEdges + ?Sized, E: Iterator<Item = EdgeIn<G>>> {
    graph: &'g mut G,
    edges: E,
}

#[must_use = "extending returns iterators which have to be consumed"]
pub struct ExtendIntoEdges<'g, G: AddEdges + ?Sized, I: Iterator>
where
    I::Item: Into<EdgeIn<G>>,
{
    graph: &'g mut G,
    iterator: I,
}

impl<'g, G: AddEdges + ?Sized, E: Iterator<Item = EdgeIn<G>>> ExtendIterEdges<'g, G, E> {
    pub const fn new(graph: &'g mut G, edges: E) -> Self {
        Self { graph, edges }
    }
}

impl<'g, G: AddEdges + ?Sized, I: Iterator> ExtendIntoEdges<'g, G, I>
where
    I::Item: Into<EdgeIn<G>>,
{
    pub const fn new(graph: &'g mut G, iterator: I) -> Self {
        Self { graph, iterator }
    }
}

impl<G: AddEdges + ?Sized, E: Iterator<Item = EdgeIn<G>>> Iterator for ExtendIterEdges<'_, G, E> {
    type Item = G::EdgeId;

    fn next(&mut self) -> Option<Self::Item> {
        self.edges.next().map(|edge| self.graph.add_edge(edge))
    }
}

impl<G: AddEdges + ?Sized, I: Iterator> Iterator for ExtendIntoEdges<'_, G, I>
where
    I::Item: Into<EdgeIn<G>>,
{
    type Item = G::EdgeId;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator
            .next()
            .map(|item| self.graph.add_edge(item.into()))
    }
}

/// Represents graphs that can be extended with new nodes.
pub trait TryExtendNodes: TryAddNodes {
    /// Attempts to extend this graph with the given nodes.
    fn try_extend_nodes<N: IntoIterator<Item = NodeIn<Self>>>(
        &mut self,
        nodes: N,
    ) -> TryExtendIterNodes<'_, Self, N::IntoIter> {
        TryExtendIterNodes::new(self, nodes.into_iter())
    }

    /// Attempts to extend this graph with the given iterable of items that can be
    /// converted into nodes.
    fn try_extend_into_nodes<I: IntoIterator>(
        &mut self,
        iterable: I,
    ) -> TryExtendIntoNodes<'_, Self, I::IntoIter>
    where
        I::Item: Into<NodeIn<Self>>,
    {
        TryExtendIntoNodes::new(self, iterable.into_iter())
    }
}

impl<G: TryAddNodes + ?Sized> TryExtendNodes for G {}

/// Represents graphs that can be extended with new nodes where errors are not expected.
pub trait ExtendNodes: AddNodes {
    /// Extends this graph with the given nodes.
    fn extend_nodes<N: IntoIterator<Item = NodeIn<Self>>>(
        &mut self,
        nodes: N,
    ) -> ExtendIterNodes<'_, Self, N::IntoIter> {
        ExtendIterNodes::new(self, nodes.into_iter())
    }

    /// Extends this graph with the given iterable of items that can be converted into nodes.
    fn extend_into_nodes<I: IntoIterator>(
        &mut self,
        iterable: I,
    ) -> ExtendIntoNodes<'_, Self, I::IntoIter>
    where
        I::Item: Into<NodeIn<Self>>,
    {
        ExtendIntoNodes::new(self, iterable.into_iter())
    }
}

impl<G: AddNodes + ?Sized> ExtendNodes for G {}

/// Represents graphs that can be extended with new edges.
pub trait TryExtendEdges: TryAddEdges {
    /// Attempts to extend this graph with the given edges.
    fn try_extend_edges<E: IntoIterator<Item = EdgeIn<Self>>>(
        &mut self,
        edges: E,
    ) -> TryExtendIterEdges<'_, Self, E::IntoIter> {
        TryExtendIterEdges::new(self, edges.into_iter())
    }

    /// Attempts to extend this graph with the given iterable of items that can be
    /// converted into edges.
    fn try_extend_into_edges<I: IntoIterator>(
        &mut self,
        iterable: I,
    ) -> TryExtendIntoEdges<'_, Self, I::IntoIter>
    where
        I::Item: Into<EdgeIn<Self>>,
    {
        TryExtendIntoEdges::new(self, iterable.into_iter())
    }
}

impl<G: TryAddEdges + ?Sized> TryExtendEdges for G {}

/// Represents graphs that can be extended with new edges where errors are not expected.
pub trait ExtendEdges: AddEdges {
    /// Extends this graph with the given edges.
    fn extend_edges<E: IntoIterator<Item = EdgeIn<Self>>>(
        &mut self,
        edges: E,
    ) -> ExtendIterEdges<'_, Self, E::IntoIter> {
        ExtendIterEdges::new(self, edges.into_iter())
    }

    /// Extends this graph with the given iterable of items that can be converted into edges.
    fn extend_into_edges<I: IntoIterator>(
        &mut self,
        iterable: I,
    ) -> ExtendIntoEdges<'_, Self, I::IntoIter>
    where
        I::Item: Into<EdgeIn<Self>>,
    {
        ExtendIntoEdges::new(self, iterable.into_iter())
    }
}

impl<G: AddEdges + ?Sized> ExtendEdges for G {}

trait_aliases! {
    /// Represents graphs that can be extended with new nodes and edges.
    ///
    /// Implemented for any graph that implements both [`TryExtendNodes`] and [`TryExtendEdges`].
    #[trait_alias(G)]
    pub trait TryExtend = TryExtendNodes + TryExtendEdges;

    /// Represents graphs that can be extended with new nodes and edges
    /// where errors are not expected.
    ///
    /// Implemented for any graph that implements both [`ExtendNodes`] and [`ExtendEdges`].
    #[trait_alias(G)]
    pub trait Extend = ExtendNodes + ExtendEdges;
}
