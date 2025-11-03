use core::marker::PhantomData;

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

use graphs_core::{
    base::Base,
    capacity::{Capacities, EdgeCapacity, NodeCapacity},
    clear::{Clear, ClearEdges},
    count::{Counts, EdgeCount, NodeCount},
    create::Create,
    data::{Data, DataMut, DataRef},
    index::{DefaultUntypedIndex, EdgeIndex, Index, NodeIndex, UntypedIndex},
    indexed::{EdgeIndexed, NodeIndexed},
    kinds::{DefaultKind, Kind},
    loops::{DefaultLoop, Loop},
    types::{DefaultType, Type},
};

use crate::parts::{Connection, Edge, Node};

/// Represents generic graphs.
pub struct GenericGraph<
    N,
    E,
    I: UntypedIndex = DefaultUntypedIndex,
    K: Kind = DefaultKind,
    T: Type = DefaultType,
    L: Loop = DefaultLoop,
> {
    pub(crate) nodes: Vec<Node<N, I>>,
    pub(crate) edges: Vec<Edge<E, I, K>>,
    kind_marker: PhantomData<K>,
    type_marker: PhantomData<T>,
    loop_marker: PhantomData<L>,
}

impl<N, E, I: UntypedIndex, K: Kind, T: Type, L: Loop> Default for GenericGraph<N, E, I, K, T, L> {
    fn default() -> Self {
        Self::new()
    }
}

impl<N, E, I: UntypedIndex, K: Kind, T: Type, L: Loop> Create for GenericGraph<N, E, I, K, T, L> {
    fn empty() -> Self {
        Self::new()
    }

    fn with_capacity(capacities: Capacities) -> Self {
        Self::with_capacity(capacities)
    }
}

impl<N, E, I: UntypedIndex, K: Kind, T: Type, L: Loop> Clear for GenericGraph<N, E, I, K, T, L> {
    fn clear(&mut self) {
        self.nodes.clear();
        self.edges.clear();
    }
}

impl<N, E, I: UntypedIndex, K: Kind, T: Type, L: Loop> ClearEdges
    for GenericGraph<N, E, I, K, T, L>
{
    fn clear_edges(&mut self) {
        self.nodes.iter_mut().for_each(|node| node.reset());

        self.edges.clear();
    }
}

impl<N, E, I: UntypedIndex, K: Kind, T: Type, L: Loop> Base for GenericGraph<N, E, I, K, T, L> {
    type NodeId = NodeIndex<I>;
    type EdgeId = EdgeIndex<I>;

    type Connection = Connection<I, K>;

    type Kind = K;
    type Type = T;
    type Loop = L;
}

impl<N, E, I: UntypedIndex, K: Kind, T: Type, L: Loop> Data for GenericGraph<N, E, I, K, T, L> {
    type NodeValue = N;
    type EdgeValue = E;
}

impl<N, E, I: UntypedIndex, K: Kind, T: Type, L: Loop> DataRef for GenericGraph<N, E, I, K, T, L> {
    fn node_value(&self, id: Self::NodeId) -> Option<&Self::NodeValue> {
        self.node(id).map(|node| &node.value)
    }

    fn edge_value(&self, id: Self::EdgeId) -> Option<&Self::EdgeValue> {
        self.edge(id).map(|edge| &edge.value)
    }
}

impl<N, E, I: UntypedIndex, K: Kind, T: Type, L: Loop> DataMut for GenericGraph<N, E, I, K, T, L> {
    fn node_value_mut(&mut self, id: Self::NodeId) -> Option<&mut Self::NodeValue> {
        self.node_mut(id).map(|node| &mut node.value)
    }

    fn edge_value_mut(&mut self, id: Self::EdgeId) -> Option<&mut Self::EdgeValue> {
        self.edge_mut(id).map(|edge| &mut edge.value)
    }
}

impl<N, E, I: UntypedIndex, K: Kind, T: Type, L: Loop> NodeCount
    for GenericGraph<N, E, I, K, T, L>
{
    fn node_count(&self) -> usize {
        self.node_count()
    }
}

impl<N, E, I: UntypedIndex, K: Kind, T: Type, L: Loop> EdgeCount
    for GenericGraph<N, E, I, K, T, L>
{
    fn edge_count(&self) -> usize {
        self.edge_count()
    }
}

impl<N, E, I: UntypedIndex, K: Kind, T: Type, L: Loop> NodeCapacity
    for GenericGraph<N, E, I, K, T, L>
{
    fn node_capacity(&self) -> usize {
        self.node_capacity()
    }
}

impl<N, E, I: UntypedIndex, K: Kind, T: Type, L: Loop> EdgeCapacity
    for GenericGraph<N, E, I, K, T, L>
{
    fn edge_capacity(&self) -> usize {
        self.edge_capacity()
    }
}

impl<N, E, I: UntypedIndex, K: Kind, T: Type, L: Loop> NodeIndexed
    for GenericGraph<N, E, I, K, T, L>
{
    fn node_bound(&self) -> usize {
        self.node_count()
    }

    fn node_index(&self, id: Self::NodeId) -> usize {
        id.index()
    }

    fn node_id(&self, index: usize) -> Self::NodeId {
        Self::NodeId::of(index)
    }
}

impl<N, E, I: UntypedIndex, K: Kind, T: Type, L: Loop> EdgeIndexed
    for GenericGraph<N, E, I, K, T, L>
{
    fn edge_bound(&self) -> usize {
        self.edge_count()
    }

    fn edge_index(&self, id: Self::EdgeId) -> usize {
        id.index()
    }

    fn edge_id(&self, index: usize) -> Self::EdgeId {
        Self::EdgeId::of(index)
    }
}

impl<N, E, I: UntypedIndex, K: Kind, T: Type, L: Loop> GenericGraph<N, E, I, K, T, L> {
    pub const fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            kind_marker: PhantomData,
            type_marker: PhantomData,
            loop_marker: PhantomData,
        }
    }

    pub const fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub const fn edge_count(&self) -> usize {
        self.edges.len()
    }

    pub const fn count(&self) -> Counts {
        Counts::new(self.node_count(), self.edge_count())
    }

    pub const fn is_null(&self) -> bool {
        self.count().is_null()
    }

    pub const fn node_capacity(&self) -> usize {
        self.nodes.capacity()
    }

    pub const fn edge_capacity(&self) -> usize {
        self.edges.capacity()
    }

    pub const fn capacity(&self) -> Capacities {
        Capacities::new(self.node_capacity(), self.edge_capacity())
    }
}

impl<N, E, I: UntypedIndex, K: Kind, T: Type, L: Loop> GenericGraph<N, E, I, K, T, L> {
    pub fn with_capacity(capacities: Capacities) -> Self {
        Self {
            nodes: Vec::with_capacity(capacities.nodes),
            edges: Vec::with_capacity(capacities.edges),
            kind_marker: PhantomData,
            type_marker: PhantomData,
            loop_marker: PhantomData,
        }
    }
}

impl<N, E, I: UntypedIndex, K: Kind, T: Type, L: Loop> GenericGraph<N, E, I, K, T, L> {
    pub(crate) fn node(&self, index: NodeIndex<I>) -> Option<&Node<N, I>> {
        self.nodes.get(index.index())
    }

    pub(crate) fn edge(&self, index: EdgeIndex<I>) -> Option<&Edge<E, I, K>> {
        self.edges.get(index.index())
    }

    pub(crate) fn node_mut(&mut self, index: NodeIndex<I>) -> Option<&mut Node<N, I>> {
        self.nodes.get_mut(index.index())
    }

    pub(crate) fn edge_mut(&mut self, index: EdgeIndex<I>) -> Option<&mut Edge<E, I, K>> {
        self.edges.get_mut(index.index())
    }
}
