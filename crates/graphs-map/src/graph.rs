use core::{hash::BuildHasher, marker::PhantomData};

use std::convert::Infallible;

#[cfg(feature = "std")]
use std::hash::RandomState;

use cfg_if::cfg_if;
use graphs_core::{
    base::Base,
    build::{Build, Output},
    connection::{Canonical, Connection},
    data::Data,
    direction::WithDirection,
    id::NodeTypeId,
    kind::{Directed, Kind, Kinded, Undirected},
};
use indexmap::IndexMap;

pub type WithDirections<N> = Vec<WithDirection<N>>;
pub type Nodes<N, S> = IndexMap<N, WithDirections<N>, S>;
pub type Edges<N, E, S> = IndexMap<Canonical<N>, E, S>;

pub struct GraphMap<
    N: NodeTypeId,
    E, // unrestricted
    K: Kind,
    #[cfg(feature = "std")] S: BuildHasher = RandomState,
    #[cfg(not(feature = "std"))] S: BuildHasher,
> {
    nodes: Nodes<N, S>,
    edges: Edges<N, E, S>,
    kind: PhantomData<K>,
}

cfg_if! {
    if #[cfg(feature = "std")] {
        pub type DiGraphMap<N, E, S = RandomState> = GraphMap<N, E, Directed, S>;
        pub type UnGraphMap<N, E, S = RandomState> = GraphMap<N, E, Undirected, S>;
    } else {
        pub type DiGraphMap<N, E, S> = GraphMap<N, E, Directed, S>;
        pub type UnGraphMap<N, E, S> = GraphMap<N, E, Undirected, S>;
    }
}

impl<N: NodeTypeId, E, K: Kind, S: BuildHasher> Kinded for GraphMap<N, E, K, S> {
    type Kind = K;
}

impl<N: NodeTypeId, E, K: Kind, S: BuildHasher> Base for GraphMap<N, E, K, S> {
    type NodeId = N;
    type EdgeId = Canonical<N>;
}

impl<N: NodeTypeId, E, K: Kind, S: BuildHasher> Data for GraphMap<N, E, K, S> {
    type NodeValue = N;
    type EdgeValue = E;
}

impl<N: NodeTypeId, E, K: Kind, S: BuildHasher> GraphMap<N, E, K, S> {
    pub fn canonicalize(connection: Connection<N>) -> Canonical<N> {
        connection.canonical::<K>()
    }
}

impl<N: NodeTypeId, E, K: Kind, S: BuildHasher> Build for GraphMap<N, E, K, S> {
    type AddNodeError = Infallible;
    type AddEdgeError = Infallible;

    fn try_add_node(&mut self, value: Self::NodeValue) -> Result<Self::NodeId, Self::AddNodeError> {
        Ok(self.add_node(value))
    }

    fn add_node(&mut self, value: Self::NodeValue) -> Self::NodeId {
        // NOTE: `or_default` here creates empty `WithDirections`

        self.nodes.entry(value).or_default();

        value
    }

    fn try_add_edge_with(
        &mut self,
        connection: Connection<Self::NodeId>,
        value: Self::EdgeValue,
    ) -> Result<Output<Self::EdgeValue, Self::EdgeId>, Self::AddEdgeError> {
        Ok(self.add_edge_with(connection, value))
    }

    fn add_edge_with(
        &mut self,
        connection: Connection<Self::NodeId>,
        value: Self::EdgeValue,
    ) -> Output<Self::EdgeValue, Self::EdgeId> {
        let canonical = Self::canonicalize(connection);

        let (source, target) = canonical.get();

        let previous = self.edges.insert(canonical, value);

        self.nodes
            .entry(source)
            .or_default() // `or_default` creates empty `WithDirections`
            .push(WithDirection::outgoing(target));

        // self-loops are only outgoing

        if target != source {
            self.nodes
                .entry(target)
                .or_default() // `or_default` creates empty `WithDirections`
                .push(WithDirection::incoming(source))
        }

        Output::new(canonical, previous)
    }
}
