use std::option::IntoIter;

use graphs_assert::{assert_looped, assert_multi, assert_pseudo, assert_simple, assert_undirected};
use graphs_common::index::NodeIndex;
use graphs_core::{
    attached::Edges,
    build::{RecoverableEdge, TryAddEdges},
    connections::Connection,
    direction::Direction,
    edges::EdgeIn,
    find::{Find, Found, or_missing},
    index::{DefaultUntypedIndex, Index, UntypedIndex},
    kinds::Undirected,
    loops::{Allow, DefaultLoop, Forbid, Loop},
    recoverable, recoverable_return,
    references::{EdgeRef, EdgeReferences},
    sentinel::Sentinel,
    types::{DefaultType, Multiple, Single, Type},
};

use crate::{
    errors::{LoopError, LoopedError, MultipleError, SimpleError, edge_index},
    generic::{GenericGraph, MaybePair},
    parts::{Edge, EdgeSlice, Next},
    references::UndirectedEdgeRef,
};

/// Represents undirected graphs.
pub type Graph<N, E, I = DefaultUntypedIndex, T = DefaultType, L = DefaultLoop> =
    GenericGraph<N, E, I, Undirected, T, L>;

/// Represents undirected simple graphs.
pub type SimpleGraph<N, E, I = DefaultUntypedIndex> = Graph<N, E, I, Single, Forbid>;

/// Represents undirected looped graphs.
pub type LoopedGraph<N, E, I = DefaultUntypedIndex> = Graph<N, E, I, Single, Allow>;

/// Represents undirected multi graphs.
pub type MultiGraph<N, E, I = DefaultUntypedIndex> = Graph<N, E, I, Multiple, Forbid>;

/// Represents undirected pseudo graphs.
pub type PseudoGraph<N, E, I = DefaultUntypedIndex> = Graph<N, E, I, Multiple, Allow>;

pub type SingleTypeGraph<N, E, I = DefaultUntypedIndex, L = DefaultLoop> =
    Graph<N, E, I, Single, L>;

pub type MultipleTypeGraph<N, E, I = DefaultUntypedIndex, L = DefaultLoop> =
    Graph<N, E, I, Multiple, L>;

pub type AllowLoopGraph<N, E, I = DefaultUntypedIndex, T = DefaultType> = Graph<N, E, I, T, Allow>;

pub type ForbidLoopGraph<N, E, I = DefaultUntypedIndex, T = DefaultType> =
    Graph<N, E, I, T, Forbid>;

assert_undirected!(Graph<N, E, I: UntypedIndex, T: Type, L: Loop>);
assert_simple!(SimpleGraph<N, E, I: UntypedIndex>);
assert_looped!(LoopedGraph<N, E, I: UntypedIndex>);
assert_multi!(MultiGraph<N, E, I: UntypedIndex>);
assert_pseudo!(PseudoGraph<N, E, I: UntypedIndex>);

pub struct Maybe<'g, G: EdgeReferences + 'g> {
    iterator: IntoIter<G::EdgeRef<'g>>,
}

impl<'g, G: EdgeReferences + 'g> Maybe<'g, G> {
    pub fn new(option: Option<G::EdgeRef<'g>>) -> Self {
        let iterator = option.into_iter();

        Self { iterator }
    }

    pub fn some(edge: G::EdgeRef<'g>) -> Self {
        Self::new(Some(edge))
    }

    pub fn none() -> Self {
        Self::new(None)
    }
}

impl<'g, G: EdgeReferences + 'g> Iterator for Maybe<'g, G> {
    type Item = G::EdgeRef<'g>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator.next()
    }
}

impl<N, E, I: UntypedIndex, L: Loop> Find for SingleTypeGraph<N, E, I, L> {
    type Connecting<'g>
        = Maybe<'g, Self>
    where
        Self: 'g;

    fn find(&self, connection: Self::Connection) -> Found<'_, Self> {
        let (one_index, two_index) = connection.parts();

        let (node, _) = or_missing(self.node(one_index), self.node(two_index), connection)?;

        for direction in Direction::ALL {
            let reversed = direction.reversed();

            let mut index = node.next.directed(direction);

            while let Some(edge) = self.edge(index) {
                if edge.connection.directed(reversed) == two_index {
                    return Ok(Self::Connecting::some(edge.as_ref_with(index)));
                }

                index = edge.next.directed(direction);
            }
        }

        Ok(Self::Connecting::none())
    }
}

impl<N, E, I: UntypedIndex, L: Loop> Find for MultipleTypeGraph<N, E, I, L> {
    type Connecting<'g>
        = Connecting<'g, E, I>
    where
        Self: 'g;

    fn find(&self, connection: Self::Connection) -> Found<'_, Self> {
        let (node, connect) = connection.parts();

        let _ = or_missing(self.node(node), self.node(connect), connection)?;

        Ok(self.edges(node).connecting(connect))
    }
}

pub struct EdgeIterator<'a, E, I: UntypedIndex = DefaultUntypedIndex> {
    index: NodeIndex<I>,
    edges: EdgeSlice<'a, E, I, Undirected>,
    next: Next<I>,
}

impl<'a, E, I: UntypedIndex> EdgeIterator<'a, E, I> {
    pub fn connecting(self, connect: NodeIndex<I>) -> Connecting<'a, E, I> {
        Connecting {
            connect,
            iterator: self,
        }
    }
}

pub struct Connecting<'a, E, I: UntypedIndex = DefaultUntypedIndex> {
    connect: NodeIndex<I>,
    iterator: EdgeIterator<'a, E, I>,
}

impl<'a, E, I: UntypedIndex> Iterator for Connecting<'a, E, I> {
    type Item = UndirectedEdgeRef<'a, E, I>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator
            .find(|node| node.connection.two == self.connect)
    }
}

impl<'a, E, I: UntypedIndex> Iterator for EdgeIterator<'a, E, I> {
    type Item = UndirectedEdgeRef<'a, E, I>;

    fn next(&mut self) -> Option<Self::Item> {
        let outgoing = self.next.outgoing;

        if outgoing.is_regular()
            && let Some(edge) = self.edges.get(outgoing.index())
        {
            self.next.outgoing = edge.next.outgoing;

            return Some(edge.as_ref_with(outgoing));
        }

        let incoming = self.next.incoming;

        while incoming.is_regular()
            && let Some(edge) = self.edges.get(incoming.index())
        {
            self.next.incoming = edge.next.incoming;

            // self-loop, already returned before
            if edge.connection.one == self.index {
                continue;
            }

            return Some(edge.as_ref_with(incoming));
        }

        None
    }
}

impl<N, E, I: UntypedIndex, T: Type, L: Loop> Edges for Graph<N, E, I, T, L> {
    type EdgeIterator<'g>
        = EdgeIterator<'g, E, I>
    where
        Self: 'g;

    fn edges(&self, index: Self::NodeId) -> Self::EdgeIterator<'_> {
        let next = self
            .node(index)
            .map(|node| node.next)
            .unwrap_or(Next::SENTINEL);

        let edges = self.edges_ref();

        Self::EdgeIterator { index, edges, next }
    }

    fn has_edges(&self, node: Self::NodeId) -> bool {
        todo!()
    }
}

impl<N, E, I: UntypedIndex> TryAddEdges for SimpleGraph<N, E, I> {
    type Error = SimpleError<I, Undirected>;

    fn try_add_edge(&mut self, edge: EdgeIn<Self>) -> RecoverableEdge<Self> {
        let (connection, value) = edge.get();

        if connection.is_loop() {
            return recoverable!(LoopError::new(connection), value);
        }

        if let Some(edge) = recoverable_return!(self.find(connection), value).next() {
            return recoverable!(MultipleError::new(edge.id(), edge.connection()), value);
        }

        let size = self.size();

        let index = recoverable_return!(edge_index(size), value);

        let mut edge = Edge::new(value, connection);

        let (one, two) = connection.parts();

        // SAFETY: non-looped connection, both nodes exist
        let (one_node, two_node) = unsafe { self.node_pair_mut(one, two) };

        edge.next = Next::new(
            one_node.next.replace_outgoing(index),
            two_node.next.replace_incoming(index),
        );

        self.edges.push(edge);

        Ok(index)
    }
}

impl<N, E, I: UntypedIndex> TryAddEdges for LoopedGraph<N, E, I> {
    type Error = LoopedError<I, Undirected>;

    fn try_add_edge(&mut self, edge: EdgeIn<Self>) -> RecoverableEdge<Self> {
        let (connection, value) = edge.get();

        if let Some(edge) = recoverable_return!(self.find(connection), value).next() {
            return recoverable!(MultipleError::new(edge.id(), edge.connection()), value);
        }

        let size = self.size();

        let index = recoverable_return!(edge_index(size), value);

        let mut edge = Edge::new(value, connection);

        let (one, two) = connection.parts();

        let option = self.node_twice_mut(one, two);

        // SAFETY: both nodes exist (possibly looped)
        let maybe_pair = unsafe { option.unwrap_unchecked() };

        match maybe_pair {
            MaybePair::One(node) => {
                edge.next = Next::new(
                    node.next.replace_outgoing(index),
                    node.next.replace_incoming(index),
                );
            }
            MaybePair::Two(one_node, two_node) => {
                edge.next = Next::new(
                    one_node.next.replace_outgoing(index),
                    two_node.next.replace_incoming(index),
                );
            }
        }

        self.edges.push(edge);

        Ok(index)
    }
}

// use core::marker::PhantomData;

// use graphs_core::{
//     base::Base,
//     build::Build,
//     capacity::{Capacities, EdgeCapacity, NodeCapacity},
//     clear::{Clear, ClearEdges},
//     connection::Connection,
//     count::{Counts, EdgeCount, NodeCount},
//     create::Create,
//     data::{Data, DataMut, DataRef},
//     direction::Direction::{self, Outgoing},
//     find::{Find, FindDirected},
//     id::{Id, NodeId},
//     index::{DefaultUntypedIndex, EdgeIndex, Index, NodeIndex, UntypedIndex},
//     indexed::{EdgeIndexed, NodeIndexed},
//     kinds::{DefaultKind, Directed, Kind, Undirected},
//     loops::{Allow, DefaultLoop, Forbid, Loop},
//     recoverable::RecoverableResult,
//     recoverable_result,
//     reverse::Reverse,
//     specs::Specs,
//     types::{DefaultType, Multiple, Single, Type},
// };
// use thiserror::Error;

// use crate::internal::{Edge, Edges, InternalConnection, Node, Nodes};

// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Error)]
// #[error("node limit reached")]
// pub struct NodeError;

// impl NodeError {
//     /// Constructs [`Self`].
//     pub const fn new() -> Self {
//         Self
//     }
// }

// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Error)]
// #[error("edge limit reached")]
// pub struct LimitError;

// impl LimitError {
//     /// Constructs [`Self`].
//     pub const fn new() -> Self {
//         Self
//     }
// }

// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Error)]
// #[error("node {index} is missing")]
// pub struct MissingError<I: UntypedIndex = DefaultUntypedIndex> {
//     pub index: NodeIndex<I>,
// }

// impl<I: UntypedIndex> MissingError<I> {
//     /// Constructs [`Self`].
//     pub const fn new(index: NodeIndex<I>) -> Self {
//         Self { index }
//     }
// }

// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Error)]
// #[error("loop {connection} is forbidden")]
// pub struct LoopError<I: UntypedIndex = DefaultUntypedIndex> {
//     pub connection: Connection<NodeIndex<I>>,
// }

// impl<I: UntypedIndex> LoopError<I> {
//     /// Constructs [`Self`].
//     pub const fn new(connection: Connection<NodeIndex<I>>) -> Self {
//         Self { connection }
//     }
// }

// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Error)]
// #[error("edge {connection} already exists")]
// pub struct MultipleError<I: UntypedIndex = DefaultUntypedIndex> {
//     pub connection: Connection<NodeIndex<I>>,
// }

// impl<I: UntypedIndex> MultipleError<I> {
//     /// Constructs [`Self`].
//     pub const fn new(connection: Connection<NodeIndex<I>>) -> Self {
//         Self { connection }
//     }
// }

// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Error)]
// #[error(transparent)]
// pub enum EdgeError<I: UntypedIndex = DefaultUntypedIndex> {
//     Limit(#[from] LimitError),
//     Missing(#[from] MissingError<I>),
//     Loop(#[from] LoopError<I>),
//     Multiple(#[from] MultipleError<I>),
// }

// impl<I: UntypedIndex> EdgeError<I> {
//     pub const fn limit() -> Self {
//         Self::Limit(LimitError::new())
//     }

//     pub const fn missing(index: NodeIndex<I>) -> Self {
//         Self::Missing(MissingError::new(index))
//     }

//     pub const fn self_loop(connection: Connection<NodeIndex<I>>) -> Self {
//         Self::Loop(LoopError::new(connection))
//     }

//     pub const fn multiple(connection: Connection<NodeIndex<I>>) -> Self {
//         Self::Multiple(MultipleError::new(connection))
//     }
// }

// /// Represents simple graphs.
// pub struct Generic<
//     N,
//     E,
//     K: Kind = DefaultKind,
//     I: UntypedIndex = DefaultUntypedIndex,
//     T: Type = DefaultType,
//     L: Loop = DefaultLoop,
// > {
//     nodes: Nodes<N, I>,
//     edges: Edges<E, I>,
//     specs: PhantomData<Specs<K, T, L>>,
// }

// /// Represents the parts of the graph, namely nodes and edges.
// pub type Parts<N, E, I = DefaultUntypedIndex> = (Nodes<N, I>, Edges<E, I>);

// pub type Graph<N, E, I = DefaultUntypedIndex, T = DefaultType, L = DefaultLoop> =
//     Generic<N, E, Undirected, I, T, L>;

// pub type DiGraph<N, E, I = DefaultUntypedIndex, T = DefaultType, L = DefaultLoop> =
//     Generic<N, E, Directed, I, T, L>;

// pub type SimpleGraph<N, E, I = DefaultUntypedIndex> = Graph<N, E, I, Single, Forbid>;
// pub type LoopedGraph<N, E, I = DefaultUntypedIndex> = Graph<N, E, I, Single, Allow>;
// pub type MultiGraph<N, E, I = DefaultUntypedIndex> = Graph<N, E, I, Multiple, Forbid>;
// pub type PseudoGraph<N, E, I = DefaultUntypedIndex> = Graph<N, E, I, Multiple, Allow>;

// pub type SimpleDiGraph<N, E, I = DefaultUntypedIndex> = DiGraph<N, E, I, Single, Forbid>;
// pub type LoopedDiGraph<N, E, I = DefaultUntypedIndex> = DiGraph<N, E, I, Single, Allow>;
// pub type MultiDiGraph<N, E, I = DefaultUntypedIndex> = DiGraph<N, E, I, Multiple, Forbid>;
// pub type PseudoDiGraph<N, E, I = DefaultUntypedIndex> = DiGraph<N, E, I, Multiple, Allow>;

// impl<N, E, K: Kind, I: UntypedIndex, T: Type, L: Loop> Base for Generic<N, E, K, I, T, L> {
//     type NodeId = NodeIndex<I>;
//     type EdgeId = EdgeIndex<I>;

//     type Kind = K;
//     type Loop = L;
//     type Type = T;
// }

// impl<N, E, K: Kind, I: UntypedIndex, T: Type, L: Loop> Generic<N, E, K, I, T, L> {
//     /// Constructs [`Self`].
//     pub const fn new() -> Self {
//         Self {
//             nodes: Nodes::new(),
//             edges: Edges::new(),
//             specs: PhantomData,
//         }
//     }

//     /// Returns the node count of the graph.
//     pub const fn node_count(&self) -> usize {
//         self.nodes.len()
//     }

//     /// Returns the edge count of the graph.
//     pub const fn edge_count(&self) -> usize {
//         self.edges.len()
//     }

//     pub const fn count(&self) -> Counts {
//         Counts::new(self.node_count(), self.edge_count())
//     }

//     /// Returns the node capacity of the graph.
//     pub const fn node_capacity(&self) -> usize {
//         self.nodes.capacity()
//     }

//     /// Returns the edge capacity of the graph.
//     pub const fn edge_capacity(&self) -> usize {
//         self.edges.capacity()
//     }

//     pub const fn capacity(&self) -> Capacities {
//         Capacities::new(self.node_capacity(), self.edge_capacity())
//     }
// }

// impl<N, E, K: Kind, I: UntypedIndex, T: Type, L: Loop> Data for Generic<N, E, K, I, T, L> {
//     type NodeValue = N;
//     type EdgeValue = E;
// }

// impl<N, E, K: Kind, I: UntypedIndex, T: Type, L: Loop> NodeCount for Generic<N, E, K, I, T, L> {
//     fn node_count(&self) -> usize {
//         self.node_count()
//     }
// }

// impl<N, E, K: Kind, I: UntypedIndex, T: Type, L: Loop> EdgeCount for Generic<N, E, K, I, T, L> {
//     fn edge_count(&self) -> usize {
//         self.edge_count()
//     }
// }

// impl<N, E, K: Kind, I: UntypedIndex, T: Type, L: Loop> NodeCapacity for Generic<N, E, K, I, T, L> {
//     fn node_capacity(&self) -> usize {
//         self.node_capacity()
//     }
// }

// impl<N, E, K: Kind, I: UntypedIndex, T: Type, L: Loop> EdgeCapacity for Generic<N, E, K, I, T, L> {
//     fn edge_capacity(&self) -> usize {
//         self.edge_capacity()
//     }
// }

// impl<N, E, K: Kind, I: UntypedIndex, T: Type, L: Loop> NodeIndexed for Generic<N, E, K, I, T, L> {
//     fn node_bound(&self) -> usize {
//         self.node_count()
//     }

//     fn node_index(&self, id: Self::NodeId) -> usize {
//         id.index()
//     }

//     fn node_id(&self, index: usize) -> Self::NodeId {
//         Self::NodeId::of(index)
//     }
// }

// impl<N, E, K: Kind, I: UntypedIndex, T: Type, L: Loop> EdgeIndexed for Generic<N, E, K, I, T, L> {
//     fn edge_bound(&self) -> usize {
//         self.edge_count()
//     }

//     fn edge_index(&self, id: Self::EdgeId) -> usize {
//         id.index()
//     }

//     fn edge_id(&self, index: usize) -> Self::EdgeId {
//         Self::EdgeId::of(index)
//     }
// }

// impl<N, E, K: Kind, I: UntypedIndex, T: Type, L: Loop> Create for Generic<N, E, K, I, T, L> {
//     fn empty() -> Self {
//         Self::new()
//     }

//     fn with_capacity(capacities: Capacities) -> Self {
//         Self {
//             nodes: Nodes::with_capacity(capacities.nodes),
//             edges: Edges::with_capacity(capacities.edges),
//             specs: PhantomData,
//         }
//     }
// }

// impl<N, E, I: UntypedIndex> Build for SimpleGraph<N, E, I> {
//     type NodeError = NodeError;
//     type EdgeError = EdgeError<I>;

//     fn add_node(
//         &mut self,
//         value: Self::NodeValue,
//     ) -> RecoverableResult<Self::NodeId, Self::NodeError, Self::NodeValue> {
//         let index = NodeIndex::of(self.node_count());

//         if index.is_limit() {
//             return recoverable_result!(Self::NodeError::new(), value);
//         }

//         let node = Node::new(value);

//         self.nodes.push(node);

//         Ok(index)
//     }

//     fn add_edge(
//         &mut self,
//         connection: Connection<Self::NodeId>,
//         value: Self::EdgeValue,
//     ) -> RecoverableResult<Self::EdgeId, Self::EdgeError, Self::EdgeValue> {
//         todo!()
//     }
// }

// impl<N, E, I: UntypedIndex> Build for LoopedGraph<N, E, I> {}

// impl<N, E, K: Kind, I: UntypedIndex, T: Type, L: Loop> Build for Graph<N, E, K, I, T, L> {
//     type NodeError = NodeError;
//     type EdgeError = EdgeError<I>;

//     fn add_node(
//         &mut self,
//         value: Self::NodeValue,
//     ) -> RecoverableResult<Self::NodeId, Self::NodeError, Self::NodeValue> {
//         let index = NodeIndex::of(self.node_count());

//         if index.is_limit() {
//             return recoverable_result!(Self::NodeError::new(), value);
//         }

//         let node = Node::new(value);

//         self.nodes.push(node);

//         Ok(index)
//     }

//     fn add_edge(
//         &mut self,
//         connection: Connection<Self::NodeId>,
//         value: Self::EdgeValue,
//     ) -> RecoverableResult<Self::Output, Self::EdgeError, Self::EdgeValue> {
//         if self.is_forbid() && connection.is_loop() {
//             return recoverable_result!(Self::EdgeError::self_loop(connection), value);
//         }

//         if self.is_single() {
//             todo!() // TODO: handle multiple edges
//         }

//         let index = EdgeIndex::of(self.edge_count());

//         if index.is_limit() {
//             return recoverable_result!(Self::EdgeError::limit(), value);
//         }

//         Ok(index)
//     }
// }

// impl<N, E, K: Kind, I: UntypedIndex, T: Type, L: Loop> DataRef for Generic<N, E, K, I, T, L> {
//     fn node_value(&self, id: Self::NodeId) -> Option<&Self::NodeValue> {
//         self.node(id).map(|node| node.get())
//     }

//     fn edge_value(&self, id: Self::EdgeId) -> Option<&Self::EdgeValue> {
//         self.edge(id).map(|edge| edge.get())
//     }
// }

// impl<N, E, K: Kind, I: UntypedIndex, T: Type, L: Loop> DataMut for Generic<N, E, K, I, T, L> {
//     fn node_value_mut(&mut self, id: Self::NodeId) -> Option<&mut Self::NodeValue> {
//         self.node_mut(id).map(|node| node.get_mut())
//     }

//     fn edge_value_mut(&mut self, id: Self::EdgeId) -> Option<&mut Self::EdgeValue> {
//         self.edge_mut(id).map(|edge| edge.get_mut())
//     }
// }

// impl<N, E, K: Kind, I: UntypedIndex, T: Type, L: Loop> Default for Generic<N, E, K, I, T, L> {
//     fn default() -> Self {
//         Self::new()
//     }
// }

// impl<N, E, K: Kind, I: UntypedIndex, T: Type, L: Loop> Generic<N, E, K, I, T, L> {
//     fn node(&self, index: NodeIndex<I>) -> Option<&Node<N, I>> {
//         self.nodes.get(index.index())
//     }

//     fn node_mut(&mut self, index: NodeIndex<I>) -> Option<&mut Node<N, I>> {
//         self.nodes.get_mut(index.index())
//     }

//     fn edge(&self, index: EdgeIndex<I>) -> Option<&Edge<E, I>> {
//         self.edges.get(index.index())
//     }

//     fn edge_mut(&mut self, index: EdgeIndex<I>) -> Option<&mut Edge<E, I>> {
//         self.edges.get_mut(index.index())
//     }
// }

// impl<N, E, K: Kind, I: UntypedIndex, T: Type, L: Loop> Clear for Generic<N, E, K, I, T, L> {
//     fn clear(&mut self) {
//         self.nodes.clear();
//         self.edges.clear();
//     }
// }

// impl<N, E, K: Kind, I: UntypedIndex, T: Type, L: Loop> ClearEdges for Generic<N, E, K, I, T, L> {
//     fn clear_edges(&mut self) {
//         self.nodes.iter_mut().for_each(|node| {
//             node.next.reset();
//         });

//         self.edges.clear();
//     }
// }

// impl<N, E, K: Kind, I: UntypedIndex, T: Type, L: Loop> Reverse for Graph<N, E, K, I, T, L> {
//     fn reverse(&mut self) {
//         self.nodes.iter_mut().for_each(|node| node.next.reverse());

//         self.edges.iter_mut().for_each(|edge| {
//             edge.connection.reverse();
//             edge.next.reverse();
//         });
//     }
// }

// impl<N, E, K: Kind, I: UntypedIndex, T: Type, L: Loop> Find for Graph<N, E, K, I, T, L> {
//     fn find(
//         &self,
//         connection: Connection<Self::NodeId>,
//     ) -> Result<Self::Iterator<'_>, Self::MissingError> {
//         todo!()
//     }
// }

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
