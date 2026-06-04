use trait_aliases::trait_aliases;

use crate::{base::Base, capacity::Capacities};

pub trait ReserveNodes: Base {
    fn reserve_nodes(&mut self, additional: usize);

    fn reserve_nodes_exact(&mut self, additional: usize);
}

pub trait ReserveEdges: Base {
    fn reserve_edges(&mut self, additional: usize);

    fn reserve_edges_exact(&mut self, additional: usize);
}

trait_aliases! {
    #[trait_alias(G)]
    pub trait Reserve = ReserveNodes + ReserveEdges;
}

pub trait ReserveMethods: Reserve {
    fn reserve(&mut self, additional: Capacities) {
        self.reserve_nodes(additional.nodes);
        self.reserve_edges(additional.edges);
    }

    fn reserve_exact(&mut self, additional: Capacities) {
        self.reserve_nodes_exact(additional.nodes);
        self.reserve_edges_exact(additional.edges);
    }
}

impl<G: Reserve + ?Sized> ReserveMethods for G {}
