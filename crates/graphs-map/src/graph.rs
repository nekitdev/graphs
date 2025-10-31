use core::{hash::BuildHasher, marker::PhantomData};

use std::convert::Infallible;

#[cfg(feature = "std")]
use std::hash::RandomState;

use graphs_core::{
    base::Base,
    build::{Build, Output},
    canonical::Canonical,
    connection::Connection,
    data::Data,
    direction::Directional,
    id::NodeTypeId,
    kinds::{Directed, Kind, Undirected},
    loops::{Forbid, Loop},
    recoverable::RecoverableResult,
    specs::Specs,
    types::Single,
};
use indexmap::IndexMap;

pub type Directionals<N> = Vec<Directional<N>>;
pub type Nodes<N, S> = IndexMap<N, Directionals<N>, S>;
pub type Edges<N, E, K, S> = IndexMap<Canonical<K, N>, E, S>;

pub struct GraphMap<
    N: NodeTypeId,
    E, // unrestricted
    K: Kind,
    #[cfg(feature = "std")] S: BuildHasher = RandomState,
    #[cfg(not(feature = "std"))] S: BuildHasher,
    L: Loop = Forbid,
> {
    nodes: Nodes<N, S>,
    edges: Edges<N, E, K, S>,
    specs: PhantomData<Specs<K, Single, L>>,
}

cfg_if! {
    if #[cfg(feature = "std")] {
        pub type DiGraphMap<N, E, S = RandomState, L = Forbid> = GraphMap<N, E, Directed, S, L>;
        pub type UnGraphMap<N, E, S = RandomState, L = Forbid> = GraphMap<N, E, Undirected, S, L>;
    } else {
        pub type DiGraphMap<N, E, S, L = Forbid> = GraphMap<N, E, Directed, S, L>;
        pub type UnGraphMap<N, E, S, L = Forbid> = GraphMap<N, E, Undirected, S, L>;
    }
}

impl<N: NodeTypeId, E, K: Kind, S: BuildHasher, L: Loop> Base for GraphMap<N, E, K, S, L> {
    type NodeId = N;
    type EdgeId = Canonical<K, N>;

    type Kind = K;
    type Loop = L;

    type Type = Single;
}

impl<N: NodeTypeId, E, K: Kind, S: BuildHasher> Data for GraphMap<N, E, K, S> {
    type NodeValue = N;
    type EdgeValue = E;
}

impl<N: NodeTypeId, E, K: Kind, S: BuildHasher> GraphMap<N, E, K, S> {
    fn create(&mut self, node: N) -> &mut Directionals<N> {
        self.nodes.entry(node).or_default() // `or_default` creates empty `Directionals`
    }
}

impl<N: NodeTypeId, E, K: Kind, S: BuildHasher> Build for GraphMap<N, E, K, S> {
    type NodeError = Infallible;
    type EdgeError = Infallible;

    fn add_node(
        &mut self,
        value: Self::NodeValue,
    ) -> RecoverableResult<Self::NodeId, Self::NodeError, Self::NodeValue> {
        self.create(value);

        Ok(value)
    }

    fn add_edge(
        &mut self,
        connection: Connection<Self::NodeId>,
        value: Self::EdgeValue,
    ) -> Output<Self::EdgeValue, Self::EdgeId> {
        let canonical = Canonical::new(connection);

        let previous = self.edges.insert(canonical, value);

        let (source, target) = canonical.parts();

        self.create(source).push(Directional::outgoing(target));

        // self-loops are only outgoing

        if target != source {
            self.create(target).push(Directional::incoming(source));
        }

        Output::new(canonical, previous)
    }
}
