use core::ops::Deref;

use trait_aliases::trait_aliases;

use graphs_algorithms::cycles::{CycleIn, Cycles};
use graphs_core::{
    algorithms::Apply,
    base::{Base, Directed, ForbidLoop},
    by::By,
    identifiers::NodeIdentifiers,
    neighbors::Neighbors,
    visit::Visit,
};

trait_aliases! {
    #[trait_alias(G)]
    pub trait Check = Visit + Neighbors + NodeIdentifiers;

    #[trait_alias(G)]
    pub trait Input = Directed + ForbidLoop + Check;
}

pub struct Acyclic<G: Input> {
    graph: G,
}

impl<G: Input> Base for Acyclic<G> {
    type NodeId = G::NodeId;
    type EdgeId = G::EdgeId;

    type Connection = G::Connection;

    type Kind = G::Kind;
    type Type = G::Type;
    type Loop = G::Loop;
}

impl<G: Input> Acyclic<G> {
    pub const unsafe fn new_unchecked(graph: G) -> Self {
        Self { graph }
    }

    pub const fn get_ref(&self) -> &G {
        &self.graph
    }

    pub(crate) const fn get_mut(&mut self) -> &mut G {
        &mut self.graph
    }

    pub fn get(self) -> G {
        self.graph
    }
}

impl<G: Input> Deref for Acyclic<G> {
    type Target = G;

    fn deref(&self) -> &Self::Target {
        self.get_ref()
    }
}
