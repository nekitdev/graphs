use graphs_core::{
    base::Base,
    capacity::Capacities,
    clear::{Clear, ClearEdges},
    count::Counts,
    create::Create,
    data::Data,
    index::{DefaultUntypedIndex, EdgeIndex, NodeIndex, UntypedIndex},
    kinds::{DefaultKind, Directed, Kind},
    loops::{DefaultLoop, Loop},
    reverse::Reverse,
    types::{DefaultType, Type},
};

use crate::graph::{GenericGraph, Graph};

struct Info<I: UntypedIndex = DefaultUntypedIndex> {
    node_count: usize,
    edge_count: usize,
    free_node: NodeIndex<I>,
    free_edge: EdgeIndex<I>,
}

impl<I: UntypedIndex> Info<I> {
    const fn new() -> Self {
        Self {
            node_count: 0,
            edge_count: 0,
            free_node: NodeIndex::limit(),
            free_edge: EdgeIndex::limit(),
        }
    }

    const fn reset_nodes(&mut self) {
        self.node_count = 0;
        self.free_node = NodeIndex::limit();
    }

    const fn reset_edges(&mut self) {
        self.edge_count = 0;
        self.free_edge = EdgeIndex::limit();
    }

    const fn reset(&mut self) {
        self.reset_nodes();
        self.reset_edges();
    }
}

pub struct StableGraph<
    N,
    E,
    K: Kind = DefaultKind,
    I: UntypedIndex = DefaultUntypedIndex,
    T: Type = DefaultType,
    L: Loop = DefaultLoop,
> {
    graph: GenericGraph<Option<N>, Option<E>, K, I, T, L>,
    info: Info<I>,
}

pub type StableDiGraph<N, E, I = DefaultUntypedIndex, T = DefaultType, L = DefaultLoop> =
    StableGraph<N, E, Directed, I, T, L>;

pub type StableUnGraph<N, E, I = DefaultUntypedIndex, T = DefaultType, L = DefaultLoop> =
    StableGraph<N, E, DefaultKind, I, T, L>;

impl<N, E, K: Kind, I: UntypedIndex, T: Type, L: Loop> StableGraph<N, E, K, I, T, L> {
    pub const fn new() -> Self {
        Self {
            graph: Graph::new(),
            info: Info::new(),
        }
    }

    pub const fn node_count(&self) -> usize {
        self.info.node_count
    }

    pub const fn edge_count(&self) -> usize {
        self.info.edge_count
    }

    pub const fn count(&self) -> Counts {
        Counts::new(self.node_count(), self.edge_count())
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

    pub const fn is_directed(&self) -> bool {
        self.graph.is_directed()
    }

    pub const fn is_single(&self) -> bool {
        self.graph.is_single()
    }

    pub const fn is_forbid(&self) -> bool {
        self.graph.is_forbid()
    }
}

impl<N, E, K: Kind, I: UntypedIndex, T: Type, L: Loop> Base for StableGraph<N, E, K, I, T, L> {
    type NodeId = NodeIndex<I>;
    type EdgeId = EdgeIndex<I>;

    type Kind = K;
    type Type = T;
    type Loop = L;
}

impl<N, E, K: Kind, I: UntypedIndex, T: Type, L: Loop> Data for StableGraph<N, E, K, I, T, L> {
    type NodeValue = N;
    type EdgeValue = E;
}

impl<N, E, K: Kind, I: UntypedIndex, T: Type, L: Loop> Create for StableGraph<N, E, K, I, T, L> {
    fn empty() -> Self {
        Self::new()
    }

    fn with_capacity(capacities: Capacities) -> Self {
        Self {
            graph: Graph::with_capacity(capacities),
            info: Info::new(),
        }
    }
}

impl<N, E, I: UntypedIndex, T: Type, L: Loop> StableDiGraph<N, E, I, T, L> {
    pub const fn directed() -> Self {
        Self::new()
    }
}

impl<N, E, I: UntypedIndex, T: Type, L: Loop> StableUnGraph<N, E, I, T, L> {
    pub const fn undirected() -> Self {
        Self::new()
    }
}

impl<N, E, K: Kind, I: UntypedIndex, T: Type, L: Loop> Default for StableGraph<N, E, K, I, T, L> {
    fn default() -> Self {
        Self::new()
    }
}

impl<N, E, K: Kind, I: UntypedIndex, T: Type, L: Loop> Clear for StableGraph<N, E, K, I, T, L> {
    fn clear(&mut self) {
        self.graph.clear();
        self.info.reset();
    }
}

impl<N, E, K: Kind, I: UntypedIndex, T: Type, L: Loop> ClearEdges
    for StableGraph<N, E, K, I, T, L>
{
    fn clear_edges(&mut self) {
        self.graph.clear_edges();
        self.info.reset_edges();
    }
}

impl<N, E, K: Kind, I: UntypedIndex, T: Type, L: Loop> Reverse for StableGraph<N, E, K, I, T, L> {
    fn reverse(&mut self) {
        self.graph.reverse();
    }
}
