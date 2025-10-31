use crate::{
    base::{Base, Directed},
    connections::Connection,
    kinds::Kind,
};

pub struct Undirected<G: Directed> {
    graph: G,
}

impl<G: Directed> Base for Undirected<G> {
    type NodeId = G::NodeId;
    type EdgeId = G::EdgeId;

    type Connection = <G::Connection as Connection>::Inverse;

    type Kind = <G::Kind as Kind>::Inverse;
    type Type = G::Type;
    type Loop = G::Loop;
}

impl<G: Directed> Undirected<G> {
    pub const fn new(graph: G) -> Self {
        Self { graph }
    }

    pub const fn get_ref(&self) -> &G {
        &self.graph
    }

    pub const fn get_mut(&mut self) -> &mut G {
        &mut self.graph
    }

    pub fn get(self) -> G {
        self.graph
    }
}
