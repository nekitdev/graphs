use trait_aliases::trait_aliases;

use crate::{
    data::Data,
    id::{EdgeTypeId, NodeTypeId},
};

trait_aliases! {
    /// Represents the associated output type in [`Map`] and [`MapOwned`].
    pub trait Output<N, E, G: Data + ?Sized> = Data<
            // all associated types are preserved...
            NodeId = G::NodeId,
            EdgeId = G::EdgeId,
            Connection = G::Connection,
            Kind = G::Kind,
            Type = G::Type,
            Loop = G::Loop,
            // ... except for node and edge values
            NodeValue = N,
            EdgeValue = E,
        >;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Identity;

impl<I: NodeTypeId, N> NodeMap<I, N> for Identity {
    type Output = N;

    fn map_node(&mut self, _id: I, value: N) -> Self::Output {
        value
    }
}

impl<I: EdgeTypeId, E> EdgeMap<I, E> for Identity {
    type Output = E;

    fn map_edge(&mut self, _id: I, value: E) -> Self::Output {
        value
    }
}

pub trait NodeMap<I: NodeTypeId, N> {
    type Output;

    fn map_node(&mut self, id: I, value: N) -> Self::Output;
}

pub trait EdgeMap<I: EdgeTypeId, E> {
    type Output;

    fn map_edge(&mut self, id: I, value: E) -> Self::Output;
}

trait_aliases! {
    #[trait_alias(M)]
    pub trait NodeMapIn<G: Data + ?Sized> = NodeMap<G::NodeId, G::NodeValue>;

    #[trait_alias(M)]
    pub trait EdgeMapIn<G: Data + ?Sized> = EdgeMap<G::EdgeId, G::EdgeValue>;
}

impl<I: NodeTypeId, N, T, F: FnMut(I, N) -> T> NodeMap<I, N> for F {
    type Output = T;

    fn map_node(&mut self, id: I, value: N) -> Self::Output {
        self(id, value)
    }
}

impl<I: EdgeTypeId, E, T, F: FnMut(I, E) -> T> EdgeMap<I, E> for F {
    type Output = T;

    fn map_edge(&mut self, id: I, value: E) -> Self::Output {
        self(id, value)
    }
}

pub trait Map: Data {
    type Output<N, E>: Output<N, E, Self>;

    fn map<N: NodeMapIn<Self>, E: EdgeMapIn<Self>>(
        self,
        node_map: N,
        edge_map: E,
    ) -> Self::Output<N::Output, E::Output>;

    fn map_nodes<N: NodeMapIn<Self>>(self, node_map: N) -> Self::Output<N::Output, Self::EdgeValue>
    where
        Self: Sized,
    {
        self.map(node_map, Identity)
    }

    fn map_edges<E: EdgeMapIn<Self>>(self, edge_map: E) -> Self::Output<Self::NodeValue, E::Output>
    where
        Self: Sized,
    {
        self.map(Identity, edge_map)
    }
}
