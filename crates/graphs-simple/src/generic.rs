#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

use core::{cmp::max, marker::PhantomData};

use graphs_common::index::{EdgeIndex, NodeIndex};
use graphs_core::{
    base::Base,
    build::{RecoverableNode, TryAddNodes},
    capacity::{Capacities, EdgeCapacity, NodeCapacity},
    cardinality::{Cardinality, Order, Size},
    clear::{Clear, ClearEdges},
    create::Create,
    data::{Data, DataMut, DataRef},
    identifiers::{EdgeIdentifiers, NodeIdentifiers},
    index::{DefaultUntypedIndex, Index, UntypedIndex},
    indexed::{EdgeIndexed, NodeIndexed},
    kinds::{DefaultKind, Kind},
    loops::{DefaultLoop, Loop},
    nodes::NodeIn,
    recoverable_return,
    references::{EdgeReferences, NodeReferences},
    specs::Specs,
    types::{DefaultType, Type},
};

use crate::{
    errors::{IndexError, node_index},
    indices::{EdgeIndices, NodeIndices},
    parts::{Connection, Edge, EdgeMutSlice, EdgeSlice, Node, NodeMutSlice, NodeSlice},
    references::{EdgeRef, EdgeRefIterator, NodeRef, NodeRefIterator},
};

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
    specs: PhantomData<Specs<K, T, L>>,
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
        self.node(id).map(|node| node.value())
    }

    fn edge_value(&self, id: Self::EdgeId) -> Option<&Self::EdgeValue> {
        self.edge(id).map(|edge| edge.value())
    }
}

impl<N, E, I: UntypedIndex, K: Kind, T: Type, L: Loop> DataMut for GenericGraph<N, E, I, K, T, L> {
    fn node_value_mut(&mut self, id: Self::NodeId) -> Option<&mut Self::NodeValue> {
        self.node_mut(id).map(|node| node.value_mut())
    }

    fn edge_value_mut(&mut self, id: Self::EdgeId) -> Option<&mut Self::EdgeValue> {
        self.edge_mut(id).map(|edge| edge.value_mut())
    }
}

impl<N, E, I: UntypedIndex, K: Kind, T: Type, L: Loop> Order for GenericGraph<N, E, I, K, T, L> {
    fn order(&self) -> usize {
        self.order()
    }
}

impl<N, E, I: UntypedIndex, K: Kind, T: Type, L: Loop> Size for GenericGraph<N, E, I, K, T, L> {
    fn size(&self) -> usize {
        self.size()
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
        self.order()
    }

    fn try_node_index(&self, id: Self::NodeId) -> Option<usize> {
        id.try_index()
    }

    fn node_index(&self, id: Self::NodeId) -> usize {
        id.index()
    }

    fn try_node_id(&self, index: usize) -> Option<Self::NodeId> {
        Self::NodeId::try_of(index)
    }

    fn node_id(&self, index: usize) -> Self::NodeId {
        Self::NodeId::of(index)
    }
}

impl<N, E, I: UntypedIndex, K: Kind, T: Type, L: Loop> EdgeIndexed
    for GenericGraph<N, E, I, K, T, L>
{
    fn edge_bound(&self) -> usize {
        self.size()
    }

    fn try_edge_index(&self, id: Self::EdgeId) -> Option<usize> {
        id.try_index()
    }

    fn edge_index(&self, id: Self::EdgeId) -> usize {
        id.index()
    }

    fn try_edge_id(&self, index: usize) -> Option<Self::EdgeId> {
        Self::EdgeId::try_of(index)
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
            specs: PhantomData,
        }
    }

    pub const fn order(&self) -> usize {
        self.nodes.len()
    }

    pub const fn size(&self) -> usize {
        self.edges.len()
    }

    pub const fn cardinality(&self) -> Cardinality {
        Cardinality::new(self.order(), self.size())
    }

    pub const fn is_null(&self) -> bool {
        self.cardinality().is_zero()
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

    pub(crate) const fn nodes_ref(&self) -> NodeSlice<'_, N, I> {
        self.nodes.as_slice()
    }

    pub(crate) const fn nodes_mut(&mut self) -> NodeMutSlice<'_, N, I> {
        self.nodes.as_mut_slice()
    }

    pub(crate) const fn edges_ref(&self) -> EdgeSlice<'_, E, I, K> {
        self.edges.as_slice()
    }

    pub(crate) const fn edges_mut(&mut self) -> EdgeMutSlice<'_, E, I, K> {
        self.edges.as_mut_slice()
    }
}

impl<N, E, I: UntypedIndex, K: Kind, T: Type, L: Loop> GenericGraph<N, E, I, K, T, L> {
    pub fn with_capacity(capacities: Capacities) -> Self {
        Self {
            nodes: Vec::with_capacity(capacities.nodes),
            edges: Vec::with_capacity(capacities.edges),
            specs: PhantomData,
        }
    }
}

impl<N, E, I: UntypedIndex, K: Kind, T: Type, L: Loop> GenericGraph<N, E, I, K, T, L> {
    pub const fn node_indices(&self) -> NodeIndices<I> {
        NodeIndices::new(self.order())
    }

    pub const fn edge_indices(&self) -> EdgeIndices<I> {
        EdgeIndices::new(self.size())
    }

    pub(crate) fn node(&self, id: NodeIndex<I>) -> Option<&Node<N, I>> {
        self.nodes.get(id.index())
    }

    pub(crate) fn node_mut(&mut self, id: NodeIndex<I>) -> Option<&mut Node<N, I>> {
        self.nodes.get_mut(id.index())
    }

    pub(crate) fn edge(&self, id: EdgeIndex<I>) -> Option<&Edge<E, I, K>> {
        self.edges.get(id.index())
    }

    pub(crate) fn edge_mut(&mut self, id: EdgeIndex<I>) -> Option<&mut Edge<E, I, K>> {
        self.edges.get_mut(id.index())
    }

    pub(crate) fn node_twice_mut(
        &mut self,
        one: NodeIndex<I>,
        two: NodeIndex<I>,
    ) -> Option<MaybePair<&mut Node<N, I>>> {
        get_twice_mut(self.nodes_mut(), one.index(), two.index())
    }

    pub(crate) unsafe fn node_pair_mut(
        &mut self,
        one: NodeIndex<I>,
        two: NodeIndex<I>,
    ) -> Pair<&mut Node<N, I>> {
        unsafe { get_pair_mut(self.nodes_mut(), one.index(), two.index()) }
    }
}

pub(crate) type Pair<T> = (T, T);

pub(crate) enum MaybePair<T> {
    Two(T, T),
    One(T),
}

pub(crate) fn get_twice_mut<T>(
    slice: &mut [T],
    one_index: usize,
    two_index: usize,
) -> Option<MaybePair<&mut T>> {
    let index = max(one_index, two_index);

    if index < slice.len() {
        if one_index == two_index {
            let value = unsafe { slice.get_unchecked_mut(index) };

            Some(MaybePair::One(value))
        } else {
            let (one, two) = unsafe { get_pair_mut(slice, one_index, two_index) };

            Some(MaybePair::Two(one, two))
        }
    } else {
        None
    }
}

pub(crate) unsafe fn get_pair_mut<T>(
    slice: &mut [T],
    one_index: usize,
    two_index: usize,
) -> Pair<&mut T> {
    let [one, two] = unsafe { slice.get_disjoint_unchecked_mut([one_index, two_index]) };

    (one, two)
}

impl<N, E, I: UntypedIndex, K: Kind, T: Type, L: Loop> NodeIdentifiers
    for GenericGraph<N, E, I, K, T, L>
{
    type NodeIdIterator<'g>
        = NodeIndices<I>
    where
        Self: 'g;

    fn node_identifiers(&self) -> Self::NodeIdIterator<'_> {
        self.node_indices()
    }
}

impl<N, E, I: UntypedIndex, K: Kind, T: Type, L: Loop> EdgeIdentifiers
    for GenericGraph<N, E, I, K, T, L>
{
    type EdgeIdIterator<'g>
        = EdgeIndices<I>
    where
        Self: 'g;

    fn edge_identifiers(&self) -> Self::EdgeIdIterator<'_> {
        self.edge_indices()
    }
}

impl<N, E, I: UntypedIndex, K: Kind, T: Type, L: Loop> NodeReferences
    for GenericGraph<N, E, I, K, T, L>
{
    type NodeRef<'g>
        = NodeRef<'g, N, I>
    where
        Self: 'g;

    type NodeRefIterator<'g>
        = NodeRefIterator<'g, N, I>
    where
        Self: 'g;

    fn node_references(&self) -> Self::NodeRefIterator<'_> {
        Self::NodeRefIterator::new(self.nodes.iter().enumerate())
    }
}

impl<N, E, I: UntypedIndex, K: Kind, T: Type, L: Loop> EdgeReferences
    for GenericGraph<N, E, I, K, T, L>
{
    type EdgeRef<'g>
        = EdgeRef<'g, E, I, K>
    where
        Self: 'g;

    type EdgeRefIterator<'g>
        = EdgeRefIterator<'g, E, I, K>
    where
        Self: 'g;

    fn edge_references(&self) -> Self::EdgeRefIterator<'_> {
        Self::EdgeRefIterator::new(self.edges.iter().enumerate())
    }
}

impl<N, E, I: UntypedIndex, K: Kind, T: Type, L: Loop> TryAddNodes
    for GenericGraph<N, E, I, K, T, L>
{
    type Error = IndexError<I>;

    fn try_add_node(&mut self, node: NodeIn<Self>) -> RecoverableNode<Self> {
        let value = node.get();

        let order = self.order();

        let id = recoverable_return!(node_index(order), value);

        let node = Node::new(value);

        self.nodes.push(node);

        Ok(id)
    }
}
