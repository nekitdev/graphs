use graphs_core::{
    base::Base,
    capacity::Capacities,
    count::Counts,
    create::Create,
    index::{DefaultUntypedIndex, EdgeIndex, NodeIndex, UntypedIndex},
    kinds::{DefaultKind, Kind},
    loops::{DefaultLoop, Loop},
    types::{DefaultType, Type},
};
use graphs_simple::generic::GenericGraph;

use crate::parts::{Connection, Info};

pub struct GenericStableGraph<
    N,
    E,
    I: UntypedIndex = DefaultUntypedIndex,
    K: Kind = DefaultKind,
    T: Type = DefaultType,
    L: Loop = DefaultLoop,
> {
    pub(crate) graph: GenericGraph<Option<N>, Option<E>, I, K, T, L>,
    pub(crate) info: Info<I>,
}

impl<N, E, I: UntypedIndex, K: Kind, T: Type, L: Loop> Default
    for GenericStableGraph<N, E, I, K, T, L>
{
    fn default() -> Self {
        Self::new()
    }
}

impl<N, E, I: UntypedIndex, K: Kind, T: Type, L: Loop> GenericStableGraph<N, E, I, K, T, L> {
    pub const fn new() -> Self {
        Self {
            graph: GenericGraph::new(),
            info: Info::INITIAL,
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

impl<N, E, I: UntypedIndex, K: Kind, T: Type, L: Loop> GenericStableGraph<N, E, I, K, T, L> {
    pub fn with_capacity(capacities: Capacities) -> Self {
        Self {
            graph: GenericGraph::with_capacity(capacities),
            info: Info::INITIAL,
        }
    }
}

impl<N, E, I: UntypedIndex, K: Kind, T: Type, L: Loop> Base
    for GenericStableGraph<N, E, I, K, T, L>
{
    type NodeId = NodeIndex<I>;
    type EdgeId = EdgeIndex<I>;

    type Connection = Connection<I, K>;

    type Kind = K;
    type Type = T;
    type Loop = L;
}

impl<N, E, I: UntypedIndex, K: Kind, T: Type, L: Loop> Create
    for GenericStableGraph<N, E, I, K, T, L>
{
    fn empty() -> Self {
        Self::new()
    }

    fn with_capacity(capacities: Capacities) -> Self {
        Self::with_capacity(capacities)
    }
}
