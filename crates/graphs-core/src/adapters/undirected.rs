use core::iter::Chain;

use crate::{
    base::Base,
    direction::{Incoming, Outgoing},
    kind::{self, Kinded},
    neighbors::{DirectedNeighbors, Neighbors},
};

pub struct Undirected<G> {
    graph: G,
}

impl<G> Undirected<G> {
    pub const fn new(graph: G) -> Self {
        Self { graph }
    }

    pub const fn get(&self) -> &G {
        &self.graph
    }

    pub const fn get_mut(&mut self) -> &mut G {
        &mut self.graph
    }

    pub fn take(self) -> G {
        self.graph
    }
}

impl<G: Base> Base for Undirected<G> {
    type NodeId = G::NodeId;
    type EdgeId = G::EdgeId;
}

impl<G> Kinded for Undirected<G> {
    type Kind = kind::Undirected;
}

impl<G: DirectedNeighbors> Neighbors for Undirected<G> {
    type Iterator<'n>
        = Chain<G::DirectedIterator<'n>, G::DirectedIterator<'n>>
    where
        Self: 'n;

    fn neighbors(&self, node: Self::NodeId) -> Self::Iterator<'_> {
        self.graph
            .directed_neighbors(node, Incoming)
            .chain(self.graph.directed_neighbors(node, Outgoing))
    }
}
