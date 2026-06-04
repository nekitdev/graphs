use thiserror::Error;

use graphs_common::{
    dfs::{events::SimpleEvent, simple},
    err_or::ErrOr,
};
use graphs_core::{
    algorithms::Algorithm, base::Base, by::By, connections::Connection,
    identifiers::NodeIdentifiers, neighbors::Neighbors, visit::Visit,
};
use trait_aliases::trait_aliases;

trait_aliases! {
    #[trait_alias(G)]
    pub trait Input = Visit + Neighbors + NodeIdentifiers;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Error)]
#[error("cycle caused by connection {connection}")]
pub struct Cycle<C: Connection> {
    pub connection: C,
}

impl<C: Connection> Cycle<C> {
    /// Constructs [`Self`].
    pub const fn new(connection: C) -> Self {
        Self { connection }
    }

    /// Returns the connection that caused the cycle.
    pub const fn get(self) -> C {
        self.connection
    }
}

pub type CycleIn<G> = Cycle<<G as Base>::Connection>;

pub type Check<G> = Result<(), CycleIn<G>>;

pub type Output<G> = Option<CycleIn<G>>;

pub fn find<G: Input>(graph: G) -> Output<G> {
    simple::dfs(graph.by_ref(), graph.node_identifiers(), |event| {
        if let SimpleEvent::Back(connection) = event {
            Some(Cycle::new(connection))
        } else {
            None
        }
    })
}

pub fn check<G: Input>(graph: G) -> Check<G> {
    find(graph).err_or(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Cycles;

impl<G: Input> Algorithm<G> for Cycles {
    type Output = Output<G>;

    fn perform(&mut self, graph: G) -> Self::Output {
        find(graph)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct CheckAcyclic;

impl<G: Input> Algorithm<G> for CheckAcyclic {
    type Output = Check<G>;

    fn perform(&mut self, graph: G) -> Self::Output {
        check(graph)
    }
}
