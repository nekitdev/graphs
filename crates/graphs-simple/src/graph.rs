//! Simple graph implementation.

use core::marker::PhantomData;

use graphs_core::{
    base::Base,
    build::Build,
    capacity::{Capacities, EdgeCapacity, NodeCapacity},
    count::{Counts, EdgeCount, NodeCount},
    create::Create,
    data::{Data, DataMut, DataRef},
    direction::{Direction, Incoming, Outgoing},
    id::{DefaultId, EdgeId, NodeId},
    kind::{Directed, Kind, Kinded, Undirected},
};

use crate::{
    at_most_two::{AtMostTwo, AtMostTwoOutput},
    error::{EdgeError, NodeError},
    internal::{Edge, Edges, Node, Nodes},
};

/// Represents simple graphs.
pub struct Graph<N, E, K: Kind, I: IndexId = DefaultId> {
    nodes: Nodes<N, I>,
    edges: Edges<E, I>,
    kind: PhantomData<K>,
}

/// Represents the parts of the graph, namely nodes and edges.
pub type Parts<N, E, I = DefaultId> = (Nodes<N, I>, Edges<E, I>);

/// Represents simple directed graphs.
pub type DiGraph<N, E, I = DefaultId> = Graph<N, E, Directed, I>;

/// Represents simple undirected graphs.
pub type UnGraph<N, E, I = DefaultId> = Graph<N, E, Undirected, I>;

pub type DefaultDiGraph<N, E> = DiGraph<N, E, DefaultId>;
pub type DefaultUnGraph<N, E> = UnGraph<N, E, DefaultId>;

impl<N, E, K: Kind, I: IndexId> Kinded for Graph<N, E, K, I> {
    type Kind = K;
}

impl<N, E, K: Kind, I: IndexId> Graph<N, E, K, I> {
    /// Constructs [`Self`].
    pub const fn new() -> Self {
        Self {
            nodes: Nodes::new(),
            edges: Edges::new(),
            kind: PhantomData,
        }
    }

    /// Returns the node count of the graph.
    pub const fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Returns the edge count of the graph.
    pub const fn edge_count(&self) -> usize {
        self.edges.len()
    }

    pub const fn count(&self) -> Counts {
        Counts::new(self.node_count(), self.edge_count())
    }

    /// Returns the node capacity of the graph.
    pub const fn node_capacity(&self) -> usize {
        self.nodes.capacity()
    }

    /// Returns the edge capacity of the graph.
    pub const fn edge_capacity(&self) -> usize {
        self.edges.capacity()
    }

    pub const fn capacity(&self) -> Capacities {
        Capacities::new(self.node_capacity(), self.edge_capacity())
    }

    /// Returns whether the graph is directed.
    pub const fn is_directed(&self) -> bool {
        K::IS_DIRECTED
    }
}

impl<N, E, K: Kind, I: IndexId> Base for Graph<N, E, K, I> {
    type NodeId = NodeId<I>;
    type EdgeId = EdgeId<I>;
}

impl<N, E, K: Kind, I: IndexId> Data for Graph<N, E, K, I> {
    type NodeValue = N;
    type EdgeValue = E;
}

impl<N, E, K: Kind, I: IndexId> NodeCount for Graph<N, E, K, I> {
    fn node_count(&self) -> usize {
        self.node_count()
    }
}

impl<N, E, K: Kind, I: IndexId> EdgeCount for Graph<N, E, K, I> {
    fn edge_count(&self) -> usize {
        self.edge_count()
    }
}

impl<N, E, K: Kind, I: IndexId> NodeCapacity for Graph<N, E, K, I> {
    fn node_capacity(&self) -> usize {
        self.node_capacity()
    }
}

impl<N, E, K: Kind, I: IndexId> EdgeCapacity for Graph<N, E, K, I> {
    fn edge_capacity(&self) -> usize {
        self.edge_capacity()
    }
}

impl<N, E, K: Kind, I: IndexId> Create for Graph<N, E, K, I> {
    fn new() -> Self {
        Self::new()
    }

    fn with_capacity(capacities: Capacities) -> Self {
        Self {
            nodes: Nodes::with_capacity(capacities.nodes),
            edges: Edges::with_capacity(capacities.edges),
            kind: PhantomData,
        }
    }
}

impl<N, E, K: Kind, I: IndexId> Build for Graph<N, E, K, I> {
    type NodeError = NodeError;
    type EdgeError = EdgeError;

    fn try_add_node(&mut self, value: Self::NodeValue) -> Result<Self::NodeId, Self::NodeError> {
        todo!()
    }

    fn try_add_edge(
        &mut self,
        source: Self::NodeId,
        target: Self::NodeId,
        value: Self::EdgeValue,
    ) -> Result<Self::EdgeId, Self::EdgeError> {
        todo!()
    }
}

impl<N, E, K: Kind, I: IndexId> DataRef for Graph<N, E, K, I> {
    fn node_value(&self, id: Self::NodeId) -> Option<&Self::NodeValue> {
        self.node_at(id).map(|node| node.get())
    }

    fn edge_value(&self, id: Self::EdgeId) -> Option<&Self::EdgeValue> {
        self.edge_at(id).map(|edge| edge.get())
    }
}

impl<N, E, K: Kind, I: IndexId> DataMut for Graph<N, E, K, I> {
    fn node_value_mut(&mut self, id: Self::NodeId) -> Option<&mut Self::NodeValue> {
        self.node_at_mut(id).map(|node| node.get_mut())
    }

    fn edge_value_mut(&mut self, id: Self::EdgeId) -> Option<&mut Self::EdgeValue> {
        self.edge_at_mut(id).map(|edge| edge.get_mut())
    }
}

impl<N, E, I: IndexId> DiGraph<N, E, I> {
    /// Constructs directed [`Self`].
    pub const fn directed() -> Self {
        Self::new()
    }
}

impl<N, E, I: IndexId> UnGraph<N, E, I> {
    /// Constructs undirected [`Self`].
    pub const fn undirected() -> Self {
        Self::new()
    }
}

impl<N, E, K: Kind, I: IndexId> Default for Graph<N, E, K, I> {
    fn default() -> Self {
        Self::new()
    }
}

impl<N, E, K: Kind, I: IndexId> Graph<N, E, K, I> {
    fn at_most_two(
        &mut self,
        source: NodeId<I>,
        target: NodeId<I>,
    ) -> AtMostTwoOutput<&mut Node<N, I>> {
        self.nodes.at_most_two(source.index(), target.index())
    }

    fn node_at(&self, index: NodeId<I>) -> Option<&Node<N, I>> {
        self.nodes.get(index.index())
    }

    fn node_at_mut(&mut self, index: NodeId<I>) -> Option<&mut Node<N, I>> {
        self.nodes.get_mut(index.index())
    }

    fn edge_at(&self, index: NodeId<I>) -> Option<&Edge<E, I>> {
        self.edges.get(index.index())
    }

    fn edge_at_mut(&mut self, index: NodeId<I>) -> Option<&mut Edge<E, I>> {
        self.edges.get_mut(index.index())
    }
}

/*
impl<N, E, K: Kind, I: Id> Graph<N, E, K, I> {
    pub fn connection(&self, index: EdgeIndex<I>) -> Option<Connecting<I>> {
        self.edge_at(index).map(|edge| edge.connecting.copy())
    }

    /// Panicking version of [`try_add_node`].
    ///
    /// # Panics
    ///
    /// This method panics if the node limit is reached.
    ///
    /// [`try_add_node`]: Self::try_add_node
    pub fn add_node(&mut self, value: N) -> NodeIndex<I> {
        self.try_add_node(value).expect(failed!(add_node))
    }

    /// Attempts to add new node with the given value to the graph.
    ///
    /// # Errors
    ///
    /// Returns [`struct@Error`] if the node limit is reached.
    ///
    /// The passed value can be extracted back from the error.
    pub fn try_add_node(&mut self, value: N) -> Result<NodeIndex<I>, N> {
        let index = NodeIndex::of(self.node_count());

        if index.is_limit() {
            return Err(Error::node_limit(value));
        }

        let node = Node::new(value);

        self.nodes.push(node);

        Ok(index)
    }

    /// Panicking version of [`try_add_edge`].
    ///
    /// # Panics
    ///
    /// This method panics in two cases:
    ///
    /// - the edge limit is reached;
    /// - provided indices (or either index) are out of bounds.
    ///
    /// [`try_add_edge`]: Self::try_add_edge
    pub fn add_edge(
        &mut self,
        source: NodeIndex<I>,
        target: NodeIndex<I>,
        value: E,
    ) -> EdgeIndex<I> {
        self.try_add_edge(source, target, value)
            .expect(failed!(add_edge))
    }

    /// Attempts to add new edge connecting the given source and target nodes.
    ///
    /// # Errors
    ///
    /// Returns [`struct@Error`] if:
    ///
    /// - the edge limit is reached;
    /// - provided indices (or either index) are out of bounds.
    ///
    /// The passed value can be extracted back from the error.
    pub fn try_add_edge(
        &mut self,
        source: NodeIndex<I>,
        target: NodeIndex<I>,
        value: E,
    ) -> Result<EdgeIndex<I>, E> {
        let index = EdgeIndex::of(self.edge_count());

        if index.is_limit() {
            return Err(Error::edge_limit(value));
        }

        let edge = match self.at_most_two(source, target) {
            Output::None => return Err(Error::out_of_bounds(value)),
            Output::One(node) => {
                let mut edge = Edge::connecting(source, target, value);

                edge.next = node.next.replace_same(index);

                edge
            }
            Output::Two(one, two) => {
                let mut edge = Edge::connecting(source, target, value);

                edge.next = Next::new(
                    one.next.replace_outgoing(index),
                    two.next.replace_incoming(index),
                );

                edge
            }
        };

        self.edges.push(edge);

        Ok(index)
    }

    /// Panicking version of [`try_update_edge`].
    ///
    /// # Panics
    ///
    /// This method panics if the [`try_add_edge`] method is called
    /// and fails within [`try_update_edge`].
    ///
    /// [`try_add_edge`]: Self::try_add_edge
    pub fn update_edge(
        &mut self,
        source: NodeIndex<I>,
        target: NodeIndex<I>,
        value: E,
    ) -> Update<E, I> {
        self.try_update_edge(source, target, value)
            .expect(failed!(update_edge))
    }

    /// Adds or updates the edge connecting the given source and target nodes with the given value.
    ///
    /// If the edge already exists, its value is replaced with the provided one and the previous
    /// value is returned as part of the [`Update`] struct.
    ///
    /// Otherwise, new edge is added to the graph via [`try_add_edge`].
    ///
    /// # Errors
    ///
    /// [`struct@Error`] is propagated from the [`try_add_edge`] method.
    ///
    /// [`try_add_edge`]: Self::try_add_edge
    pub fn try_update_edge(
        &mut self,
        source: NodeIndex<I>,
        target: NodeIndex<I>,
        value: E,
    ) -> Result<Update<E, I>, E> {
        if let Some(index) = self.find_edge(source, target) {
            if let Some(edge) = self.edge_at_mut(index) {
                let taken = replace(edge.get_mut(), value);

                return Ok(Update::taken(index, taken));
            };
        };

        let index = self.try_add_edge(source, target, value)?;

        Ok(Update::not_taken(index))
    }

    /// Checks whether there is some edge connecting the given source and target nodes.
    pub fn contains_edge(&self, source: NodeIndex<I>, target: NodeIndex<I>) -> bool {
        self.find_edge(source, target).is_some()
    }

    /// Finds the edge connecting the given source and target nodes, if there is one.
    pub fn find_edge(&self, source: NodeIndex<I>, target: NodeIndex<I>) -> Option<EdgeIndex<I>> {
        if self.is_directed() {
            self.directed_find_edge(source, target)
        } else {
            self.undirected_find_edge(source, target)
        }
    }

    pub fn remove_node(&mut self, index: NodeIndex<I>) -> Option<N> {
        todo!()
    }

    pub fn remove_edge(&mut self, index: EdgeIndex<I>) -> Option<E> {
        todo!()
    }

    /// Consumes the graph and returns its parts (nodes and edges).
    pub fn into_parts(self) -> Parts<N, E, I> {
        (self.nodes, self.edges)
    }

    /// Reverses the graph.
    pub fn reverse(&mut self) {
        self.nodes.iter_mut().for_each(|node| node.next.reverse());

        self.edges.iter_mut().for_each(|edge| {
            edge.connecting.reverse();
            edge.next.reverse();
        });
    }

    /// Clears the graph, removing all nodes and edges.
    pub fn clear(&mut self) {
        self.nodes.clear();
        self.edges.clear();
    }

    /// Clears the edges of the graph while keeping the nodes.
    pub fn clear_edges(&mut self) {
        // reset next indices of nodes
        self.nodes
            .iter_mut()
            .for_each(|node| node.next = Next::limit());

        self.edges.clear();
    }
}

impl<N, E, K: Kind, I: Id> Graph<N, E, K, I> {
    fn directed_find_edge(
        &self,
        source: NodeIndex<I>,
        target: NodeIndex<I>,
    ) -> Option<EdgeIndex<I>> {
        let node = self.node_at(source)?;

        let mut index = node.next.outgoing;

        while let Some(edge) = self.edge_at(index) {
            if edge.connecting.target == target {
                return Some(index);
            }

            index = edge.next.outgoing;
        }

        None
    }

    fn undirected_find_edge(
        &self,
        source: NodeIndex<I>,
        target: NodeIndex<I>,
    ) -> Option<EdgeIndex<I>> {
        let node = self.node_at(source)?;

        for direction in Direction::ARRAY {
            let mut index = node.next.directed(direction);

            while let Some(edge) = self.edge_at(index) {
                if edge.connecting.directed(direction) == target {
                    return Some(index);
                }

                index = edge.next.directed(direction);
            }
        }

        None
    }
}
*/
