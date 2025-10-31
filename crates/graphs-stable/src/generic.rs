use graphs_core::{
    capacity::Capacities,
    count::Counts,
    keys::{DefaultUntypedIndex, EdgeIndex, NodeIndex, UntypedIndex},
    kinds::{DefaultKind, Kind},
    loops::{DefaultLoop, Loop},
    types::{DefaultType, Type},
};
use graphs_simple::generic::Generic;

pub(crate) struct Free<I: UntypedIndex = DefaultUntypedIndex> {
    pub(crate) node: NodeIndex<I>,
    pub(crate) edge: EdgeIndex<I>,
}

impl<I: UntypedIndex> Free<I> {
    pub(crate) const fn new(node: NodeIndex<I>, edge: EdgeIndex<I>) -> Self {
        Self { node, edge }
    }

    pub(crate) const fn limit() -> Self {
        Self::new(NodeIndex::limit(), EdgeIndex::limit())
    }

    pub(crate) const fn reset(&mut self) {
        self.node = NodeIndex::limit();
        self.edge = EdgeIndex::limit();
    }
}

impl<I: UntypedIndex> Default for Free<I> {
    fn default() -> Self {
        Self::limit()
    }
}

pub(crate) struct Info<I: UntypedIndex = DefaultUntypedIndex> {
    pub(crate) count: Counts,
    pub(crate) free: Free<I>,
}

impl<I: UntypedIndex> Info<I> {
    pub(crate) const fn new(count: Counts, free: Free<I>) -> Self {
        Self { count, free }
    }

    pub(crate) const fn initial() -> Self {
        Self::new(Counts::null(), Free::limit())
    }

    pub(crate) const fn reset(&mut self) {
        self.count.reset();
        self.free.reset();
    }
}

pub struct StableGeneric<
    N,
    E,
    K: Kind = DefaultKind,
    I: UntypedIndex = DefaultUntypedIndex,
    T: Type = DefaultType,
    L: Loop = DefaultLoop,
> {
    pub(crate) graph: Generic<Option<N>, Option<E>, K, I, T, L>,
    pub(crate) info: Info<I>,
}

impl<N, E, K: Kind, I: UntypedIndex, T: Type, L: Loop> Default for StableGeneric<N, E, K, I, T, L> {
    fn default() -> Self {
        Self::new()
    }
}

impl<N, E, K: Kind, I: UntypedIndex, T: Type, L: Loop> StableGeneric<N, E, K, I, T, L> {
    pub const fn new() -> Self {
        Self {
            graph: Generic::new(),
            info: Info::initial(),
        }
    }

    pub const fn count(&self) -> Counts {
        self.info.count
    }

    pub const fn node_count(&self) -> usize {
        self.info.count.nodes
    }

    pub const fn edge_count(&self) -> usize {
        self.info.count.edges
    }

    pub const fn node_capacity(&self) -> usize {
        self.graph.node_capacity()
    }

    pub const fn edge_capacity(&self) -> usize {
        self.graph.edge_capacity()
    }

    pub const fn capacity(&self) -> Capacities {
        self.graph.capacity()
    }
}

impl<N, E, K: Kind, I: UntypedIndex, T: Type, L: Loop> StableGeneric<N, E, K, I, T, L> {
    pub fn with_capacity(capacities: Capacities) -> Self {
        todo!()
    }
}

impl<N, E, K: Kind, I: UntypedIndex, T: Type, L: Loop> Base for StableGeneric<N, E, K, I, T, L> {
    type NodeId = NodeIndex<I>;
    type EdgeId = EdgeIndex<I>;

    type Kind = K;
    type Type = T;
    type Loop = L;
}

impl<N, E, K: Kind, I: UntypedIndex, T: Type, L: Loop> Create for StableGeneric<N, E, K, I, T, L> {
    fn empty() -> Self {
        Self::new()
    }

    fn with_capacity(capacities: Capacities) -> Self {
        Self::with_capacity(capacities)
    }
}
