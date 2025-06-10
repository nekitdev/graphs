//! Simple graph implementation.

use core::{marker::PhantomData, mem::replace};

use graphs_core::{
    base::Base,
    direction::{Direction, Incoming, Outgoing},
    id::{DefaultId, Id},
    kind::{Directed, Kind, Kinded, Undirected},
};

use crate::{
    at_most_two::{AtMostTwo, Output},
    error::{Error, Result},
    index::{EdgeIndex, NodeIndex},
    internal::{Connecting, Edge, Edges, Node, Nodes},
    next::Next,
};

/// Represents simple graphs.
pub struct Graph<N, E, K: Kind, I: Id = DefaultId> {
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

impl<N, E, K: Kind, I: Id> Kinded for Graph<N, E, K, I> {
    type Kind = K;
}

pub struct Capacities {
    pub node: usize,
    pub edge: usize,
}

impl Capacities {
    pub const fn new(node: usize, edge: usize) -> Self {
        Self { node, edge }
    }
}

impl<N, E, K: Kind, I: Id> Graph<N, E, K, I> {
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

impl<N, E, K: Kind, I: Id> Graph<N, E, K, I> {
    /// Constructs [`Self`], reserving [`Capacities`] for nodes and edges.
    pub fn with_capacity(capacities: Capacities) -> Self {
        Self {
            nodes: Nodes::with_capacity(capacities.node),
            edges: Edges::with_capacity(capacities.edge),
            kind: PhantomData,
        }
    }
}

impl<N, E, I: Id> DiGraph<N, E, I> {
    /// Constructs directed [`Self`].
    pub const fn directed() -> Self {
        Self::new()
    }
}

impl<N, E, I: Id> UnGraph<N, E, I> {
    /// Constructs undirected [`Self`].
    pub const fn undirected() -> Self {
        Self::new()
    }
}

// TODO: direction + capacity

impl<N, E, K: Kind, I: Id> Default for Graph<N, E, K, I> {
    fn default() -> Self {
        Self::new()
    }
}

impl<N, E, K: Kind, I: Id> Graph<N, E, K, I> {
    fn at_most_two(
        &mut self,
        source: NodeIndex<I>,
        target: NodeIndex<I>,
    ) -> Output<&mut Node<N, I>> {
        self.nodes.at_most_two(source.index(), target.index())
    }

    fn node_at(&self, index: NodeIndex<I>) -> Option<&Node<N, I>> {
        self.nodes.get(index.index())
    }

    fn node_at_mut(&mut self, index: NodeIndex<I>) -> Option<&mut Node<N, I>> {
        self.nodes.get_mut(index.index())
    }

    fn edge_at(&self, index: EdgeIndex<I>) -> Option<&Edge<E, I>> {
        self.edges.get(index.index())
    }

    fn edge_at_mut(&mut self, index: EdgeIndex<I>) -> Option<&mut Edge<E, I>> {
        self.edges.get_mut(index.index())
    }
}

/// Represents edge updates in the graph.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Update<T, I: Id = DefaultId> {
    /// The index of the updated edge.
    pub index: EdgeIndex<I>,
    /// The previous value of the edge, if it was taken.
    pub previous: Option<T>,
}

impl<T, I: Id> Update<T, I> {
    /// Constructs [`Self`].
    pub const fn new(index: EdgeIndex<I>, previous: Option<T>) -> Self {
        Self { index, previous }
    }

    /// Constructs [`Self`] for the case when the value was *not* taken.
    pub const fn not_taken(index: EdgeIndex<I>) -> Self {
        Self::new(index, None)
    }

    /// Constructs [`Self`] for the case when the `value` was taken.
    pub const fn taken(index: EdgeIndex<I>, value: T) -> Self {
        Self::new(index, Some(value))
    }
}

macro_rules! failed {
    ($name: ident) => {
        concat!("`", stringify!($name), "` failed")
    };
}

impl<N, E, K: Kind, I: Id> Graph<N, E, K, I> {
    pub fn node_value(&self, index: NodeIndex<I>) -> Option<&N> {
        self.node_at(index).map(|node| node.get())
    }

    pub fn node_value_mut(&mut self, index: NodeIndex<I>) -> Option<&mut N> {
        self.node_at_mut(index).map(|node| node.get_mut())
    }

    pub fn edge_value(&self, index: EdgeIndex<I>) -> Option<&E> {
        self.edge_at(index).map(|edge| edge.get())
    }

    pub fn edge_value_mut(&mut self, index: EdgeIndex<I>) -> Option<&mut E> {
        self.edge_at_mut(index).map(|edge| edge.get_mut())
    }

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

impl<N, E, K: Kind, I: Id> Base for Graph<N, E, K, I> {
    type NodeId = NodeIndex<I>;
    type EdgeId = EdgeIndex<I>;
}
